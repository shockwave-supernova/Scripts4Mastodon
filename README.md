# Mastodon Personal Infrastructure 🐘

A collection of self-hosted bots, maintenance scripts, and automation tools for my Mastodon instance.

## 📂 Repository Structure

This is a **monorepo**. Each tool is located in its own dedicated subdirectory and functions independently.

* **Rust Bots:** High-performance binaries for automated posting and media handling.
* **Bash/Python Scripts:** Maintenance, backups, and system utilities.

> ℹ️ **Documentation:** Please navigate to the specific subdirectory of the tool you are interested in to find its installation guide and usage instructions.

## 🚀 General Prerequisites

Most tools in this repository share a common setup philosophy:

1.  **Environment Variables:** Configuration is handled via `.env` files within each tool's directory.
2.  **Mastodon API:** You will need a Developer Access Token from your instance settings.
3.  **Linux Environment:** Designed to run on a VPS or Raspberry Pi (Systemd/Cron).

## ⚠️ Security Note

All `.env` files containing API keys and secrets are strictly git-ignored.
If you fork or clone this repository, you must create your own configuration files based on the instructions in the subfolders.

## 🛠 Usage

1.  Clone the repository:
    ```bash
    git clone [https://github.com/your-username/mastodon-scripts.git](https://github.com/your-username/mastodon-scripts.git)
    ```
2.  Enter the directory of the specific tool:
    ```bash
    cd unsplash_bot
    # or
    cd maintenance_scripts
    ```
3.  Follow the local `README.md` inside that folder.

## 📜 License

MIT License.
