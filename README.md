# Superpower Time Manager Study

Windows-first browser time tracking MVP for personal productivity study.

## Scope

- Chromium browser extension for active tab tracking.
- Tauri desktop app for local statistics.
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
C:\Users\bluew\.cargo\bin\cargo.exe test
C:\Users\bluew\.cargo\bin\cargo.exe check
```

Manual verification steps are in [docs/manual-test.md](docs/manual-test.md).
