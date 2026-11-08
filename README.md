Maintained by [Ankara Yazılım](https://github.com/Ankara-Software) | [Project page](https://ankarayazilim.org/acik-kaynak/rusttp)

**Status: stable.** It builds, the 7 unit tests pass and the README examples (GET, POST with JSON, form, offline preview) were run against a local server. It implements a subset of the httpie request syntax, not all of it.

<div align="center">

<h1>rusttp</h1>

<p><strong>An HTTP client for the terminal with httpie-style request syntax — written in Rust.</strong></p>

[![License: MIT](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE)
[![GitHub Release](https://img.shields.io/github/v/release/Ankara-Software/rusttp?style=flat-square&color=8be04e)](https://github.com/Ankara-Software/rusttp/releases)

```
rusttp POST https://httpbin.org/post name="Anilcan Kara" age:=28 X-Token:secret
```

</div>

## Türkçe

rusttp, httpie benzeri istek söz dizimiyle çalışan, terminal için bir HTTP istemcisidir. `key=value` ile JSON alanı, `key:=değer` ile ham JSON değeri, `key==değer` ile sorgu parametresi, `Header:değer` ile başlık ve `alan@dosya` ile dosya yükleme gönderir. Yanıtı renklendirerek gösterir. `--offline` ile isteği göndermeden ne gideceğini yazdırır. Tek bir yerel (native) ikili dosya olarak gelir, Python gerektirmez. httpie söz diziminin yalnızca bir alt kümesini uygular.

---

## Why rusttp?

`httpie` is a Python tool and needs a Python runtime. rusttp offers a similar request syntax as a single native binary, with no runtime to install.

| | rusttp | httpie |
|---|---|---|
| Runtime dependency | None | Python 3 |
| Request syntax | httpie-style subset (see below) | Full httpie syntax |
| Syntax highlighting | Yes | Yes |
| JSON, form and file upload | Yes | Yes |
| Offline mode (print request) | Yes (`--offline`) | Yes (`--offline`) |

---

## Installation

### Install script (Linux / macOS)
```bash
curl -fsSL https://raw.githubusercontent.com/Ankara-Software/rusttp/master/install.sh | sh
```

### Cargo (from source, Rust 1.85+)
```bash
cargo install --git https://github.com/Ankara-Software/rusttp
```
The crate is also published on crates.io as `rusttp-cli`. The published 0.1.1 installs a binary named `rusttp-cli`; building from this repository installs `rusttp`.

### Pre-built binaries

Download the binary for your platform from the [latest release](https://github.com/Ankara-Software/rusttp/releases/latest): `rusttp-win32-x64.exe`, `rusttp-linux-x64`, `rusttp-linux-arm64`, `rusttp-darwin-x64`, `rusttp-darwin-arm64`.

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
Prints the request that would be sent, without sending it:
```bash
rusttp --offline POST https://httpbin.org/post name=Anilcan
```

### Request items
```
key=value      JSON string field
key:=value     Raw JSON value (number, bool, array, object)
key==value     URL query parameter
Header:value   HTTP header
key@file       File upload (multipart)
```
Use `--form` for form-encoded fields, `--auth user:password` (or `--auth-type bearer`), `--follow` for redirects, `--download -o file`, `--proxy`, `--raw` to disable formatting. Run `rusttp --help` for all options.

---

## Development

```bash
cargo build
cargo test
```

See [CONTRIBUTING.md](CONTRIBUTING.md), [SECURITY.md](SECURITY.md) and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

## License

MIT. Copyright (c) 2026 Anilcan Kara. See [LICENSE](LICENSE).
