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
- **GStreamer** runtime for Windows ([gstreamer.freedesktop.org](https://gstreamer.freedesktop.org/download/))

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
