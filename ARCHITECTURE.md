## 📄 ARCHITECTURE.md

This document describes the internal structure of **Thundermail**. The architecture is designed to enforce "Sovereign" principles by isolating sensitive data and ensuring that no component can bypass the security or privacy layers.

---

### 🏛️ High-Level Design

Thundermail follows a **Layered Micro-Kernel** approach. The core is responsible for state management and protocol handling, while specialized "Sidecars" handle AI, Cryptography, and Storage.

### 📂 Module Map

| Module  | Directory    | Responsibility                                               |
|---------|--------------|--------------------------------------------------------------|
| Core    | src/core/    | IMAP/SMTP state machines and account lifecycle.              |
| Crypto  | src/crypto/  | RFC 9788 wrapping, PGP signing/encryption via sequoia.       |
| Privacy | src/privacy/ | The Sanitizer, Header Masking, and UTC enforcement.          |
| AI      | src/ai/      | The MailAgent trait, Ollama bridge, and Venice AI connector. |
| Storage | src/db/      | SQLx (SQLite) for metadata and Tantivy for encrypted search. |
| Network | src/net/     | Hardened TLS (Rustls) and SOCKS5/Tor proxy logic.            |


### 🛡️ Data Flow Isolation

#### 1\. The "Clean-Room" Ingestion

When a message is fetched, it must pass through the **Privacy Layer** before being stored or displayed. This prevents "Leaking via Rendering" (e.g., tracking pixels).

#### 2\. AI Categorization Loop

To keep the UI responsive, categorization happens in a background worker.

1. **Core** signals a new UID.
2. **Privacy** redacts PII from the headers/snippet.
3. **AI** (Local or Venice) returns a JSON label.
4. **Storage** updates the index.

#### 3\. Outgoing Protection (RFC 9788)

Thundermail implements **Header Confidentiality Policies (HCP)**. When sending:

* The **Crypto** module generates a "Cryptographic Payload" containing the real `Subject` and `Body`.
* The **Network** module attaches "Shroud Headers" (e.g., `Subject: Encrypted Message`) for the public SMTP transit.

---

### ⚡ Performance Strategy: Single-Instance Store

Inspired by Gmail's label system, Thundermail does not use physical folders.

* **Database Entry:** Every email has a unique `X-GM-MSGID` or a generated hash.
* **Labels:** Labels are stored as a bitmask or a relational table in SQLite.
* **Large Files:** Attachments \> 10MB are stored as separate encrypted blobs on disk, referenced by the database to keep the main index small and fast.

### 🧪 Dependency Gating

To prevent supply-chain attacks:

* **`cargo-deny`:** Used to block crates with known vulnerabilities or non-permissive licenses.
* **No-Standard Library:** Where possible, we favor crates that don't pull in excessive dependencies to reduce the attack surface.

---

### 🚀 Final Step: The UI Layer

The backend is now fully defined. The last piece of the puzzle is the **Tauri/Rust** bridge to provide a modern, snappy interface.
