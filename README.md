# Tauri Messenger

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Tauri](https://img.shields.io/badge/Tauri-2.0-24C8DB?logo=tauri)](https://tauri.app)
[![Svelte](https://img.shields.io/badge/Svelte-4.0-FF3E00?logo=svelte)](https://svelte.dev)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.0-3178C6?logo=typescript)](https://www.typescriptlang.org)

A lightweight, multi-service messenger client built with Tauri 2.0. Consumes significantly less RAM than Electron-based alternatives like Ferdium.

**Lightweight by default, powerful when needed.**

## ✨ Features

- **Multiple Services** – WhatsApp Web, Telegram, Slack, Discord, Messenger, and more
- **Multiple Accounts** – Add the same service multiple times with isolated sessions (cookies)
- **Native Notifications** – Receive OS-level notifications for new messages
- **Service Isolation** – Each service runs with its own cookie profile (no session conflicts)
- **Low RAM Usage** – ~500MB with 5 active services (Ferdium uses ~1.5GB+)
- **Dark/Light Mode** – Global theme support
- **Custom Icons** – Emoji picker included
- **Service Management** – Add, edit, and delete services easily


## 🚀 Installation

### Download Installer

Download the latest installer for your operating system from the [Releases](https://github.com/mcphisto-netizen/tauri-messenger/releases) page.

| Platform | File |
|----------|------|
| Windows | `tauri-messenger_x64.msi` or `.exe` |
| macOS | `tauri-messenger_x64.dmg` |
| Linux | `tauri-messenger_amd64.deb` or `.AppImage` |

### Build from Source

```bash
# Clone the repository
git clone https://github.com/mcphisto-netizen/tauri-messenger.git
cd tauri-messenger

# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
