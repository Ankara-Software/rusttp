<div align="center">

<h1>rusttp</h1>

<p><strong>A fast, user-friendly HTTP client for the terminal — httpie, rewritten in Rust.</strong></p>

[![Crates.io](https://img.shields.io/crates/v/rusttp-cli?style=flat-square&color=fc8d62)](https://crates.io/crates/rusttp-cli)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE)
[![Build](https://img.shields.io/github/actions/workflow/status/anilcan-kara/rusttp/release.yml?style=flat-square)](https://github.com/anilcan-kara/rusttp/actions)
[![GitHub Release](https://img.shields.io/github/v/release/anilcan-kara/rusttp?style=flat-square&color=8be04e)](https://github.com/anilcan-kara/rusttp/releases)

```
rusttp POST https://httpbin.org/post name="Anilcan Kara" age:=28 X-Token:secret
```

</div>

---

## Why rusttp?

`httpie` is great — but it's Python. That means slow startup, a heavy runtime dependency, and noticeable latency on every invocation. **rusttp** gives you the same human-friendly syntax with the performance of a native binary.

| | rusttp REX 🦀 | httpie 🐍 |
|---|---|---|
| Startup time | ~2ms | ~300ms |
| Runtime dependency | None | Python 3 + pip |
| Binary size | ~4MB (static) | N/A |
| Syntax | Identical | ✓ |
| Syntax highlighting | ✓ | ✓ |
| JSON support | ✓ | ✓ |
| File uploads | ✓ | ✓ |
| Offline mode | ✓ | ✗ |

---

## Installation

### ⚡ Quick Install (Linux / macOS)
```bash
curl -fsSL https://raw.githubusercontent.com/anilcan-kara/rusttp/master/install.sh | sh
```

### Cargo (from source)
```bash
cargo install --git https://github.com/anilcan-kara/rusttp.git
```

### Pre-built Binaries

Download a static binary directly — no dependencies, just run it:

| Platform | Download |
|---|---|
| 🪟 Windows (x64) | [rusttp-win32-x64.exe](https://github.com/anilcan-kara/rusttp/releases/download/v0.1.1/rusttp-win32-x64.exe) |
| 🐧 Linux (x64) | [rusttp-linux-x64](https://github.com/anilcan-kara/rusttp/releases/download/v0.1.1/rusttp-linux-x64) |
| 🐧 Linux (ARM64) | [rusttp-linux-arm64](https://github.com/anilcan-kara/rusttp/releases/download/v0.1.1/rusttp-linux-arm64) |
| 🍎 macOS (x64) | [rusttp-darwin-x64](https://github.com/anilcan-kara/rusttp/releases/download/v0.1.1/rusttp-darwin-x64) |
| 🍎 macOS (ARM64/M-series) | [rusttp-darwin-arm64](https://github.com/anilcan-kara/rusttp/releases/download/v0.1.1/rusttp-darwin-arm64) |

---

## Usage

### Simple GET
```bash
rusttp GET https://httpbin.org/get
rusttp https://httpbin.org/get          # GET is the default
```

### POST with JSON Body
```bash
rusttp POST https://httpbin.org/post name="Anilcan Kara" age:=28 active:=true
```

### Offline Mode (Preview Request)
```bash
rusttp --offline POST https://httpbin.org/post name=Anilcan
```

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). All PRs welcome.

---

## License

MIT © [Anilcan Kara](https://github.com/anilcan-kara)
