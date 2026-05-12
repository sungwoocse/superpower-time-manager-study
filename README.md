# Superpower Time Manager Study

Windows-first browser time tracking MVP for personal productivity study.

## Scope

- Chromium browser extension for active tab tracking.
- Tauri desktop app for local storage, ingestion, and MVP dashboard surfaces.
- SQLite local storage.
- No blocking, alerts, cloud sync, accounts, macOS support, or desktop-wide app tracking in the MVP.

## Development Status

This repository contains the MVP app, extension, local ingest API, and manual verification notes.

## Development

Install dependencies:

```powershell
npm install
```

Run the Tauri desktop app in development mode:

```powershell
npm run tauri dev
```

Build the Chromium extension for Chrome or Edge:

```powershell
npm run extension:build
```

Run verification before review:

```powershell
npm test
npm run build
npm run extension:build
cargo test --manifest-path src-tauri/Cargo.toml
cargo check --manifest-path src-tauri/Cargo.toml
```

If `cargo` is not on `PATH`, use the bundled Rust install path from `src-tauri`:

```powershell
Push-Location src-tauri
& "$env:USERPROFILE\.cargo\bin\cargo.exe" test
& "$env:USERPROFILE\.cargo\bin\cargo.exe" check
Pop-Location
```

Manual verification steps are in [docs/manual-test.md](docs/manual-test.md).
