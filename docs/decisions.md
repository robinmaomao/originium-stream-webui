# Architecture decisions

## ADR-001 — Tauri 2 instead of Electron

**Status:** accepted

Smaller footprint, Rust-native process/service management, first-class Windows WebView2,
and a strict capability model. Electron was rejected due to installer size and
unnecessary Node runtime for privileged operations.

## ADR-002 — Orchestrate official binaries instead of reimplementing protocols

**Status:** accepted

Sunshine and moonlight-qt are GPL-3.0 and mature. Originium Stream supervises them,
which guarantees 100% protocol compatibility and keeps maintenance bounded.
A future native client (moonlight-common-c + WebCodecs) remains out of scope for v0.1.

## ADR-003 — Public repositories, GPL-3.0

**Status:** accepted

All repositories are public and GPL-3.0-or-later. Modifying and distributing Sunshine
or moonlight requires source availability; public repos keep compliance trivial.

## ADR-004 — Read-only relationship with upstream

**Status:** accepted

Originium Stream never pushes to, opens pull requests against, or otherwise modifies
`SunshineStream/Sunshine` or `moonlight-stream/moonlight-qt`. Component forks keep an
`upstream` remote **for fetch/rebase only**.

## ADR-005 — Manual typed API bindings instead of tauri-specta

**Status:** accepted (deviation from plan)

`tauri-specta` adds build-time code generation and an unstable API surface. The
project instead keeps a hand-maintained `src/api` module with explicit TypeScript
interfaces. This is checked by `vue-tsc` and unit tests.

## ADR-006 — Single `config.json`

**Status:** accepted (deviation from plan)

The plan sketched `config.json`, `hosts.json` and `theme.json`. A single atomic
`config.json` (plus a `theme.json` snapshot for portability) avoids partial-write
inconsistencies. Corruption falls back to defaults and preserves a `.corrupt.json` copy.

## ADR-007 — TOFU certificate pinning at the application layer

**Status:** accepted

Rustls captures the SHA-256 fingerprint during a raw TLS handshake; subsequent
connections compare the stored value. The HTTP client accepts the self-signed
certificate because Sunshine generates a certificate whose CN does not match the
host IP. Fingerprint verification is what prevents MITM.

## ADR-008 — HUD-first embedding strategy

**Status:** accepted

v0.1 uses borderless fullscreen plus an always-on-top HUD toggled by a global
hotkey. Native `SetParent` embedding is deferred to the fork capability
`embed-parent`; it is unstable across SDL window recreation.

## ADR-009 — Component capability negotiation

**Status:** accepted

`components.lock.json` lists capabilities (`control-pipe`, `embed-parent`,
`session-events`, `no-webui`). The app checks capabilities before using
fork-specific features and otherwise falls back to CLI/official behavior.

## ADR-010 — Undecorated acrylic window

**Status:** accepted

The main window is undecorated and transparent so the Vue shell can render a
fused title bar. Native Windows Acrylic is applied through Tauri's
`set_effects` command and is user-toggleable; CSS glass remains as a fallback.
The window keeps `WS_THICKFRAME` for native resize borders and uses explicit
no-drag regions around interactive title-bar controls.

## ADR-011 — CSP style nonce exception

**Status:** accepted

Vuetify injects its generated theme stylesheet at runtime, while Tauri injects a
CSP nonce into the asset `index.html`. Browsers ignore `'unsafe-inline'` when a
nonce is present, so Tauri's nonce injection for `style-src` is disabled via
`dangerousDisableAssetCspModification: ["style-src"]`. Script nonces remain
enabled; only inline style sources are relaxed to keep dynamic themes working.
