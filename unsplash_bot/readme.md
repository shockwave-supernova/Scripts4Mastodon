# Unsplash to Mastodon Bot 🦀

![Rust](https://img.shields.io/badge/made_with-Rust-red?style=for-the-badge&logo=rust)

A minimalist, high-performance bot written in Rust that fetches popular wallpapers from Unsplash and publishes them to your Mastodon instance.

**Key Feature: Zero Visual Noise.**
The bot posts *only* the image to the feed. All metadata—location, description, tags, author credits, and original links—are neatly packed into the **Alt Text**. This ensures your feed remains purely aesthetic while maintaining full accessibility and proper attribution.

## ✨ Features

* **Built with Rust:** Blazing fast, single binary, minimal memory footprint.
* **Smart Selection:** Fetches "Popular" landscape wallpapers from Unsplash.
* **No Duplicates:** Maintains a local `history.json` to ensure fresh content every time.
* **Clean Feed:** The status body is empty. No visible hashtags or links.
* **Rich Metadata (Alt Text):**
    * 📍 Location (City, Country, or Name)
    * 📝 Photo Description
    * 🏷 Tags (up to 15 relevant tags)
    * 📷 Author Credits & Link to Original

## 🛠 Installation & Usage

### Prerequisites
* [Rust & Cargo](https://www.rust-lang.org/tools/install) installed.
* Unsplash Developer Account (for Access Key).
* Mastodon Account & App Token (Settings -> Development).

### 1. Build

Clone the repository and build the release version:

```bash
# Navigate to the bot directory
cd unsplash_bot

# Build with optimizations
cargo build --release
```

The executable will be located at `target/release/unsplash_bot`.

### 2. Configuration

Create a `.env` file in the project root (next to `Cargo.toml`) and add your credentials:

```env
UNSPLASH_ACCESS_KEY=your_unsplash_access_key
MASTODON_ACCESS_TOKEN=your_mastodon_access_token
MASTODON_INSTANCE_URL=[https://mastodon.social](https://mastodon.social)
```

> **Note:** The `.env` file is included in `.gitignore` to prevent leaking secrets.

### 3. Run

```bash
./target/release/unsplash_bot
```

If a new photo is found, it will be uploaded, and its ID will be saved to `history.json`.

## 🤖 Automation (Cron)

To post a new wallpaper automatically (e.g., 3 times a day), add a cron job.

Open crontab:
```bash
crontab -e
```

Add the following line (adjust paths accordingly):
```cron
# Run at 9:00, 15:00, and 21:00 every day
0 9,15,21 * * * cd /path/to/unsplash_bot && ./target/release/unsplash_bot >> bot.log 2>&1
```

## 📂 Project Structure

* `src/main.rs`: Core logic.
* `history.json`: Automatically generated file tracking posted image IDs. Delete this file to reset the bot's memory.

## 📝 License

MIT License. Feel free to fork and modify.
