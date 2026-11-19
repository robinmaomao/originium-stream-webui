# Changelog

All notable changes to this project are documented here.

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
