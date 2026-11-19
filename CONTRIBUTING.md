# Contributing

Thanks for your interest in Originium Stream.

## Development workflow

1. Fork the repository and create a topic branch from `main`.
2. Install the toolchain described in [docs/building.md](./docs/building.md).
3. Run the full local gate before opening a pull request:

```powershell
pnpm --filter @originium/desktop typecheck
pnpm --filter @originium/desktop test
pnpm --filter @originium/desktop lint
cargo fmt --manifest-path apps/desktop/src-tauri/Cargo.toml --check
cargo clippy --manifest-path apps/desktop/src-tauri/Cargo.toml -- -D warnings
cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml
```

4. Keep commits focused and use [Conventional Commits](https://www.conventionalcommits.org/).

## Upstream policy

This project **never** pushes to or opens pull requests against the official
Sunshine or moonlight-qt repositories. Component forks are read-only mirrors.
All patches live on our own `originium` branches.

## Licensing

By contributing you agree that your changes are licensed under GPL-3.0-or-later.
