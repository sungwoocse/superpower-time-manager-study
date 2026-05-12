# Manual Test

Use these checks before uploading or reviewing the Windows-first MVP. The app is a Tauri desktop app with local SQLite storage, and the browser tracker is a Chromium Manifest V3 extension that can be loaded in Chrome or Edge.

## Prerequisites

1. From the repository root, install dependencies:

   ```powershell
   npm install
   ```

2. Build the extension:

   ```powershell
   npm run extension:build
   ```

3. Start the desktop app and local ingest server:

   ```powershell
   npm run tauri dev
   ```

4. Confirm the app window opens and the dashboard renders summary cards, the Sites table, and the Rules list.

## Desktop App

1. Keep `npm run tauri dev` running.
2. Confirm the local ingest server starts on `127.0.0.1:51247`.
3. Confirm the dev terminal does not log an ingest server startup error.
4. Confirm the dashboard text is readable and does not overlap at the default window size.
5. Confirm the local database exists after startup:

   ```powershell
   Test-Path "$env:APPDATA\com.sungwoocse.superpower-time-manager\time_manager.sqlite3"
   ```

## Chrome Extension

1. Open Chrome to `chrome://extensions`.
2. Enable Developer mode.
3. Select Load unpacked and choose the repository `extension` folder.
4. Confirm the extension ID is `okcchcpgcebnenmenmfjbpnnkjoepien`.
5. Confirm `extension/dist/background.js` is present. If it is missing, rerun `npm run extension:build` and reload the extension.
6. With the desktop app still running, open and switch between HTTP or HTTPS pages such as:
   - `https://chatgpt.com`
   - `https://www.youtube.com`
   - `https://www.instagram.com`
7. Leave the Chrome window focused and the OS idle state active while switching tabs. The MVP only sends active tab events for the focused browser window.
8. Confirm usage events are inserted into SQLite:

   ```powershell
   sqlite3 "$env:APPDATA\com.sungwoocse.superpower-time-manager\time_manager.sqlite3" "select id, domain, title, browser, event_type, timestamp from usage_events order by id desc limit 10;"
   ```

9. Confirm rows appear for the visited domains. The extension reports `browser` as `unknown` in the current MVP.

## Edge Extension

1. Open Edge to `edge://extensions`.
2. Enable Developer mode.
3. Select Load unpacked and choose the repository `extension` folder.
4. Confirm the extension ID is `okcchcpgcebnenmenmfjbpnnkjoepien`.
5. With the desktop app still running, repeat the same focused-window site switching checks from Chrome.
6. Confirm new rows appear in `usage_events` using the same SQLite query.

## Local Ingest Notes

- The extension posts usage events to `http://127.0.0.1:51247/usage-events`.
- The extension first reads `http://127.0.0.1:51247/config` to get the per-install ingest token, then sends it in the `x-time-manager-token` header.
- CORS is limited to the companion extension origin `chrome-extension://okcchcpgcebnenmenmfjbpnnkjoepien`. The fixed manifest key keeps that extension ID stable for local unpacked Chrome and Edge testing. If the extension ID changes, the desktop app will not allow the extension origin until the companion origin constant is updated.
- `chrome://`, `edge://`, and other non-HTTP URLs are ignored by the extension.
