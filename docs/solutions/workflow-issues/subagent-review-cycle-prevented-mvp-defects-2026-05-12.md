---
title: Subagent review cycle prevented MVP defects
date: 2026-05-12
category: workflow-issues
module: Time Manager MVP implementation workflow
problem_type: workflow_issue
component: development_workflow
severity: medium
applies_when:
  - "Executing a multi-task implementation plan with worker, spec, and quality review agents"
  - "Review findings reveal defects after task commits but before final shipping"
  - "Security, UI quality, or documentation accuracy issues need to be captured as process learnings"
related_components:
  - security
  - documentation
  - testing_framework
tags:
  - subagent-review
  - implementation-review
  - security-review
  - documentation-review
  - ui-quality
---

# Subagent review cycle prevented MVP defects

## Context
The Time Manager MVP implementation used a per-task worker, spec-review, and quality-review loop on branch `feat/time-manager-mvp`. The loop found issues that functional implementation alone did not catch: local API security gaps, a runtime startup failure mode, a UI consistency violation, and documentation commands that did not work from the documented location.

The strongest lesson is that MVP scope does not remove the need for clear local security boundaries, observable runtime failure states, accurate docs, and concrete UI quality gates.

## Guidance
Run task-level reviews even after an implementation commit appears complete. Give each reviewer a narrow remit:

- Spec reviewers check whether the planned behavior exists and whether unrelated scope was added.
- Quality reviewers check whether the implementation is safe, maintainable, and practical.
- UI reviewers apply concrete visual constraints such as text overflow, card nesting, and radius gates.
- Documentation reviewers run commands from the documented working directory and look for overclaims.

Treat local MVP boundaries as real boundaries. The Task 7 quality review correctly blocked a committed shared ingest token and broad `chrome-extension://` CORS. The fix used a per-install token, exact companion extension origin, and `/config` handshake instead of a static Rust/TypeScript token.

Expose runtime service failures as app state when diagnosis belongs in the UI. A localhost ingest port bind failure should not abort Tauri setup before the app can report what happened. Recording the failure in `app_status` preserved startup and made the problem observable.

Keep documentation review separate from implementation review. The Task 10 doc pass caught that Rust verification commands were written as worker-local absolute paths and failed from the repo root. It also caught an overclaim that the UI showed local statistics when the current dashboard still uses static sample data.

## Why This Matters
Local desktop apps still have attack surfaces: browser extensions, localhost HTTP ports, CORS, and committed tokens. A token checked into source control is not a secret, and a broad extension-origin CORS rule lets unrelated extensions reach the local service.

Runtime crashes during setup hide actionable state from the user and make manual verification brittle. Capturing service failure as app status keeps the shell diagnosable.

Docs that only work from a worker's current directory waste future time. A verification command must work from the location where the documentation tells the reader to run it.

The narrow review loop found different classes of defects:

- Security review found local boundary flaws.
- Frontend review found a design consistency issue in `src/styles.css`.
- Documentation review found verification drift and overclaims.
- Full verification proved the integrated result after each fix.

## When to Apply
- Building local desktop apps that expose localhost APIs.
- Pairing a browser extension with a native or desktop app.
- Using "temporary" MVP tokens, CORS rules, or dev-only boundaries.
- Starting background services during Tauri setup.
- Writing README or manual test instructions intended to run from repo root.
- Shipping UI work where visual consistency is part of acceptance.
- Running agent-driven implementation where each reviewer can own a narrow remit.

## Examples
Before the Task 7 review, the local ingest boundary had the wrong shape:

```rust
const INGEST_TOKEN: &str = "superpower-time-manager-dev-token";

fn allowed_cors_origin(origin: Option<&str>) -> Option<&str> {
    origin.filter(|origin| origin.starts_with("chrome-extension://"))
}
```

After review, the server used an exact companion origin and a per-install token loaded from app data:

```rust
pub const COMPANION_EXTENSION_ORIGIN: &str =
    "chrome-extension://okcchcpgcebnenmenmfjbpnnkjoepien";

fn allowed_cors_origin(origin: Option<&str>) -> Option<&str> {
    origin.filter(|origin| *origin == COMPANION_EXTENSION_ORIGIN)
}
```

Before the runtime review, a bind error could abort setup:

```rust
http::start_ingest_server(conn.clone(), ingest_token)?;
```

After review, the app recorded the failure and exposed status:

```rust
if let Err(error) = http::start_ingest_server(conn.clone(), ingest_token) {
    log::error!("{error}");
    if let Ok(mut current_error) = ingest_server_error.lock() {
        *current_error = Some(error);
    }
}
```

The verification set used for the final cycle was:

```powershell
npm test
npm run build
npm run extension:build
cargo test --manifest-path src-tauri/Cargo.toml
cargo check --manifest-path src-tauri/Cargo.toml
```

## Related
- [Local ingest token and CORS boundary](../security-issues/local-ingest-token-cors-boundary-2026-05-12.md)
- `46acbe3` `fix: harden local ingest handshake`
- `a43dfdc` `fix: align dashboard badge radius`
- `4f77cb9` `docs: correct verification instructions`
