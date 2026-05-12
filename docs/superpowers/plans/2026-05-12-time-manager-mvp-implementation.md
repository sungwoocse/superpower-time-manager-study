# Time Manager MVP Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Windows 우선 브라우저 사용 시간 기록/통계 MVP를 Tauri 데스크탑 앱과 Chromium 확장 프로그램으로 구현한다.

**Architecture:** Chromium 확장 프로그램이 활성 탭 이벤트를 로컬 HTTP 수집 API로 보내고, Tauri 앱이 이벤트를 SQLite에 저장한 뒤 React UI에서 오늘/주간/사이트별 통계를 보여준다. 분류 책임은 데스크탑 앱이 가지며, 확장 프로그램은 URL 관측과 이벤트 전송만 담당한다.

**Tech Stack:** Tauri, React, TypeScript, Vite, Vitest, Rust, SQLite, Chromium Manifest V3.

---

## 현재 상태

작업 폴더: `C:\Users\bluew\Desktop\MyWork\02_current\05_CodeProject\24_timeManager`

현재 존재하는 파일:

- `docs/superpowers/specs/2026-05-12-time-manager-mvp-design.md`
- `docs/superpowers/specs/2026-05-12-time-manager-mvp-design-kor.md`
- `docs/superpowers/plans/2026-05-12-time-manager-mvp-implementation.md`

현재 폴더는 이 계획 작성 시점에 git 저장소가 아니다.

## 구현 파일 구조

생성할 주요 경로:

- `package.json`: 루트 npm 스크립트와 workspace 진입점.
- `src/`: Tauri React 앱 프론트엔드.
- `src/shared/`: 프론트엔드에서 사용하는 순수 TypeScript 도메인 로직.
- `src/components/`: 대시보드, 사이트 목록, 규칙 편집 UI 컴포넌트.
- `src-tauri/`: Tauri Rust 백엔드.
- `src-tauri/src/`: SQLite 저장소, 수집 API, 집계 커맨드.
- `extension/`: Chromium Manifest V3 확장 프로그램.
- `extension/src/`: 확장 background service worker 소스.
- `docs/manual-test.md`: Chrome/Edge 수동 검증 절차.

책임 분리:

- 확장 프로그램은 URL 관측과 이벤트 전송만 한다.
- Rust 백엔드는 이벤트 검증, SQLite 저장, 집계를 맡는다.
- TypeScript shared 로직은 UI 표시용 도메인 정규화와 타입 정의를 맡는다.
- React 컴포넌트는 데이터를 표시하고 규칙 편집 명령을 호출한다.

## Task 1: 저장소 초기화와 기본 문서 커밋

**Files:**
- Create: `.gitignore`
- Create: `README.md`
- Modify: existing docs only through git staging

- [ ] **Step 1: git 저장소를 초기화한다**

Run:

```powershell
git init
```

Expected: `.git` 디렉터리가 생성된다.

- [ ] **Step 2: `.gitignore`를 작성한다**

Create `.gitignore`:

```gitignore
node_modules/
dist/
dist-ssr/
.vite/
.turbo/
target/
src-tauri/target/
*.log
*.sqlite
*.sqlite3
.env
.env.*
!.env.example
extension/dist/
```

- [ ] **Step 3: `README.md`를 작성한다**

Create `README.md`:

```markdown
# Superpower Time Manager Study

Windows-first browser time tracking MVP for personal productivity study.

## Scope

- Chromium browser extension for active tab tracking.
- Tauri desktop app for local statistics.
- SQLite local storage.
- No blocking, alerts, cloud sync, accounts, macOS support, or desktop-wide app tracking in the MVP.

## Development Status

The current repository contains design and implementation planning documents. Application scaffolding comes next.
```

- [ ] **Step 4: 문서와 기본 파일을 커밋한다**

Run:

```powershell
git add .gitignore README.md docs
git commit -m "docs: add time manager MVP design and plan"
```

Expected: first commit is created.

## Task 2: Tauri React TypeScript 앱 스캐폴드

**Files:**
- Create/Modify: `package.json`
- Create: `index.html`
- Create: `src/main.tsx`
- Create: `src/App.tsx`
- Create: `src/styles.css`
- Create: `src-tauri/tauri.conf.json`
- Create: `src-tauri/Cargo.toml`
- Create: `src-tauri/src/main.rs`

- [ ] **Step 1: Rust 도구 설치 여부를 확인한다**

Run:

```powershell
rustc --version
cargo --version
```

Expected: both commands print versions. If either command is not found, stop this task and install Rust from the official Rust installer before continuing.

- [ ] **Step 2: Vite React TypeScript 앱을 생성한다**

Run:

```powershell
npm create vite@latest . -- --template react-ts
```

Expected: `package.json`, `index.html`, `src/`, `vite.config.ts`, `tsconfig.json` are created.

- [ ] **Step 3: Tauri를 추가한다**

Run:

```powershell
npm install
npm install -D @tauri-apps/cli
npm run tauri init -- --app-name "Superpower Time Manager" --window-title "Superpower Time Manager" --frontend-dist "../dist" --dev-url "http://localhost:5173" --before-dev-command "npm run dev" --before-build-command "npm run build"
```

Expected: `src-tauri/` is created.

- [ ] **Step 4: 앱 첫 화면을 MVP 대시보드 자리로 바꾼다**

Replace `src/App.tsx`:

```tsx
import "./styles.css";

export function App() {
  return (
    <main className="app-shell">
      <header className="app-header">
        <div>
          <p className="eyebrow">Windows MVP</p>
          <h1>Superpower Time Manager</h1>
        </div>
      </header>
      <section className="summary-grid" aria-label="Today summary">
        <article>
          <span>Productive</span>
          <strong>0m</strong>
        </article>
        <article>
          <span>Unproductive</span>
          <strong>0m</strong>
        </article>
        <article>
          <span>Neutral</span>
          <strong>0m</strong>
        </article>
      </section>
    </main>
  );
}

export default App;
```

Replace `src/styles.css`:

```css
:root {
  color: #172026;
  background: #f6f7f4;
  font-family: Inter, Segoe UI, Arial, sans-serif;
}

body {
  margin: 0;
}

.app-shell {
  min-height: 100vh;
  padding: 32px;
}

.app-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 24px;
}

.eyebrow {
  margin: 0 0 6px;
  color: #5f6b5d;
  font-size: 12px;
  text-transform: uppercase;
}

h1 {
  margin: 0;
  font-size: 28px;
  letter-spacing: 0;
}

.summary-grid {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 12px;
}

.summary-grid article {
  border: 1px solid #d8ded2;
  border-radius: 8px;
  background: #ffffff;
  padding: 18px;
}

.summary-grid span {
  display: block;
  color: #5f6b5d;
  font-size: 13px;
}

.summary-grid strong {
  display: block;
  margin-top: 10px;
  font-size: 30px;
}
```

- [ ] **Step 5: 빌드를 확인한다**

Run:

```powershell
npm run build
```

Expected: TypeScript build and Vite build succeed.

- [ ] **Step 6: 커밋한다**

Run:

```powershell
git add package.json package-lock.json index.html src src-tauri vite.config.ts tsconfig*.json
git commit -m "feat: scaffold Tauri React app"
```

Expected: scaffold commit is created.

## Task 3: TypeScript 도메인 타입과 분류 로직

**Files:**
- Create: `src/shared/types.ts`
- Create: `src/shared/domain.ts`
- Create: `src/shared/domain.test.ts`
- Modify: `package.json`

- [ ] **Step 1: Vitest를 설치한다**

Run:

```powershell
npm install -D vitest jsdom @testing-library/react @testing-library/jest-dom
```

Expected: dev dependencies are installed.

- [ ] **Step 2: 테스트 스크립트를 추가한다**

Modify `package.json` scripts:

```json
{
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "preview": "vite preview",
    "tauri": "tauri",
    "test": "vitest run"
  }
}
```

- [ ] **Step 3: 타입 파일을 만든다**

Create `src/shared/types.ts`:

```ts
export type Classification = "productive" | "unproductive" | "neutral";

export interface UsageEvent {
  url: string;
  domain: string;
  title: string;
  browser: "chrome" | "edge" | "unknown";
  eventType: "focus" | "blur" | "idle" | "active";
  timestamp: string;
}

export interface DomainRule {
  domain: string;
  classification: Classification;
}
```

- [ ] **Step 4: 실패하는 도메인 테스트를 작성한다**

Create `src/shared/domain.test.ts`:

```ts
import { describe, expect, it } from "vitest";
import { classifyDomain, normalizeDomain } from "./domain";

describe("normalizeDomain", () => {
  it("extracts hostname and removes www prefix", () => {
    expect(normalizeDomain("https://www.youtube.com/watch?v=abc")).toBe("youtube.com");
  });

  it("returns empty string for invalid URLs", () => {
    expect(normalizeDomain("not a url")).toBe("");
  });
});

describe("classifyDomain", () => {
  it("matches exact domains and subdomains", () => {
    const rules = [{ domain: "youtube.com", classification: "unproductive" as const }];

    expect(classifyDomain("youtube.com", rules)).toBe("unproductive");
    expect(classifyDomain("m.youtube.com", rules)).toBe("unproductive");
  });

  it("returns neutral when no rule matches", () => {
    expect(classifyDomain("example.com", [])).toBe("neutral");
  });
});
```

- [ ] **Step 5: 테스트 실패를 확인한다**

Run:

```powershell
npm test -- src/shared/domain.test.ts
```

Expected: FAIL because `src/shared/domain.ts` does not exist.

- [ ] **Step 6: 도메인 로직을 구현한다**

Create `src/shared/domain.ts`:

```ts
import type { Classification, DomainRule } from "./types";

export function normalizeDomain(url: string): string {
  try {
    const hostname = new URL(url).hostname.toLowerCase();
    return hostname.startsWith("www.") ? hostname.slice(4) : hostname;
  } catch {
    return "";
  }
}

export function classifyDomain(domain: string, rules: DomainRule[]): Classification {
  const normalized = domain.toLowerCase();
  const matched = rules.find((rule) => {
    const ruleDomain = rule.domain.toLowerCase();
    return normalized === ruleDomain || normalized.endsWith(`.${ruleDomain}`);
  });

  return matched?.classification ?? "neutral";
}
```

- [ ] **Step 7: 테스트 통과를 확인한다**

Run:

```powershell
npm test -- src/shared/domain.test.ts
```

Expected: PASS.

- [ ] **Step 8: 커밋한다**

Run:

```powershell
git add package.json package-lock.json src/shared
git commit -m "feat: add domain classification logic"
```

Expected: classification commit is created.

## Task 4: Rust SQLite 저장소와 기본 규칙

**Files:**
- Modify: `src-tauri/Cargo.toml`
- Create: `src-tauri/src/db.rs`
- Create: `src-tauri/src/models.rs`
- Modify: `src-tauri/src/main.rs`

- [ ] **Step 1: Rust 의존성을 추가한다**

Modify `src-tauri/Cargo.toml` dependencies:

```toml
[dependencies]
tauri = { version = "2", features = [] }
tauri-plugin-shell = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
rusqlite = { version = "0.31", features = ["bundled"] }
chrono = { version = "0.4", features = ["serde"] }
```

- [ ] **Step 2: 모델을 작성한다**

Create `src-tauri/src/models.rs`:

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Classification {
    Productive,
    Unproductive,
    Neutral,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsageEvent {
    pub url: String,
    pub domain: String,
    pub title: String,
    pub browser: String,
    pub event_type: String,
    pub timestamp: String,
}
```

- [ ] **Step 3: 저장소 테스트를 먼저 작성한다**

Create `src-tauri/src/db.rs` with tests first:

```rust
use rusqlite::{Connection, Result};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initializes_default_domain_rules() {
        let conn = Connection::open_in_memory().unwrap();
        init_db(&conn).unwrap();

        let count: i64 = conn
            .query_row("select count(*) from domain_rules", [], |row| row.get(0))
            .unwrap();

        assert_eq!(count, 4);
    }
}
```

- [ ] **Step 4: Rust 테스트 실패를 확인한다**

Run:

```powershell
cd src-tauri
cargo test db::tests::initializes_default_domain_rules
```

Expected: FAIL because `init_db` is not implemented.

- [ ] **Step 5: SQLite 초기화를 구현한다**

Replace `src-tauri/src/db.rs`:

```rust
use rusqlite::{params, Connection, Result};

pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
        create table if not exists usage_events (
            id integer primary key autoincrement,
            url text not null,
            domain text not null,
            title text not null,
            browser text not null,
            event_type text not null,
            timestamp text not null,
            created_at text not null default current_timestamp
        );

        create table if not exists usage_sessions (
            id integer primary key autoincrement,
            domain text not null,
            title text not null,
            browser text not null,
            classification text not null,
            started_at text not null,
            ended_at text
        );

        create table if not exists domain_rules (
            id integer primary key autoincrement,
            domain text not null unique,
            classification text not null
        );
        ",
    )?;

    for (domain, classification) in [
        ("chatgpt.com", "productive"),
        ("chat.openai.com", "productive"),
        ("youtube.com", "unproductive"),
        ("instagram.com", "unproductive"),
    ] {
        conn.execute(
            "insert or ignore into domain_rules (domain, classification) values (?1, ?2)",
            params![domain, classification],
        )?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initializes_default_domain_rules() {
        let conn = Connection::open_in_memory().unwrap();
        init_db(&conn).unwrap();

        let count: i64 = conn
            .query_row("select count(*) from domain_rules", [], |row| row.get(0))
            .unwrap();

        assert_eq!(count, 4);
    }
}
```

- [ ] **Step 6: 테스트 통과를 확인한다**

Run:

```powershell
cd src-tauri
cargo test db::tests::initializes_default_domain_rules
```

Expected: PASS.

- [ ] **Step 7: 모듈을 연결한다**

Modify `src-tauri/src/main.rs`:

```rust
mod db;
mod models;

fn main() {
    tauri::Builder::default()
        .setup(|_app| Ok(()))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

- [ ] **Step 8: 커밋한다**

Run:

```powershell
git add src-tauri
git commit -m "feat: add SQLite schema and default rules"
```

Expected: database commit is created.

## Task 5: 이벤트 수집과 집계 커맨드

**Files:**
- Modify: `src-tauri/src/db.rs`
- Create: `src-tauri/src/commands.rs`
- Modify: `src-tauri/src/main.rs`

- [ ] **Step 1: 이벤트 저장 테스트를 작성한다**

Append to `src-tauri/src/db.rs` tests:

```rust
#[test]
fn stores_usage_event() {
    let conn = Connection::open_in_memory().unwrap();
    init_db(&conn).unwrap();

    insert_usage_event(
        &conn,
        "https://youtube.com/watch?v=abc",
        "youtube.com",
        "Video",
        "chrome",
        "active",
        "2026-05-12T08:00:00Z",
    )
    .unwrap();

    let count: i64 = conn
        .query_row("select count(*) from usage_events", [], |row| row.get(0))
        .unwrap();

    assert_eq!(count, 1);
}
```

- [ ] **Step 2: 테스트 실패를 확인한다**

Run:

```powershell
cd src-tauri
cargo test db::tests::stores_usage_event
```

Expected: FAIL because `insert_usage_event` is missing.

- [ ] **Step 3: 이벤트 저장 함수를 구현한다**

Add to `src-tauri/src/db.rs`:

```rust
pub fn insert_usage_event(
    conn: &Connection,
    url: &str,
    domain: &str,
    title: &str,
    browser: &str,
    event_type: &str,
    timestamp: &str,
) -> Result<()> {
    conn.execute(
        "
        insert into usage_events
            (url, domain, title, browser, event_type, timestamp)
        values
            (?1, ?2, ?3, ?4, ?5, ?6)
        ",
        params![url, domain, title, browser, event_type, timestamp],
    )?;

    Ok(())
}
```

- [ ] **Step 4: 테스트 통과를 확인한다**

Run:

```powershell
cd src-tauri
cargo test db::tests::stores_usage_event
```

Expected: PASS.

- [ ] **Step 5: Tauri 커맨드를 작성한다**

Create `src-tauri/src/commands.rs`:

```rust
use rusqlite::Connection;
use tauri::State;
use std::sync::Mutex;

use crate::db::insert_usage_event;
use crate::models::UsageEvent;

pub struct AppState {
    pub conn: Mutex<Connection>,
}

#[tauri::command]
pub fn ingest_usage_event(event: UsageEvent, state: State<AppState>) -> Result<(), String> {
    if event.domain.trim().is_empty() || event.timestamp.trim().is_empty() {
        return Err("domain and timestamp are required".to_string());
    }

    let conn = state.conn.lock().map_err(|_| "database lock failed".to_string())?;
    insert_usage_event(
        &conn,
        &event.url,
        &event.domain,
        &event.title,
        &event.browser,
        &event.event_type,
        &event.timestamp,
    )
    .map_err(|error| error.to_string())
}
```

- [ ] **Step 6: 커맨드를 Tauri 앱에 연결한다**

Modify `src-tauri/src/main.rs`:

```rust
mod commands;
mod db;
mod models;

use commands::{ingest_usage_event, AppState};
use rusqlite::Connection;
use std::sync::Mutex;

fn main() {
    let conn = Connection::open("time_manager.sqlite3").expect("failed to open database");
    db::init_db(&conn).expect("failed to initialize database");

    tauri::Builder::default()
        .manage(AppState {
            conn: Mutex::new(conn),
        })
        .invoke_handler(tauri::generate_handler![ingest_usage_event])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

- [ ] **Step 7: Rust 테스트 전체를 실행한다**

Run:

```powershell
cd src-tauri
cargo test
```

Expected: PASS.

- [ ] **Step 8: 커밋한다**

Run:

```powershell
git add src-tauri
git commit -m "feat: ingest usage events"
```

Expected: ingestion commit is created.

## Task 6: Chromium 확장 프로그램 MVP

**Files:**
- Create: `extension/manifest.json`
- Create: `extension/src/background.ts`
- Create: `extension/src/background.test.ts`
- Create: `extension/tsconfig.json`
- Modify: `package.json`

- [ ] **Step 1: 확장 빌드 의존성을 설치한다**

Run:

```powershell
npm install -D tsup @types/chrome
```

Expected: dependencies are installed.

- [ ] **Step 2: 확장 매니페스트를 작성한다**

Create `extension/manifest.json`:

```json
{
  "manifest_version": 3,
  "name": "Superpower Time Manager Tracker",
  "version": "0.1.0",
  "description": "Tracks active browser tab usage for the local desktop app.",
  "permissions": ["tabs", "activeTab", "idle"],
  "host_permissions": ["http://127.0.0.1:51247/*"],
  "background": {
    "service_worker": "dist/background.js"
  }
}
```

- [ ] **Step 3: 이벤트 payload 테스트를 작성한다**

Create `extension/src/background.test.ts`:

```ts
import { describe, expect, it } from "vitest";
import { buildUsageEvent } from "./background";

describe("buildUsageEvent", () => {
  it("normalizes tab data into a usage event", () => {
    const event = buildUsageEvent({
      url: "https://www.instagram.com/reels/",
      title: "Instagram",
    });

    expect(event.domain).toBe("instagram.com");
    expect(event.browser).toBe("unknown");
    expect(event.eventType).toBe("active");
    expect(event.title).toBe("Instagram");
  });
});
```

- [ ] **Step 4: 테스트 실패를 확인한다**

Run:

```powershell
npm test -- extension/src/background.test.ts
```

Expected: FAIL because `extension/src/background.ts` does not exist.

- [ ] **Step 5: background service worker를 구현한다**

Create `extension/src/background.ts`:

```ts
const INGEST_URL = "http://127.0.0.1:51247/usage-events";

export interface MinimalTab {
  url?: string;
  title?: string;
}

export function normalizeDomain(url: string): string {
  try {
    const hostname = new URL(url).hostname.toLowerCase();
    return hostname.startsWith("www.") ? hostname.slice(4) : hostname;
  } catch {
    return "";
  }
}

export function buildUsageEvent(tab: MinimalTab) {
  const url = tab.url ?? "";

  return {
    url,
    domain: normalizeDomain(url),
    title: tab.title ?? "",
    browser: "unknown",
    eventType: "active",
    timestamp: new Date().toISOString(),
  };
}

async function sendActiveTab(tab: chrome.tabs.Tab) {
  const event = buildUsageEvent(tab);
  if (!event.domain) return;

  try {
    await fetch(INGEST_URL, {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify(event),
    });
  } catch {
    // The desktop app may be closed. MVP retries on the next browser event.
  }
}

chrome.tabs.onActivated.addListener(async ({ tabId }) => {
  const tab = await chrome.tabs.get(tabId);
  await sendActiveTab(tab);
});

chrome.tabs.onUpdated.addListener(async (_tabId, changeInfo, tab) => {
  if (changeInfo.status === "complete" && tab.active) {
    await sendActiveTab(tab);
  }
});
```

- [ ] **Step 6: 확장 테스트를 통과시킨다**

Run:

```powershell
npm test -- extension/src/background.test.ts
```

Expected: PASS.

- [ ] **Step 7: 확장 빌드 스크립트를 추가한다**

Modify `package.json` scripts:

```json
{
  "scripts": {
    "extension:build": "tsup extension/src/background.ts --format iife --global-name TimeManagerExtension --outfile extension/dist/background.js"
  }
}
```

- [ ] **Step 8: 확장 빌드를 확인한다**

Run:

```powershell
npm run extension:build
```

Expected: `extension/dist/background.js` is created.

- [ ] **Step 9: 커밋한다**

Run:

```powershell
git add package.json package-lock.json extension
git commit -m "feat: add Chromium tracking extension"
```

Expected: extension commit is created.

## Task 7: 로컬 HTTP 수집 API

**Files:**
- Modify: `src-tauri/Cargo.toml`
- Create: `src-tauri/src/http.rs`
- Modify: `src-tauri/src/main.rs`

- [ ] **Step 1: HTTP 서버 의존성을 추가한다**

Modify `src-tauri/Cargo.toml` dependencies:

```toml
tiny_http = "0.12"
```

- [ ] **Step 2: HTTP 요청 처리 테스트를 작성한다**

Create `src-tauri/src/http.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_only_usage_events_path() {
        assert!(is_usage_events_path("/usage-events"));
        assert!(!is_usage_events_path("/"));
    }
}
```

- [ ] **Step 3: 테스트 실패를 확인한다**

Run:

```powershell
cd src-tauri
cargo test http::tests::accepts_only_usage_events_path
```

Expected: FAIL because `is_usage_events_path` is missing.

- [ ] **Step 4: 최소 HTTP 유틸리티를 구현한다**

Replace `src-tauri/src/http.rs`:

```rust
use crate::db::insert_usage_event;
use crate::models::UsageEvent;
use rusqlite::Connection;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::thread;
use tiny_http::{Header, Method, Response, Server};

pub fn is_usage_events_path(path: &str) -> bool {
    path == "/usage-events"
}

pub fn start_ingest_server(conn: Arc<Mutex<Connection>>) {
    thread::spawn(move || {
        let server = Server::http("127.0.0.1:51247").expect("failed to bind ingest server");

        for mut request in server.incoming_requests() {
            let cors = Header::from_bytes("Access-Control-Allow-Origin", "*").unwrap();

            if request.method() == &Method::Options {
                let response = Response::empty(204).with_header(cors);
                let _ = request.respond(response);
                continue;
            }

            if request.method() != &Method::Post || !is_usage_events_path(request.url()) {
                let response = Response::from_string("not found").with_status_code(404).with_header(cors);
                let _ = request.respond(response);
                continue;
            }

            let mut body = String::new();
            if request.as_reader().read_to_string(&mut body).is_err() {
                let response = Response::from_string("bad request").with_status_code(400).with_header(cors);
                let _ = request.respond(response);
                continue;
            }

            let parsed: Result<UsageEvent, _> = serde_json::from_str(&body);
            let result = parsed.and_then(|event| {
                let conn = conn.lock().map_err(|_| serde_json::Error::io(std::io::Error::other("lock failed")))?;
                insert_usage_event(
                    &conn,
                    &event.url,
                    &event.domain,
                    &event.title,
                    &event.browser,
                    &event.event_type,
                    &event.timestamp,
                )
                .map_err(|error| serde_json::Error::io(std::io::Error::other(error.to_string())))
            });

            let response = match result {
                Ok(()) => Response::from_string("ok").with_status_code(202).with_header(cors),
                Err(_) => Response::from_string("bad request").with_status_code(400).with_header(cors),
            };
            let _ = request.respond(response);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_only_usage_events_path() {
        assert!(is_usage_events_path("/usage-events"));
        assert!(!is_usage_events_path("/"));
    }
}
```

- [ ] **Step 5: HTTP 모듈을 앱에 연결한다**

Modify `src-tauri/src/main.rs` to include:

```rust
mod http;
```

Use shared connection:

```rust
use std::sync::{Arc, Mutex};

let conn = Arc::new(Mutex::new(Connection::open("time_manager.sqlite3").expect("failed to open database")));
{
    let locked = conn.lock().expect("failed to lock database");
    db::init_db(&locked).expect("failed to initialize database");
}
http::start_ingest_server(conn.clone());
```

Adjust `AppState` in `commands.rs` to store `Arc<Mutex<Connection>>` instead of `Mutex<Connection>`.

- [ ] **Step 6: Rust 테스트를 실행한다**

Run:

```powershell
cd src-tauri
cargo test
```

Expected: PASS.

- [ ] **Step 7: 커밋한다**

Run:

```powershell
git add src-tauri
git commit -m "feat: add local HTTP ingestion endpoint"
```

Expected: local HTTP commit is created.

## Task 8: 대시보드 집계 UI

**Files:**
- Create: `src/shared/format.ts`
- Create: `src/shared/format.test.ts`
- Create: `src/components/SummaryCards.tsx`
- Create: `src/components/SiteTable.tsx`
- Modify: `src/App.tsx`
- Modify: `src/styles.css`
- Modify: `src-tauri/src/commands.rs`

- [ ] **Step 1: 시간 포맷 테스트를 작성한다**

Create `src/shared/format.test.ts`:

```ts
import { describe, expect, it } from "vitest";
import { formatDuration } from "./format";

describe("formatDuration", () => {
  it("formats minutes under one hour", () => {
    expect(formatDuration(25 * 60)).toBe("25m");
  });

  it("formats hours and minutes", () => {
    expect(formatDuration(90 * 60)).toBe("1h 30m");
  });
});
```

- [ ] **Step 2: 테스트 실패를 확인한다**

Run:

```powershell
npm test -- src/shared/format.test.ts
```

Expected: FAIL because `formatDuration` is missing.

- [ ] **Step 3: 시간 포맷을 구현한다**

Create `src/shared/format.ts`:

```ts
export function formatDuration(seconds: number): string {
  const totalMinutes = Math.max(0, Math.floor(seconds / 60));
  const hours = Math.floor(totalMinutes / 60);
  const minutes = totalMinutes % 60;

  if (hours === 0) return `${minutes}m`;
  if (minutes === 0) return `${hours}h`;
  return `${hours}h ${minutes}m`;
}
```

- [ ] **Step 4: UI 컴포넌트를 작성한다**

Create `src/components/SummaryCards.tsx`:

```tsx
import { formatDuration } from "../shared/format";

interface SummaryCardsProps {
  productiveSeconds: number;
  unproductiveSeconds: number;
  neutralSeconds: number;
}

export function SummaryCards(props: SummaryCardsProps) {
  return (
    <section className="summary-grid" aria-label="Today summary">
      <article>
        <span>Productive</span>
        <strong>{formatDuration(props.productiveSeconds)}</strong>
      </article>
      <article>
        <span>Unproductive</span>
        <strong>{formatDuration(props.unproductiveSeconds)}</strong>
      </article>
      <article>
        <span>Neutral</span>
        <strong>{formatDuration(props.neutralSeconds)}</strong>
      </article>
    </section>
  );
}
```

Create `src/components/SiteTable.tsx`:

```tsx
import { formatDuration } from "../shared/format";
import type { Classification } from "../shared/types";

export interface SiteUsageRow {
  domain: string;
  classification: Classification;
  seconds: number;
}

export function SiteTable({ rows }: { rows: SiteUsageRow[] }) {
  return (
    <section className="panel">
      <h2>Sites</h2>
      <table>
        <thead>
          <tr>
            <th>Domain</th>
            <th>Class</th>
            <th>Time</th>
          </tr>
        </thead>
        <tbody>
          {rows.map((row) => (
            <tr key={row.domain}>
              <td>{row.domain}</td>
              <td>{row.classification}</td>
              <td>{formatDuration(row.seconds)}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </section>
  );
}
```

- [ ] **Step 5: 앱에 mock 집계 데이터를 연결한다**

Modify `src/App.tsx`:

```tsx
import { SummaryCards } from "./components/SummaryCards";
import { SiteTable } from "./components/SiteTable";
import "./styles.css";

const rows = [
  { domain: "chatgpt.com", classification: "productive" as const, seconds: 0 },
  { domain: "youtube.com", classification: "unproductive" as const, seconds: 0 },
  { domain: "example.com", classification: "neutral" as const, seconds: 0 },
];

export function App() {
  return (
    <main className="app-shell">
      <header className="app-header">
        <div>
          <p className="eyebrow">Windows MVP</p>
          <h1>Superpower Time Manager</h1>
        </div>
      </header>
      <SummaryCards productiveSeconds={0} unproductiveSeconds={0} neutralSeconds={0} />
      <SiteTable rows={rows} />
    </main>
  );
}

export default App;
```

- [ ] **Step 6: 테스트와 빌드를 확인한다**

Run:

```powershell
npm test
npm run build
```

Expected: both commands pass.

- [ ] **Step 7: 커밋한다**

Run:

```powershell
git add src package.json package-lock.json
git commit -m "feat: add dashboard UI components"
```

Expected: dashboard commit is created.

## Task 9: 규칙 편집 MVP

**Files:**
- Create: `src/components/RulesPanel.tsx`
- Modify: `src/App.tsx`
- Modify: `src-tauri/src/db.rs`
- Modify: `src-tauri/src/commands.rs`

- [ ] **Step 1: Rust 규칙 조회 테스트를 작성한다**

Append to `src-tauri/src/db.rs` tests:

```rust
#[test]
fn lists_domain_rules() {
    let conn = Connection::open_in_memory().unwrap();
    init_db(&conn).unwrap();

    let rules = list_domain_rules(&conn).unwrap();

    assert!(rules.iter().any(|rule| rule.domain == "chatgpt.com"));
    assert!(rules.iter().any(|rule| rule.domain == "youtube.com"));
}
```

- [ ] **Step 2: 테스트 실패를 확인한다**

Run:

```powershell
cd src-tauri
cargo test db::tests::lists_domain_rules
```

Expected: FAIL because `list_domain_rules` is missing.

- [ ] **Step 3: 규칙 모델과 조회 함수를 구현한다**

Add to `src-tauri/src/models.rs`:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DomainRule {
    pub domain: String,
    pub classification: String,
}
```

Add to `src-tauri/src/db.rs`:

```rust
use crate::models::DomainRule;

pub fn list_domain_rules(conn: &Connection) -> Result<Vec<DomainRule>> {
    let mut statement = conn.prepare(
        "select domain, classification from domain_rules order by domain asc",
    )?;
    let rows = statement.query_map([], |row| {
        Ok(DomainRule {
            domain: row.get(0)?,
            classification: row.get(1)?,
        })
    })?;

    rows.collect()
}
```

- [ ] **Step 4: 규칙 커맨드를 추가한다**

Add to `src-tauri/src/commands.rs`:

```rust
use crate::db::list_domain_rules;
use crate::models::DomainRule;

#[tauri::command]
pub fn get_domain_rules(state: State<AppState>) -> Result<Vec<DomainRule>, String> {
    let conn = state.conn.lock().map_err(|_| "database lock failed".to_string())?;
    list_domain_rules(&conn).map_err(|error| error.to_string())
}
```

Add `get_domain_rules` to `tauri::generate_handler!`.

- [ ] **Step 5: 규칙 패널 UI를 작성한다**

Create `src/components/RulesPanel.tsx`:

```tsx
import type { DomainRule } from "../shared/types";

export function RulesPanel({ rules }: { rules: DomainRule[] }) {
  return (
    <section className="panel">
      <h2>Rules</h2>
      <ul className="rule-list">
        {rules.map((rule) => (
          <li key={rule.domain}>
            <span>{rule.domain}</span>
            <strong>{rule.classification}</strong>
          </li>
        ))}
      </ul>
    </section>
  );
}
```

- [ ] **Step 6: 앱에 규칙 패널 mock 데이터를 연결한다**

Modify `src/App.tsx` to render:

```tsx
<RulesPanel
  rules={[
    { domain: "chatgpt.com", classification: "productive" },
    { domain: "youtube.com", classification: "unproductive" },
  ]}
/>
```

- [ ] **Step 7: 테스트를 확인한다**

Run:

```powershell
cd src-tauri
cargo test
cd ..
npm test
npm run build
```

Expected: all pass.

- [ ] **Step 8: 커밋한다**

Run:

```powershell
git add src src-tauri
git commit -m "feat: show domain rules"
```

Expected: rules commit is created.

## Task 10: 수동 검증 문서와 GitHub 업로드

**Files:**
- Create: `docs/manual-test.md`
- Modify: `README.md`

- [ ] **Step 1: 수동 검증 문서를 작성한다**

Create `docs/manual-test.md`:

```markdown
# Manual Test

## Desktop App

1. Run `npm run tauri dev`.
2. Confirm the dashboard window opens.
3. Confirm the summary cards show Productive, Unproductive, and Neutral.
4. Confirm the Sites and Rules sections render without overlapping text.

## Chrome Extension

1. Run `npm run extension:build`.
2. Open Chrome `chrome://extensions`.
3. Enable Developer mode.
4. Load unpacked extension from `extension`.
5. Open `https://chatgpt.com`, `https://youtube.com`, and `https://instagram.com`.
6. Confirm the desktop database receives usage events.

## Edge Extension

1. Open Edge `edge://extensions`.
2. Enable Developer mode.
3. Load unpacked extension from `extension`.
4. Repeat the Chrome site switching checks.
```

- [ ] **Step 2: README 실행 방법을 갱신한다**

Append to `README.md`:

```markdown
## Development

```powershell
npm install
npm run tauri dev
```

Build the browser extension:

```powershell
npm run extension:build
```

Manual verification steps are in `docs/manual-test.md`.
```

- [ ] **Step 3: 전체 검증을 실행한다**

Run:

```powershell
npm test
npm run build
npm run extension:build
cd src-tauri
cargo test
```

Expected: all commands pass.

- [ ] **Step 4: 문서를 커밋한다**

Run:

```powershell
git add README.md docs/manual-test.md
git commit -m "docs: add manual test instructions"
```

Expected: documentation commit is created.

- [ ] **Step 5: GitHub public repository를 만든다**

Preferred repo name:

```text
superpower-time-manager-study
```

If GitHub CLI is available and authenticated, run:

```powershell
gh repo create sungwoocse/superpower-time-manager-study --public --source . --remote origin --push
```

Expected: public repository is created under `sungwoocse`, remote `origin` is set, and local commits are pushed.

If `gh` is not available, install and authenticate it before this step or create the repository manually on GitHub, then run:

```powershell
git remote add origin https://github.com/sungwoocse/superpower-time-manager-study.git
git branch -M main
git push -u origin main
```

Expected: repository is visible at `https://github.com/sungwoocse/superpower-time-manager-study`.

## Self-Review

Spec coverage:

- Browser extension active tab tracking: Task 6.
- Windows desktop app with Tauri/React: Task 2.
- SQLite local storage and default rules: Task 4.
- Event ingestion: Tasks 5 and 7.
- Dashboard, site list, and rules view: Tasks 8 and 9.
- Chrome/Edge manual verification: Task 10.
- Excluded features remain excluded: no task implements blocking, notifications, cloud sync, accounts, macOS, or desktop-wide app tracking.

Type consistency:

- TypeScript uses `eventType`; Rust uses `event_type` with camelCase serde mapping.
- Classifications use `productive`, `unproductive`, and `neutral`.
- Domain rules use `domain` and `classification` across TypeScript, Rust, and SQLite.

Execution choice after this plan:

1. Subagent-Driven: fresh subagent per task with spec and code-quality reviews.
2. Inline Execution: execute tasks in this session with checkpoints.
