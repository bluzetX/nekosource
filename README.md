# 🐱 NekoSource

A lightweight, cross-platform desktop Git client — written in pure Rust.

---

## Features

- **Cross-platform** — native builds for Windows, macOS, and Linux.
- **Localization** — English, Russian, Ukrainian, and Belarusian.
- **Easter eggs** — seasonal UI tweaks and a background soundtrack.

---

## Tech Stack

| Crate                                              | Role                               |
|----------------------------------------------------|------------------------------------|
| [`eframe` / `egui`](https://github.com/emilk/egui) | Immediate-mode GPU-accelerated GUI |
| [`git2`](https://github.com/rust-lang/git2-rs)     | libgit2 bindings — the Git engine  |

---

## Building

You only need the Rust toolchain. No other prerequisites.

```bash
git clone https://github.com/OctoBanon-Main/nekosource.git
cd nekosource-rs
cargo build --release
```

The binary lands in `target/release/`.

## Credits

**Ukrainian translation & belarusian translation** — [ShadowCj](https://github.com/bluzetX)  
**Belarusian translation** — [xelframe](https://github.com/xelframe)

## License

This project is licensed under the MIT license.
