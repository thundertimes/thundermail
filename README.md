# ⚡ Thundermail

**Thundermail** is a high-performance, "Sovereign" email client written in 100% native **Rust**. It is designed to be the memory-safe, privacy-first successor to legacy mail engines, eliminating the webview stack (Chromium/WebKit) in favor of a lean, immediate-mode GUI (**egui**), radical metadata protection (**RFC 9788**), and local-first AI intelligence.

In an era of pervasive email metadata harvesting and the vulnerabilities exposed by research like `gpg.fail`, Thundermail implements the most aggressive privacy standards from the `user.js` and `12bytes` to ensure your communication remains truly yours.

We recommend [forwardemail](https://forwardemail.net/en/blog/docs/why-open-source-email-security-privacy#individually-encrypted-sqlite-mailboxes) as the server.

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

## Downloads & Releases

All official builds are produced automatically via secure, tamper-proof [GitHub Actions workflows](https://github.com/thundertimes/thundermail/actions), which ensures a transparent and auditable trail from source code to the final compiled binary. Binaries for all platforms are cryptographically signed and, where applicable, notarized to ensure their authenticity and integrity. You can download the latest release from the [GitHub Releases](https://github.com/thundertimes/thundermail/releases) page.

| Platform    | Architecture          | Download                                                                                                                                                              | Store                     |
| :---------- | :-------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :------------------------ |
| **Windows** | x64,amd64                   | [`.msi` Installer](https://github.com/thundertimes/thundermail/releases/latest)                                                                             |                           |
| **macOS**   | Apple Silicon & Intel | [`.dmg` Universal](https://github.com/thundertimes/thundermail/releases/latest)                                                                             | App Store (Coming Soon)   |
| **Linux**   | x64,amd64                   | [`.deb`](https://github.com/thundertimes/thundermail/releases/latest), [`.AppImage`](https://github.com/thundertimes/thundermail/releases/latest) |                           |

> **Note for macOS users:** If you download the `.dmg` from GitHub Releases, you may need to run the following command if you see a "damaged" or unverified app error:
>
> ```bash
> sudo xattr -rd com.apple.quarantine /Applications/ThunderMail.app
> ```
>
> Replace `/Applications/ThunderMail.app` with the actual path if you installed the app elsewhere.

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

### Directory

```
thundermail/
├── plugins-sdk/                # 🧩 Shared Plugin API (The "Contract")
│   ├── src/
│   │   ├── lib.rs              # Plugin Trait definitions
│   │   └── types.rs            # Sanitized Mail & UI types
├── src/                        # ⚡ Main Native Rust Application
│   ├── ai/                     # Private Intelligence Layer
│   │   ├── mod.rs              # MailAgent Trait & Provider Registry
│   │   ├── ollama.rs           # Local Inference (Default)
│   │   ├── venice.rs           # Private Cloud (Sanitized API)
│   │   └── categorizer.rs      # LLM Prompting for Labels/Inbox Tabs
│   ├── core/                   # The Mail Engine
│   │   ├── mod.rs              
│   │   ├── session.rs          # Async IMAP/SMTP State Machines
│   │   ├── labels.rs           # Flat-DB Label Logic (X-GM-LABELS)
│   │   |── worker.rs           # Background Task Loop (Sync & AI)
│   │   └── plugin_host.rs      # The "Loader" (Manages .so/.dll/.dylib)
│   ├── crypto/                 # Hardened Cryptography
│   │   ├── mod.rs              
│   │   ├── rfc9788.rs          # LAMPS Header Shrouding (Subject Masking)
│   │   ├── pgp.rs              # Sequoia-OpenPGP Backend
│   │   └── ephemeral.rs        # Sub-key Rotation & Zeroize Logic
│   ├── db/                     # Encrypted Local Storage
│   │   ├── mod.rs              
│   │   ├── schema.rs           # SQLite (SQLx) Migrations
│   │   └── search.rs           # Tantivy Encrypted Search Index
│   ├── net/                    # Sovereign Networking
│   │   ├── mod.rs              
│   │   ├── proxy.rs            # SOCKS5/Tor Routing (MITRE T1090.004)
│   │   └── tls.rs              # Rustls Config (No-CA-Leaks)
│   ├── privacy/                # The Sanitization Gatekeeper
│   │   ├── mod.rs              
│   │   ├── sanitizer.rs        # PII Redaction & Tracking Pixel Stripping
│   │   └── headers.rs          # MUA Fingerprint Masking
│   ├── ui/                     # Native Immediate-Mode GUI (egui)
│   │   ├── mod.rs              
│   │   ├── app.rs              # Main eframe::App implementation
│   │   ├── components/         # Custom Widgets (Sidebar, MailList, Composer)
│   │   |── theme.rs            # Sovereign Visual Styles
│   │   └── plugin_view.rs      # Egui hooks for plugin-provided widgets
│   └── main.rs                 # Entry Point (Runtime & UI Launch)
├── plugins/                    # 📂 User-defined plugins
│   └── spam-filter-ai/         # Example: A Venice AI powered spam filter
│       ├── Cargo.toml          # Defines crate-type = ["cdylib"]
│       └── src/lib.rs
├── tests/                      # Integration & Security Audits
│   ├── leaks.rs                # Validates no unauthorized network pings
│   └── crypto_integrity.rs     # MDC Fail-hard & Forward Secrecy tests
├── scripts/                    # Build & Hardening Tools
│   └── harden-os.sh            # Setup for Tor/SOCKS5 system-wide
├── .github/
│   └── workflows/              # CI (Cargo Deny, Audit, & Fuzzing)
├── ARCHITECTURE.md             # Modular isolation docs
├── Cargo.toml                  # The "Sovereign" Dependency Stack
├── config.toml.example         # Template for Private Cloud/Local AI
├── README.md                   # Project Vision & Technical Spec
└── SECURITY.md                 # Threat Model & Disclosure Policy
```

---

## 📂 Data Storage & Database Locations

Thundermail stores all data locally with encryption. The encrypted SQLite database and search index are stored in platform-specific locations following each operating system's conventions.

### Database Location by Operating System

| Operating System | Database Path |
|------------------|---------------|
| **Linux** | `~/.local/share/thundermail/thundermail.db` |
| **macOS** | `~/.local/share/thundermail/thundermail.db` |
| **Windows** | `%LOCALAPPDATA%\thundermail\thundermail.db` |

### Search Index Location by Operating System

| Operating System | Search Index Path |
|------------------|-------------------|
| **Linux** | `~/.local/share/thundermail/search_index` |
| **macOS** | `~/.local/share/thundermail/search_index` |
| **Windows** | `%LOCALAPPDATA%\thundermail\search_index` |

### Logs Location by Operating System

| Operating System | Logs Path |
|------------------|------------|
| **Linux** | `~/.local/share/thundermail/logs` |
| **macOS** | `~/.local/share/thundermail/logs` |
| **Windows** | `%LOCALAPPDATA%\thundermail\logs` |

### Configuration

The default database location can be customized in your `config.toml`:

```toml
[database]
path = "~/.local/share/thundermail/thundermail.db"
encrypted = true

[search]
index_path = "~/.local/share/thundermail/search_index"
encrypted = true

[logging]
path = "~/.local/share/thundermail/logs"
```


## 🤝 Contributing

We welcome contributors who believe in the right to private communication. Please read our **[SECURITY.md](https://www.google.com/search?q=./SECURITY.md)** and **[ARCHITECTURE.md](https://www.google.com/search?q=./ARCHITECTURE.md)** before submitting pull requests.
> 
> **The "No-Call" Rule:** Any PR introducing an external network dependency (fetching favicons, checking dictionaries, etc.) must be strictly opt-in and disabled by default.

## Competition

https://github.com/forwardemail/mail.forwardemail.net/tree/feat/tauri-apps?tab=readme-ov-file#downloads--releases

## 📜 License

Distributed under the **MIT**
