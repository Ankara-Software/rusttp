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

| | rusttp 🦀 | httpie 🐍 |
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

## Features

- ⚡ **Blazingly fast** — native binary, zero Python overhead, instant startup
- 🎨 **Syntax highlighting** — automatic colorization for JSON, HTML, and XML responses
- 🧠 **Intuitive syntax** — `key=value`, `key:=json`, `key==query`, `header:value`, `key@file`
- 📦 **JSON first** — automatic serialization and pretty-printing of JSON payloads
- 📁 **File uploads** — multipart form data and file attachments with `key@file`
- 📥 **Download mode** — save large files with a real-time progress bar
- 🔌 **Offline mode** — preview and debug requests before sending
- 🔐 **Auth support** — Basic auth and Bearer token with `--auth`
- 🖨️ **Flexible output** — print headers, body, or full request+response with `-v`

---

## Installation

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

```bash
# Linux / macOS: make it executable
chmod +x rusttp-linux-x64
sudo mv rusttp-linux-x64 /usr/local/bin/rusttp
```

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
- `name="Anilcan Kara"` → JSON string
- `age:=28` → raw JSON number
- `active:=true` → raw JSON boolean

### Request Headers & Query Parameters
```bash
rusttp https://api.github.com/users/anilcan-kara \
  Authorization:"Bearer TOKEN" \
  page==2 per_page==50
```
- `Authorization:"Bearer TOKEN"` → HTTP header
- `page==2` → URL query parameter (`?page=2`)

### Authentication
```bash
# Basic auth
rusttp -a user:password https://httpbin.org/basic-auth/user/password

# Bearer token
rusttp -a mytoken --auth-type bearer https://api.example.com/data
```

### File Upload
```bash
rusttp --form POST https://httpbin.org/post \
  file@./photo.jpg \
  description="Profile photo"
```

### Offline Mode (Preview Request)
```bash
rusttp --offline POST https://httpbin.org/post name=Anilcan
# Prints what would be sent — no actual HTTP request made
```

### Verbose Output
```bash
rusttp -v GET https://httpbin.org/get    # Full request + response
rusttp --headers GET https://httpbin.org/get   # Response headers only
rusttp -b GET https://httpbin.org/get    # Response body only
```

---

## Request Item Syntax Reference

| Syntax | Type | Example |
|---|---|---|
| `key=value` | JSON string field | `name="Anilcan"` |
| `key:=value` | Raw JSON value | `age:=28`, `ok:=true` |
| `key==value` | URL query param | `page==2` |
| `header:value` | HTTP header | `X-Token:secret` |
| `key@file` | File upload | `photo@./image.jpg` |

---

## Contributing

Contributions are very welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

---

## License

MIT © [Anilcan Kara](https://github.com/anilcan-kara)
