# Changelog

All notable changes to this project are documented here.

## [0.1.3] - 2026-11-19

### Added

- Adaptive main-window sizing against the current monitor work area, including Windows DPI scaling and taskbar-safe centering.
- `autoStartServices` setting (enabled by default) to start the Sunshine service when the app launches; UAC is only requested when the service is stopped.
- Responsive title bar and icon-rail navigation for narrow or high-scaling displays.

### Changed

- Scrollbars are hidden throughout the UI while wheel, touch and keyboard scrolling remain available.
- Onboarding and page layouts use fluid spacing and remain scrollable without clipped content on short work areas.

## [0.1.2] - 2026-11-19

### Fixed

- Fixed the fixed navigation drawer covering the left side of page content.
- Fixed Vuetify theme variables being cleared when switching themes, which produced white empty buttons and invisible labels.
- Suppressed console windows from service detection, environment probes, pairing, and session termination on Windows.
- Allowed Vuetify's runtime theme stylesheet under Tauri's production CSP, so themed controls no longer render as blank white blocks.

### Added

- Integrated custom title bar with brand, page title, session status, theme toggle, and window controls.
- Optional native acrylic background with a CSS glass fallback.
- Dark/light theme switch in both the title bar and Settings.

## [0.1.1] - 2026-11-19

### Added

- Originium Sunshine fork component with `GET /api/session` live session events.
- Originium Moonlight fork component with a framed JSON control pipe (`session.state`, `session.stop`).
- Live session status card on the local host page, including the running application name.
- Graceful session shutdown through the control pipe before process termination.

### Changed

- `components.lock.json` now pins the Originium Sunshine and Moonlight fork releases with SHA-256 hashes and capability flags.
- Moonlight packages ship an active `portable.dat`, so the settings editor and launcher share the same INI file.

## [0.1.0] - 2026-11-19

### Added

- Unified Tauri 2 + Vue 3 + Vuetify 3 desktop shell.
- Component manager with manifest, SHA-256 verification, safe extraction, local overrides and rollback.
- Onboarding wizard (roles, Sunshine credentials, firewall, service install, theme).
- Dual-stack runtime: Sunshine service/process supervision, Moonlight session process management.
- LAN discovery (`_nvstream._tcp`) and automated PIN pairing.
- Sunshine REST client with Basic auth, TOFU certificate pinning, 302/401/403 mapping and CSRF-aware design.
- Complete Sunshine settings editor and Moonlight portable INI editor.
- Customizable theme engine: color tokens, fonts, backgrounds, presets, import/export.
- Session HUD, global hotkey, tray integration, autostart.
- Structured logging, diagnostics export, unit tests and GitHub Actions CI.
