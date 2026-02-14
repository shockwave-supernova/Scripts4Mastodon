use anyhow::{Context, Result};
use dotenv::dotenv;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use regex::Regex;
use reqwest::blocking::Client;
use reqwest::header::LINK;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// Local database filename for persisting follower state between executions.
const FOLLOWERS_FILE: &str = "followers.json";

/// Application configuration loaded from environment variables.
struct Config {
    access_token: String,
    instance_url: String,
    smtp_server: String,
    smtp_user: String,
    smtp_password: String,
    email_to: String,
}

impl Config {
    /// Ingests configuration from the environment, returning an error if any required variable is missing.
    fn from_env() -> Result<Self> {
        Ok(Config {
            access_token: env::var("MASTODON_ACCESS_TOKEN").context("Missing MASTODON_ACCESS_TOKEN")?,
            instance_url: env::var("MASTODON_INSTANCE_URL").context("Missing MASTODON_INSTANCE_URL")?,
            smtp_server: env::var("SMTP_SERVER").context("Missing SMTP_SERVER")?,
            smtp_user: env::var("SMTP_USER").context("Missing SMTP_USER")?,
            smtp_password: env::var("SMTP_PASSWORD").context("Missing SMTP_PASSWORD")?,
            email_to: env::var("EMAIL_TO").context("Missing EMAIL_TO")?,
        })
    }
}

/// Represents a Mastodon account entity as returned by the API.
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Account {
    id: String,
    username: String,
    acct: String, // Fully qualified or local account handle
}

/// Response structure for the /api/v1/accounts/verify_credentials endpoint.
#[derive(Deserialize)]
struct VerifyCredentials {
    id: String,
}

/// Dispatches a plain-text email notification via an SMTP relay.
fn send_email(config: &Config, subject: &str, body: &str) -> Result<()> {
    let email = Message::builder()
        .from(config.smtp_user.parse()?)
        .to(config.email_to.parse()?)
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(body.to_string())?;

    let creds = Credentials::new(config.smtp_user.clone(), config.smtp_password.clone());

    // Configure SMTP transport with TLS support
    let mailer = SmtpTransport::relay(&config.smtp_server)?
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("[INFO] Email notification sent successfully."),
        Err(e) => eprintln!("[ERROR] SMTP failure: {:?}", e),
    }

    Ok(())
}

/// Retrieves all followers for the authenticated user, handling API pagination.
fn get_followers(client: &Client, config: &Config) -> Result<HashMap<String, String>> {
    // 1. Identify current user ID via credential verification
    let verify_url = format!("{}/api/v1/accounts/verify_credentials", config.instance_url);
    let user_data: VerifyCredentials = client
        .get(&verify_url)
        .header("Authorization", format!("Bearer {}", config.access_token))
        .send()
        .context("Failed to verify Mastodon credentials")?
        .json()?;

    let user_id = user_data.id;
    let mut followers_url = Some(format!("{}/api/v1/accounts/{}/followers", config.instance_url, user_id));
    let mut followers_map = HashMap::new();

    // Regex for parsing RFC 5988 Link headers used by Mastodon for pagination
    let re = Regex::new(r#"<([^>]+)>;\s*rel="next""#).unwrap();

    // Determine the local instance domain for handle normalization
    let local_domain = config.instance_url
        .replace("https://", "")
        .replace("http://", "")
        .trim_end_matches('/')
        .to_string();

    // 2. Iterate through paginated results

    while let Some(url) = followers_url {
        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", config.access_token))
            .send()
            .context("Failed to retrieve followers page")?;

        // Parse 'Link' header for the next page URL
        let next_url = if let Some(link_header) = response.headers().get(LINK) {
            let link_str = link_header.to_str().unwrap_or("");
            re.captures(link_str).map(|cap| cap[1].to_string())
        } else {
            None
        };

        let accounts: Vec<Account> = response.json().context("Failed to parse follower JSON payload")?;

        for acc in accounts {
            // Normalize handles: ensure local users have the @domain suffix for consistency
            let full_handle = if acc.acct.contains('@') {
                acc.acct
            } else {
                format!("{}@{}", acc.acct, local_domain)
            };

            followers_map.insert(acc.id, full_handle);
        }

        followers_url = next_url;
    }

    Ok(followers_map)
}

/// Loads the follower state from the local JSON storage.
fn load_previous_followers() -> HashMap<String, String> {
    if !Path::new(FOLLOWERS_FILE).exists() {
        return HashMap::new();
    }
    let file = File::open(FOLLOWERS_FILE).expect("Failed to open local state file");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap_or_default()
}

/// Persists the current follower state to local storage.
fn save_followers(followers: &HashMap<String, String>) -> Result<()> {
    let file = File::create(FOLLOWERS_FILE)?;
    serde_json::to_writer_pretty(file, followers)?;
    Ok(())
}

/// Orchestrates the unfollower check: loads state, fetches current data, diffs them, and notifies if necessary.
fn check_unfollowers() -> Result<()> {
    dotenv().ok();
    let config = Config::from_env().context("Configuration error")?;
    let client = Client::new();

    println!("[INFO] Loading cached follower state...");
    let old_followers = load_previous_followers();

    println!("[INFO] Fetching current followers from the API...");
    let new_followers = get_followers(&client, &config)?;

    // Identify users present in the old list but absent in the new list
    let mut unfollowers = Vec::new();
    for (uid, username) in &old_followers {
        if !new_followers.contains_key(uid) {
            unfollowers.push(username);
        }
    }

    if !unfollowers.is_empty() {
        // Iterate by reference (&unfollowers) to avoid moving the vector.
        // This ensures the collection remains accessible for subsequent telemetry or logging.
        let mut message = String::from("The following accounts have unfollowed you:\n\n");

        for username in &unfollowers {
            message.push_str(&format!("@{}\n", username));
        }

        // Now we can safely access unfollowers.len() because we only borrowed the data in the loop above.
        println!("[ALERT] Detected {} unfollowers.", unfollowers.len());
        send_email(&config, "Mastodon Unfollower Alert", &message)?;
    } else {
        println!("[INFO] No new unfollowers detected.");
    }

    // Update the local state with the latest follower list
    save_followers(&new_followers)?;
    Ok(())
}

fn main() {
    if let Err(e) = check_unfollowers() {
        eprintln!("[FATAL] Application error: {:#}", e);
    }
}
