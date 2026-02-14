# Mastodon Unfollower Monitor 🦀📧

![Rust](https://img.shields.io/badge/language-Rust-red?style=for-the-badge&logo=rust)
![License](https://img.shields.io/badge/license-MIT-blue?style=for-the-badge)

A specialized CLI utility designed to track follower fluctuations on Mastodon instances. It compares the current follower state with a local cache and dispatches automated email alerts when an unfollow event is detected. Optimized for deployment on low-resource hardware like the **Raspberry Pi Zero**.

## 🛠 Technical Implementation

The application is engineered as a state-aware monitoring tool with the following architectural features:

- **State Persistence:** Utilizes a local JSON-based storage (`followers.json`) to track follower IDs between executions, minimizing redundant API traffic.
- **RFC 5988 Compliance:** Implements robust parsing of 'Link' headers to handle Mastodon API pagination, ensuring full coverage of large follower lists.
- **Handle Normalization:** Automatically resolves local vs. remote handles by standardizing account identifiers (e.g., ensuring the `@domain` suffix is present for consistency during diffing).
- **Static Linking Capability:** Configured with `rustls-tls` to ensure the resulting binary is portable and free of system-level OpenSSL dependencies, simplifying deployment on diverse Linux distributions.



## 🚀 Installation & Setup

### 1. Prerequisites
- **Rust Toolchain:** Stable (2021 edition).
- **Mastodon Access Token:** Requires `read:accounts` scope.
- **SMTP Credentials:** A valid relay (e.g., Gmail, SendGrid, or self-hosted) with TLS support.

### 2. Configuration
Create a `.env` file in the project root:

```env
MASTODON_ACCESS_TOKEN=your_token
MASTODON_INSTANCE_URL=https://mastodon.social
SMTP_SERVER=smtp.example.com
SMTP_USER=user@example.com
SMTP_PASSWORD=secure_password
EMAIL_TO=your_email@example.com
```

### 3. Compilation
For local execution:
```bash
cargo build --release
```

For cross-compilation (e.g., Raspberry Pi Zero):
```bash
CROSS_CONTAINER_ENGINE=podman cross build --target arm-unknown-linux-gnueabihf --release
```

## ⚙️ Automation

This utility is designed to be executed as a periodic task (e.g., via `cron` or `systemd.timer`).

**Example Cron configuration (hourly check):**
```cron
0 * * * * cd /home/kepler/Mastodon/rust/unfollowers && ./target/release/unfollowers >> monitor.log 2>&1
```



## 🏗 Resource Optimization

The program follows a "Run-to-Completion" model. By avoiding a persistent daemon architecture, it significantly reduces the memory footprint and CPU cycles on edge devices. All HTTP operations are performed sequentially to prevent rate-limiting triggers on the instance side.

## 📜 License

MIT License.
