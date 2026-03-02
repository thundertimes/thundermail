# ⚡ Thundermail

**Thundermail** is a high-performance, "Sovereign" email client written in 100% native **Rust**. It is designed to be the memory-safe, privacy-first successor to legacy mail engines, eliminating the webview stack (Chromium/WebKit) in favor of a lean, immediate-mode GUI (**egui**), radical metadata protection (**RFC 9788**), and local-first AI intelligence.

In an era of pervasive email metadata harvesting and the vulnerabilities exposed by research like `gpg.fail`, Thundermail implements the most aggressive privacy standards from the `user.js` and `12bytes` to ensure your communication remains truly yours.

---

## 🛡️ The Sovereign Standard

Thundermail is built on a **"Defense in Depth"** philosophy, specifically addressing the flaws in the "Digital Postcard" nature of traditional email.

* **Metadata Masking (RFC 9788):** The first Rust-native implementation of modern Header Protection. We move your `Subject` and `Recipient` data into the encrypted PGP packet, leaving only a generic "shroud" header for transit servers.
* **Forward Secrecy (FS):** Implements **Ephemeral Sub-keys** (Autocrypt v2 style). Thundermail rotates encryption sub-keys every 24 hours and uses the `zeroize` crate to scrub private material from memory, ensuring that a future compromise of your master key cannot decrypt past messages.
* **Native Rust UI (egui):** By using **egui** instead of Tauri or Electron, we eliminate the entire browser-exploit surface. No DOM, no XSS, no JavaScript---just pure, statically-linked Rust.
* **Zero-Leak Networking:** Native **SOCKS5/Tor** integration to defeat ISP-level traffic analysis (MITRE T1090.004). We enforce `#![forbid(unsafe_code)]` and use `rustls` for modern, memory-safe TLS 1.3.

---

## ✨ Features

* **Dual-Tier Private AI:**
  * **Local (Ollama):** Maximum sovereignty. Zero data leaves your machine.
  * **Private Cloud (Venice AI):** High-performance, uncensored inference with mandatory PII redaction and zero data retention.
* **Smart Labeling:** Database-driven (not folder-driven) categorization. Supports `X-GM-LABELS` to prevent data duplication and "phantom" IMAP fetches.
* **AI Segregation:** Automatic sorting into **Promotions**, **Social**, and **Updates** tabs using local context analysis via the `Categorizer` module.
* **Encrypted Search:** Lightning-fast full-text search via **Tantivy**, indexed locally in an encrypted SQLite store.
* **The Sanitizer:** Automatic stripping of tracking pixels, malicious CSS, and identifying signatures before rendering in the native UI.

---

## 🏗️ Technical Architecture

Thundermail is modularized to ensure that privacy logic is never bypassed by networking or UI code.**Layer****Responsibility****Technology**

| Layer        | Responsibility            | Technology              |
|--------------|---------------------------|-------------------------|
| UI           | Immediate-mode Native GUI | egui / eframe           |
| Protocol     | Async IMAP/SMTP           | tokio-imap, lettre      |
| Cryptography | RFC 9788 & Ephemeral Keys | sequoia-openpgp         |
| AI Engine    | Private Inference         | Ollama / Venice AI      |
| Storage      | Encrypted Metadata        | SQLx (SQLite) + Tantivy |
| Networking   | Proxy & TLS               | tokio-socks, rustls     |

---

## 🚀 Getting Started

### Prerequisites

* **Rust 1.75+**
* **Ollama** (Running locally) or a **Venice AI** API Key.
* **GnuPG 2.4.9+** (For legacy compatibility, though native Sequoia is preferred).

### Configuration

Thundermail uses a `config.toml` for "Sovereign Mode" presets. Secrets are managed via the system keyring.`

Bash

    cp config.toml.example ~/.config/thundermail/config.toml 

---

## 🤝 Contributing

We welcome contributors who believe in the right to private communication. Please read our **[SECURITY.md](https://www.google.com/search?q=./SECURITY.md)** and **[ARCHITECTURE.md](https://www.google.com/search?q=./ARCHITECTURE.md)** before submitting pull requests.
> 
> **The "No-Call" Rule:** Any PR introducing an external network dependency (fetching favicons, checking dictionaries, etc.) must be strictly opt-in and disabled by default.

## 📜 License

Distributed under the **MIT**
