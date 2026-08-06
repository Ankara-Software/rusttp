# Contributing to rusttp

Thank you for considering a contribution! All PRs, bug reports, and feature suggestions are welcome.

## Setup

```bash
git clone https://github.com/anilcan-kara/rusttp.git
cd rusttp
cargo build
cargo test
```

**Requirements**: Rust stable (1.75+). Install via [rustup.rs](https://rustup.rs).

## Workflow

1. **Check existing issues** — make sure no one is already working on it
2. **Open an issue** first for significant changes so we can discuss design before you invest time
3. **Fork + branch** from `master`:
   ```bash
   git checkout -b feat/my-feature
   # or
   git checkout -b fix/bug-description
   ```
4. **Write code** — keep commits small and focused
5. **Run the checks** before pushing:
   ```bash
   cargo fmt
   cargo clippy -- -D warnings
   cargo test
   ```
6. **Open a PR** against `master` with a clear description of _what_ and _why_

## Code Style

- Format with `cargo fmt` (enforced by CI)
- Lint with `cargo clippy -- -D warnings` (enforced by CI)
- Follow standard Rust naming conventions (`snake_case` functions, `CamelCase` types)

## License

By contributing, you agree that your contributions will be licensed under the **MIT License**.
