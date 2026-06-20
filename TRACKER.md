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
- [x] Add iced dependency to Cargo.toml
- [x] Add gstreamer-rs dependencies (gstreamer, gstreamer-video, gstreamer-app, gstreamer-pbutils)
- [x] Add tokio dependency for async subscriptions (replaced windows-rs)
- [x] Add supporting crates (clap, serde, toml, tracing, dirs)
- [x] Verify build compiles with all deps on Windows
- [x] Document GStreamer Windows setup in README

### 1.2 Project Structure
- [x] Create module skeleton (ui/, engine/, platform/, utils/)
- [x] Set up mod.rs files with public API stubs
- [x] Set up tracing/logging infrastructure
- [x] Set up config loading (TOML) with defaults

### 1.3 GStreamer Playback Engine
- [x] Initialize GStreamer in main()
- [x] Create playbin-based pipeline wrapper (`engine/pipeline.rs`)
- [x] Implement play / pause / stop
- [x] Implement seek (absolute + relative ±5s, ±30s)
- [x] Implement volume control + mute
- [x] Query duration and position (for seekbar)
- [x] Handle end-of-stream and error states
- [x] Extract video frames as raw RGBA for rendering in iced

### 1.4 iced UI Shell
- [x] Create iced Application with Elm-architecture message loop
- [x] Define dark theme (colors, fonts, spacing)
- [x] Build main window layout (video viewport + bottom controls bar)
- [x] Implement video frame rendering (Image widget or Shader)
- [x] Build playback controls (play/pause button, volume slider, time display)
- [x] Build seekbar with click-to-seek
- [x] Implement fullscreen toggle
- [x] Implement drag-and-drop file opening
- [x] Implement right-click context menu (Deferred to Phase 2)
- [x] Wire up keyboard shortcuts (Space, F, arrows, M, ↑↓, [, ], Backspace)

### 1.5 Basic Subtitle Support
- [x] Detect embedded subtitle tracks via GStreamer
- [x] Render SRT subtitles as text overlay (Handled via GStreamer playbin)
- [x] Render ASS/SSA subtitles (basic styling) (Handled via GStreamer playbin)
- [x] Auto-load external .srt/.ass files from same directory (Handled via `set_external_subtitles`)
- [x] Subtitle track cycling (V key)

### 1.6 Integration & Polish
- [x] CLI argument: open file from command line (`velocity.exe <path>`)
- [x] Window title shows "Velocity — filename.mkv"
- [x] OSD notifications (volume change, play/pause, speed change)
- [x] Audio track selection (B key to cycle)
- [x] Variable playback speed (0.25x–4.0x, [ and ] keys)
- [x] Error handling: graceful messages for unsupported files
- [x] Commit and push after each sub-milestone

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
