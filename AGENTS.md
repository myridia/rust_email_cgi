# AGENTS.md — rust_email_cgi

## What this is
Basic Rust CGI email processor for handling simple email webform submissions.

## Stack
- Rust (edition 2021)
- cgi (CGI interface)
- lettre (email sending)
- serde / serde_urlencoded (form parsing)

## Build
```bash
cargo build --release
```

## Run
Deploy as CGI script on a web server. Run with `run.sh`.

## Structure
- `src/main.rs` — CGI email handler
- `Cargo.toml` — Rust dependencies
- `run.sh` — run helper
- `release.sh` — release helper

## Conventions
- No comments in code unless asked.
- Verify: `cargo check`
