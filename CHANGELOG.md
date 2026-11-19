# Changelog

All notable changes to this project are documented here.

## [0.1.4] - 2026-11-20

### Added

- File-based theme import/export (`theme.json` picker) with size and schema validation; theme assets are restricted to the managed `fonts/` and `backgrounds/` directories.
- Sunshine service uninstall action on the local host and settings pages.
- Start-minimized support that reliably hides the main window after Tauri finishes creating it.
- Component install/uninstall guards for running Sunshine services and active Moonlight sessions, plus executable-name validation for custom component paths.

### Fixed

- mDNS discovery now maps Sunshine's advertised GameStream port (default 47989) to the HTTPS API port (default 47990); previously discovered hosts could not connect. Existing mDNS host entries are migrated on startup.
- Host detail, app cover, and host-log state no longer leak between hosts when switching routes or tabs.
- Clearing application logs now truncates the log file as well as the in-memory buffer; log rotation no longer deadlocks on Windows.
- Component installation rolls back the previous bundle if replacing it fails.
- Host updates preserve or clear stored credentials correctly, reset certificate trust after address/port changes, and reject empty names, invalid ports, and duplicates.
- Global hotkey changes no longer unregister the previous shortcut before validating the new one.
- `startMinimized`, `updateChannel`, and `preferServiceMode` UI inconsistencies were resolved; unavailable updater/service-mode options were removed from the settings UI until implemented.
- Theme JSON import no longer depends on WebView2 clipboard permissions.
- Config editor no longer mutates host config in place and blocks saving invalid JSON fields.
- Missing settings fields from older configs now deserialize with defaults instead of resetting the entire configuration.
- Local page service labels, LAN URLs, and copy actions are now localized and use the configured host port.
- Additional UI strings are localized in English and Chinese, including host forms, permissions, virtual input, theme colors, and profile options.
- LAN address fallback no longer invokes a Linux-only `hostname -I` command.
- Removed the stale hard-coded GitHub link and corrected the Moonlight installation status display.

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
