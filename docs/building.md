# Building

## Prerequisites (Windows)

| Component | Version | Notes |
|---|---|---|
| Windows | 10 1809+ / 11 | WebView2 runtime required |
| Node.js | 24+ | Native modules used by the frontend toolchain |
| pnpm | 10.34+ | `corepack enable` or `npm i -g pnpm` |
| Rust | 1.100+ | `stable-x86_64-pc-windows-msvc` |
| Visual Studio Build Tools | 2022 | MSVC, Windows SDK, CMake |
| WebView2 | 157+ | Preinstalled on current Windows builds |

## Commands

```powershell
# install frontend dependencies
pnpm install

# development (Tauri + Vite HMR)
pnpm tauri dev

# typecheck + unit tests + build
pnpm --filter @originium/desktop typecheck
pnpm --filter @originium/desktop test
pnpm build

# Rust checks
cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml --check
cargo clippy --manifest-path apps/desktop/src-tauri/Cargo.toml -- -D warnings
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml

# packaging (NSIS + MSI)
pnpm tauri build
```

Artifacts: `apps/desktop/src-tauri/target/release/bundle/`.

## Environment notes

- `cargo` is invoked with the MSVC environment (`vcvars64.bat`) so build scripts
  and linking succeed. CI uses the same approach on `windows-latest`.
- The Tauri config enables `protocol-asset`; the Rust `tauri` dependency must keep
  the `protocol-asset` feature or the build script will fail.
- Icons are generated from `apps/desktop/src-tauri/icons/source.png` via
  `pnpm --filter @originium/desktop exec tauri icon src-tauri/icons/source.png -o src-tauri/icons`.
- `components.lock.json` is embedded at compile time; if it is missing or invalid
  the build fails. A user override can be placed at
  `%APPDATA%/dev.originium.stream/components.lock.json`.

## Component forks

See [components.md](./components.md). Forks are built in their own repositories;
the application consumes ZIP artifacts and verifies SHA-256 at install time.
