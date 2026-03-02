## 

## 📄 SECURITY.md

This document outlines the security policies, threat model, and vulnerability reporting process for **Thundermail**. As a project focused on "Sovereign" communication, we prioritize cryptographic integrity and metadata privacy above convenience.

---

### 🛡️ Security Policy**Version****Status**

**0.2.x**✅ Current Release (Supported)

**0.1.x**⚠️ Security Patches Only

**< 0.1.0**❌ End of Life

---

### 🏗️ Our Threat Model

Thundermail is designed to defend against the following specific adversaries:

1. **Passive Network Adversaries:** ISPs or state actors performing traffic analysis or metadata harvesting. (Mitigated via **Tor/SOCKS5** and **RFC 9788**).
2. **Malicious Mail Servers:** IMAP/SMTP providers attempting to modify message headers or peek at subjects. (Mitigated via **LAMPS Header Protection** and **MDC Enforcement**).
3. **Local "Data-Slurping" Software:** Anti-virus or telemetry agents scanning local mail stores. (Mitigated via **Encrypted SQLite/Tantivy** storage).
4. **AI Surveillance:** Cloud LLMs training on sensitive user drafts. (Mitigated via **Local Ollama** and **Venice AI Sanitization**).

---

### 🔒 Reporting a Vulnerability

**Please do not report security vulnerabilities via public GitHub issues.**

If you discover a security flaw (especially related to memory safety, cryptographic bypass, or unintended network calls), please follow these steps:

1. **Email:** Send an encrypted message to `security@thundermail.org` (or the lead maintainer's PGP key).
2. **Details:** Include a proof-of-concept (PoC), the version of Thundermail used, and your operating environment.
3. **Response:** We will acknowledge your report within **48 hours** and provide a timeline for a fix.

---

### 🧪 Critical Security Requirements for Contributors

To maintain our "Sovereign" status, every Pull Request (PR) must adhere to these rules:

#### 1\. The "No-Call" Rule

The core must never initiate a network connection (HTTPS, DNS, etc.) that is not explicitly defined in the IMAP/SMTP/Proxy configuration. This includes:

* ❌ No Gravatar/Favicon fetching.
* ❌ No automatic update checks (must be manual or via package manager).
* ❌ No "Safe Browsing" API calls.

#### 2\. Cryptographic Integrity (MDC)

Thundermail follows a **"Fail-Hard"** approach inspired by the `gpg.fail` research.

* If an OpenPGP message lacks a **Modification Detection Code (MDC)** or the MDC is invalid, the client **MUST NOT** render any part of the message body.

#### 3\. Memory Safety

We enforce `#![forbid(unsafe_code)]` in all modules except for low-level FFI bindings (which must be audited). We use the `zeroize` crate for all buffers containing:

* IMAP/SMTP passwords.
* PGP Passphrases.
* Decrypted plaintext.

#### 4\. Header Protection (RFC 9788)

Any changes to the drafting or sending logic must preserve the **LAMPS** header wrapping. Sensitive headers (`Subject`, `From`, `Reply-To`) must remain in the encrypted payload.

---

### 📜 Disclosure Policy

We follow coordinated vulnerability disclosure. Once a fix is deployed, we will publish a **Security Advisory** with credit to the researcher (unless anonymity is requested).
