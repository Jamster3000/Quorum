# src-tauri

This directory contains the **Tauri-specific backend and configuration** for the Quorum client. It handles the bridge between the Svelte frontend and the Rust-based backend logic, including window management, security, and native features.

---

## Structure

| File/Folder       | Purpose                                                                                     |
|-------------------|---------------------------------------------------------------------------------------------|
| `capabilities/`   | Custom Tauri capabilities (e.g., secure storage, native APIs).                              |
| `gen/schemas/`    | Generated schemas (e.g., for Tauri store or IPC communication).                             |
| `icons/`          | App icons.                                                                                  |
| `src/`            | Rust source code for Tauri commands and backend logic.                                      |
| `Cargo.toml`      | Rust dependencies and configuration for the Tauri backend.                                  |
| `build.rs`        | Build script for the Tauri backend.                                                         |
| `tauri.conf.json` | Tauri application configuration (window settings, permissions, plugins, etc.).              |

---
