# ⚡ Velocity

> *Play anything, beautifully.*

A modern, high-performance video player for Windows — built in Rust. Velocity is an open-source alternative to VLC with a premium UI, first-class **Picture-in-Picture**, and GPU-accelerated playback.

[![License: MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)]()

---

## ✨ Highlights

- 🎬 **Universal Format Support** — MP4, MKV, WebM, AVI, MOV, and 30+ codecs including H.264, HEVC, AV1, VP9
- 🖼️ **One-Click Picture-in-Picture** — Compact, always-on-top floating window with opacity and click-through controls
- ⚡ **Hardware-Accelerated** — DXVA2 / D3D11VA GPU decoding for butter-smooth 4K HDR playback
- 🎨 **Modern Dark UI** — Sleek, adaptive interface built with [iced](https://iced.rs) and rendered via `wgpu`
- 📝 **Powerful Subtitles** — SRT, ASS/SSA, PGS with custom styling and sync adjustment
- 🔊 **Advanced Audio** — Multi-track switching, equalizer, pitch-corrected variable speed (0.25×–4.0×)
- 📦 **Clean MSIX Installer** — Modern Windows packaging with seamless install, update, and uninstall

## 🛠️ Tech Stack

| Layer | Technology |
| :--- | :--- |
| Language | Rust (Edition 2024) |
| Multimedia | GStreamer via `gstreamer-rs` |
| GUI | iced (Elm architecture, GPU-rendered) |
| GPU | wgpu (Direct3D 12/11, Vulkan) |
| Platform | windows-rs |
| Packaging | MSIX |

## 📁 Project Structure

```text
velocity/
├── Cargo.toml
├── LICENSE-MIT
├── LICENSE-APACHE
├── README.md
├── PRD.md                      # Product Requirements Document
├── packaging/                  # MSIX packaging assets
├── src/
│   ├── main.rs                 # Entry point
│   ├── app.rs                  # iced Application state
│   ├── ui/                     # UI components (controls, seekbar, PiP, etc.)
│   ├── engine/                 # GStreamer pipeline, decoders, audio
│   ├── media/                  # Format probing, playlists, library
│   ├── platform/               # Windows APIs, CLI, config
│   └── utils/                  # Logging, time formatting
└── tests/
```

## 🚀 Getting Started

### Prerequisites

- **Rust** 1.94+ (install via [rustup.rs](https://rustup.rs/))
- **GStreamer MSVC binaries** for Windows. Install both the **Runtime** and **Development** packages from [gstreamer.freedesktop.org](https://gstreamer.freedesktop.org/download/) (select MSVC 64-bit packages, v1.22+).

### Windows Environment Configuration

For Cargo to compile and link GStreamer correctly, you must set up the following environment variables (adjust paths if installed in a custom directory):

#### PowerShell (Current Session)
```powershell
$env:GSTREAMER_1_0_ROOT_MSVC_X86_64 = "C:\Program Files\gstreamer\1.0\msvc_x86_64\"
$env:PKG_CONFIG_PATH = "C:\Program Files\gstreamer\1.0\msvc_x86_64\lib\pkgconfig\"
$env:Path = "C:\Program Files\gstreamer\1.0\msvc_x86_64\bin;" + $env:Path
$env:LIB = "C:\Program Files\gstreamer\1.0\msvc_x86_64\lib;" + $env:LIB
```

#### CMD (Current Session)
```cmd
set GSTREAMER_1_0_ROOT_MSVC_X86_64=C:\Program Files\gstreamer\1.0\msvc_x86_64\
set PKG_CONFIG_PATH=C:\Program Files\gstreamer\1.0\msvc_x86_64\lib\pkgconfig\
set Path=C:\Program Files\gstreamer\1.0\msvc_x86_64\bin;%Path%
set LIB=C:\Program Files\gstreamer\1.0\msvc_x86_64\lib;%LIB%
```

### Build

```bash
cargo build
```

### Run

```bash
cargo run
```

### Live Development

```bash
cargo watch -x run
```

## 📋 Documentation

- [Product Requirements Document (PRD)](./PRD.md) — Full feature specification, architecture, and roadmap

## 📄 License

Velocity is dual-licensed under [MIT](./LICENSE-MIT) and [Apache 2.0](./LICENSE-APACHE). You may use this project under the terms of either license.
