---
title: Local ingest token and CORS boundary
date: 2026-05-12
category: security-issues
module: Tauri local HTTP ingest
problem_type: security_issue
component: authentication
symptoms:
  - "A committed shared ingest token was accepted by the desktop HTTP server and sent by the extension"
  - "CORS allowed any chrome-extension origin to call the local ingest API"
  - "A local ingest port conflict aborted Tauri setup instead of being visible runtime state"
root_cause: missing_permission
resolution_type: code_fix
severity: high
tags: [tauri, cors, extension, token, app-status]
---

# Local ingest token and CORS boundary

## Problem
The Task 7 implementation exposed a local HTTP ingest endpoint with an MVP-only committed token and permissive extension-origin CORS. It also treated ingest port bind failure as a setup error, so the desktop app could crash before any UI could explain the problem.

## Symptoms
- Rust and TypeScript both hard-coded the same shared token.
- `allowed_cors_origin` accepted every `chrome-extension://...` origin.
- `http::start_ingest_server` returned a bind error that `setup` propagated, aborting app startup.

## What Didn't Work
- A static token was not a meaningful boundary once committed to the repo.
- Matching the `chrome-extension://` scheme was too broad; any unrelated extension could satisfy that origin check.
- Returning the bind error directly from Tauri setup hid future recovery paths because the app never reached runtime state.

## Solution
Generate or load a per-install token from the Tauri app data directory and hand it to the local HTTP server at startup:

```rust
let ingest_token = token::load_or_create_ingest_token(&app_data_dir)?;
if let Err(error) = http::start_ingest_server(conn.clone(), ingest_token) {
    log::error!("{error}");
    if let Ok(mut current_error) = ingest_server_error.lock() {
        *current_error = Some(error);
    }
}
```

Expose a minimal `GET /config` handshake that returns `{ "ingestToken": "..." }` only with CORS headers for the deterministic companion extension origin. The extension gets a stable unpacked ID through `extension/manifest.json`'s `key`, and Rust restricts CORS to the derived origin:

```rust
pub const COMPANION_EXTENSION_ORIGIN: &str =
    "chrome-extension://okcchcpgcebnenmenmfjbpnnkjoepien";

fn allowed_cors_origin(origin: Option<&str>) -> Option<&str> {
    origin.filter(|origin| *origin == COMPANION_EXTENSION_ORIGIN)
}
```

The extension fetches `/config` before sending usage events, caches the token only in service-worker memory, skips event delivery when the config fetch fails, and retries the handshake on the next browser event.

## Why This Works
The accepted ingest token is no longer a repository-wide shared secret. A fresh install gets a random local token, and subsequent launches reuse the same local file. CORS no longer grants browser access to arbitrary extensions; only the extension ID derived from the manifest key receives `Access-Control-Allow-Origin`.

Port bind failure is now recorded in `AppState` and exposed by `app_status`, which preserves current ingest command behavior while allowing the app and future UI to report the local server problem.

## Prevention
- Do not commit local API bearer tokens, even for MVPs; generate and persist per-install secrets in app data.
- Treat CORS origin matching as an allowlist of exact origins, not a scheme or prefix check.
- Add tests for companion-origin rejection, config handshakes, extension retry behavior, and non-crashing startup status whenever local background services are introduced.

## Related Issues
- None found in `docs/solutions/`; this is the first documented local ingest boundary issue for this repo.
