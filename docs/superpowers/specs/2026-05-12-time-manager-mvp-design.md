# Time Manager MVP Design

## Goal

Build a Windows-first desktop time manager that records browser website usage and shows productivity statistics. The first version tracks time only; it does not block sites, send notifications, sync data, or support macOS.

## Product Scope

The MVP uses a browser extension plus a Windows desktop app:

- Browser extension: reads the active tab URL in Chrome and Edge compatible browsers.
- Windows desktop app: receives usage events, stores local records, classifies domains, and shows statistics.
- Local database: stores all usage and rule data on the user's machine.

Out of scope for the MVP:

- Site blocking.
- Time limit alerts.
- Cloud sync or accounts.
- macOS support.
- Tracking arbitrary desktop apps such as VS Code, games, messengers, or Notion desktop.

## Recommended Architecture

Use Tauri, React, TypeScript, and SQLite for the desktop app. Use a Chromium extension for Chrome and Edge.

Tauri is preferred over Electron because the MVP should feel like a lightweight Windows utility. React and TypeScript provide a practical UI stack, while SQLite is enough for durable local data without running a server.

## Components

### Browser Extension

The extension observes active tab changes, URL changes, browser focus, and idle transitions where supported by the extension APIs. It sends normalized usage events to the local desktop app.

Initial event fields:

- `url`
- `domain`
- `title`
- `browser`
- `eventType`
- `timestamp`

The extension should avoid sending browsing history in bulk. It only sends active usage events needed for time tracking.

### Desktop App

The desktop app provides the main user interface and local data layer.

Primary screens:

- Dashboard: today's productive, unproductive, and neutral time.
- Sites: site-level usage totals and classification.
- Trends: simple weekly usage chart.
- Rules: editable domain classification rules.

The first screen should be the dashboard, not a landing page.

### Local Ingestion API

The extension needs a local way to communicate with the desktop app. The preferred MVP path is a local HTTP endpoint hosted by the Tauri sidecar or Rust backend while the app is running.

The local endpoint accepts active usage events, validates them, classifies the domain, and writes usage intervals to SQLite.

Native messaging is not part of the initial implementation. The design should keep the extension transport isolated so native messaging can replace local HTTP later if packaging or security requirements demand it.

### Storage

Use SQLite tables for:

- `usage_events`: raw normalized events from the extension.
- `usage_sessions`: derived intervals of active usage.
- `domain_rules`: user-editable domain classifications.

Classifications:

- `productive`
- `unproductive`
- `neutral`

Initial default rules:

- Productive: `chatgpt.com`, `chat.openai.com`, and user-added Codex domains.
- Unproductive: `youtube.com`, `instagram.com`.
- Neutral: all unmatched domains.

## Data Flow

1. User focuses a browser tab.
2. Extension detects the active tab URL and timestamp.
3. Extension sends a usage event to the desktop app.
4. Desktop app validates and normalizes the event.
5. Desktop app closes the previous active interval and opens or updates the current interval.
6. Desktop app classifies the domain using `domain_rules`.
7. UI reads aggregated data from SQLite for dashboard and trend views.

## Time Tracking Rules

The app should count only active browser usage. It should stop or pause counting when:

- The browser loses focus.
- The machine becomes idle.
- The active tab has no valid URL.
- The extension cannot reach the desktop app.

If events arrive late or out of order, the app should cap or ignore intervals that would create unrealistic durations.

## Error Handling

When the desktop app is not running, the extension keeps a small in-memory queue and retries briefly. The MVP does not need durable offline buffering in the extension.

If the local ingestion endpoint rejects an event, the extension should fail silently for users and expose diagnostic details only in the extension console.

If the database is unavailable or corrupted, the desktop app should show a clear local error state and avoid deleting existing data automatically.

## Privacy

All data remains local. The app should not send usage records to external servers.

The UI should make it clear that records are stored locally. The MVP does not need account creation, telemetry, or remote analytics.

## Testing Strategy

Test coverage should focus on the behavior most likely to corrupt statistics:

- Domain normalization.
- Classification rule matching.
- Usage interval creation and closing.
- Aggregation by day and week.
- Handling duplicate, missing, or out-of-order events.

Manual verification should cover:

- Chrome active tab tracking.
- Edge active tab tracking.
- Desktop app running and not running.
- Switching between productive, unproductive, and neutral sites.

## Implementation Notes

Keep the transport layer, classification logic, storage logic, and UI aggregation separate. The extension should not decide whether a site is productive; the desktop app owns classification so user rules apply consistently.

Do not implement notifications, limits, blocking, or desktop-wide app tracking in this MVP.
