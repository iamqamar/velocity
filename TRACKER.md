# Velocity — Project Tracker

> Last updated: 2026-06-20
> Current stage: **Phase 1 — MVP (v0.1.0)** — In Progress

This file tracks what's been done, what's in progress, and what's next.
If a session is interrupted (crash, network, memory), pick up from the **last checked item**.

---

## Pre-Development (Complete)

- [x] Verify Rust toolchain (rustc 1.94.1, cargo 1.94.1, stable-msvc)
- [x] Initialize cargo project (`velocity`)
- [x] Write PRD with full feature spec (60+ features, 13 sections)
- [x] Resolve all open decisions (iced, GStreamer, MSIX, MIT/Apache 2.0)
- [x] Create project-specific `.gitignore`
- [x] Create `README.md` with branding and tech stack
- [x] Create `LICENSE-MIT` and `LICENSE-APACHE`
- [x] Set up git remote (`github.com/iamqamar/velocity`)
- [x] Push initial commits (2 commits on `main`)
- [x] Clean up AI fingerprints from code and docs

---

## Phase 1 — MVP (v0.1.0)

### 1.1 Dependencies & Build Setup
- [ ] Add iced dependency to Cargo.toml
- [ ] Add gstreamer-rs dependencies (gstreamer, gstreamer-video, gstreamer-app, gstreamer-pbutils)
- [ ] Add windows-rs for platform APIs
- [ ] Add supporting crates (clap, serde, toml, tracing, dirs)
- [ ] Verify build compiles with all deps on Windows
- [ ] Document GStreamer Windows setup in README

### 1.2 Project Structure
- [ ] Create module skeleton (ui/, engine/, media/, platform/, utils/)
- [ ] Set up mod.rs files with public API stubs
- [ ] Set up tracing/logging infrastructure
- [ ] Set up config loading (TOML) with defaults

### 1.3 GStreamer Playback Engine
- [ ] Initialize GStreamer in main()
- [ ] Create playbin-based pipeline wrapper (`engine/pipeline.rs`)
- [ ] Implement play / pause / stop
- [ ] Implement seek (absolute + relative ±5s, ±30s)
- [ ] Implement volume control + mute
- [ ] Query duration and position (for seekbar)
- [ ] Handle end-of-stream and error states
- [ ] Extract video frames as raw RGBA for rendering in iced

### 1.4 iced UI Shell
- [ ] Create iced Application with Elm-architecture message loop
- [ ] Define dark theme (colors, fonts, spacing)
- [ ] Build main window layout (video viewport + bottom controls bar)
- [ ] Implement video frame rendering (Image widget or Shader)
- [ ] Build playback controls (play/pause button, volume slider, time display)
- [ ] Build seekbar with click-to-seek
- [ ] Implement fullscreen toggle
- [ ] Implement drag-and-drop file opening
- [ ] Implement right-click context menu
- [ ] Wire up keyboard shortcuts (Space, F, arrows, M, ↑↓)

### 1.5 Basic Subtitle Support
- [ ] Detect embedded subtitle tracks via GStreamer
- [ ] Render SRT subtitles as text overlay
- [ ] Render ASS/SSA subtitles (basic styling)
- [ ] Auto-load external .srt/.ass files from same directory
- [ ] Subtitle track cycling (V key)

### 1.6 Integration & Polish
- [ ] CLI argument: open file from command line (`velocity.exe <file>`)
- [ ] Window title shows "Velocity — filename.mkv"
- [ ] OSD notifications (volume change, play/pause, speed change)
- [ ] Audio track selection (B key to cycle)
- [ ] Variable playback speed (0.25x–4.0x, [ and ] keys)
- [ ] Error handling: graceful messages for unsupported files
- [ ] Commit and push after each sub-milestone

### 1.7 Phase 1 Verification
- [ ] Test with MP4 (H.264 + AAC)
- [ ] Test with MKV (HEVC + Opus + SRT subs)
- [ ] Test with WebM (VP9 + Vorbis)
- [ ] Test with AVI (legacy codec)
- [ ] Test fullscreen on multi-monitor
- [ ] Test drag-and-drop
- [ ] Test all keyboard shortcuts
- [ ] Memory usage check (target: <150MB for 1080p)
- [ ] Tag release: `v0.1.0-alpha`

---

## Phase 2 — PiP & Polish (v0.2.0) — Not Started

- [ ] One-click PiP mode (Ctrl+P) with always-on-top window
- [ ] Resizable PiP with aspect ratio lock
- [ ] PiP hover controls (play/pause, close, expand)
- [ ] PiP opacity slider (50%–100%)
- [ ] Multi-monitor PiP position memory
- [ ] Playlist side panel with drag-and-drop reordering
- [ ] Resume playback (remember position per file)
- [ ] Recently played list
- [ ] Thumbnail preview on seekbar hover
- [ ] Customizable keyboard shortcuts (settings panel)
- [ ] Screenshot capture (S key → PNG)
- [ ] Screenshot to clipboard

---

## Phase 3 — Power Features (v0.3.0) — Not Started

- [ ] Audio equalizer with presets
- [ ] Audio normalization
- [ ] Dual subtitle display
- [ ] Subtitle download (OpenSubtitles API)
- [ ] Chapter navigation
- [ ] Open URL playback (HTTP/HTTPS)
- [ ] HLS / DASH streaming
- [ ] RTSP stream support
- [ ] Windows SMTC integration (media keys)
- [ ] Taskbar thumbnail controls
- [ ] MSIX installer with file associations
- [ ] Full CLI flags (--pip, --fullscreen, --volume)

---

## Phase 4 — Ecosystem (v1.0.0) — Not Started

- [ ] Plugin / extension architecture
- [ ] Media library with folder scanning
- [ ] Clip recording
- [ ] Portable mode
- [ ] MSIX auto-update
- [ ] Accessibility audit
- [ ] Microsoft Store listing

---

## Recovery Notes

If this session is interrupted, the next agent or developer should:

1. Read this file to see what's checked off
2. Check `git log --oneline` to see the last commit
3. Run `cargo build` to verify the project still compiles
4. Continue from the first unchecked `[ ]` item in the current phase
5. After completing any sub-section, commit + push + update this tracker
