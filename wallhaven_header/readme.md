# Wallhaven to Mastodon Header Sync 🦀

![Rust](https://img.shields.io/badge/language-Rust-red?style=for-the-badge&logo=rust)
![License](https://img.shields.io/badge/license-MIT-blue?style=for-the-badge)

A specialized Rust utility designed to automate the synchronization of high-resolution cyberpunk-themed imagery from Wallhaven to a Mastodon profile header.



## 📋 Overview

This tool queries the Wallhaven API for specific visual criteria (Cyberpunk City aesthetic, specific aspect ratios) and updates the Mastodon account's header image via the `PATCH /api/v1/accounts/update_credentials` endpoint. 

The binary is optimized for deployment on resource-constrained hardware, such as the **Raspberry Pi Zero**, by utilizing a pure-Rust TLS implementation to bypass complex system dependencies.

## ✨ Technical Features

- **Asynchronous Logic:** Built on the `reqwest` blocking client for reliable sequential execution in automation scripts.
- **Portability:** Uses `rustls-tls` instead of `native-tls` (OpenSSL), ensuring the binary is statically linked and portable across different Linux distributions and architectures without requiring local OpenSSL headers.
- **Dynamic Filtering:** - **Query:** `cyberpunk city`
    - **Dimensions:** Minimum 1500x500px (Mastodon standard).
    - **Ratios:** Filters for 16x9, 32x9, and 21x9 to ensure optimal framing on various devices.
    - **Randomization:** Uses API-level sorting to ensure a unique header on every execution.

## 🛠 Prerequisites

- **Rust Toolchain:** Stable (2021 edition or later).
- **Wallhaven API Key:** Required for authenticated search requests.
- **Mastodon Access Token:** Requires `write:accounts` scope to modify profile metadata.

## 🚀 Installation & Configuration

### 1. Clone and Configure
Clone the repository and create a `.env` file in the project root:

```env
WALLHAVEN_API_KEY=your_wallhaven_api_key
MASTODON_ACCESS_TOKEN=your_mastodon_access_token
MASTODON_INSTANCE_URL=[https://mastodon.social](https://mastodon.social)
```

### 2. Build for Local Architecture
```bash
cargo build --release
```

### 3. Cross-Compilation (e.g., Raspberry Pi Zero)
To build for ARMv6 architecture using `cross`:



```bash
CROSS_CONTAINER_ENGINE=podman cross build --target arm-unknown-linux-gnueabihf --release
```

## ⚙️ Automation

To rotate your profile header automatically, it is recommended to use a Cron job.

Example (Rotate every 24 hours at midnight):
```cron
0 0 * * * cd /path/to/project && ./target/release/wallhaven_header >> sync.log 2>&1
```

## 🏗 Architecture Note

The application is designed as a "Run-to-Completion" task rather than a persistent daemon. This minimizes memory overhead on low-tier VPS or edge devices. The history of downloaded images is managed by the Wallhaven random seed logic, ensuring low collision rates without local database overhead.

## 📜 License

MIT License.
