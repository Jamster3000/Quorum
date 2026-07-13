# Quorum Client (Tauri + Svelte)

Desktop, web, and mobile client built with **Tauri** and **Svelte**. Handles UI, authentication logic (via Rust commands), and communication with the Quorum server.

---

## Structure
| File/Folder       | Purpose                                                                 |
|-------------------|-------------------------------------------------------------------------|
| `src-tauri/`      | Tauri backend and configuration files for the client application.       |
| `src/`            | The Svelte frontend where pages and components live                     |
| `package.json`    | Node.js dependencies and scripts for the Svelte frontend.               |
| `tauri.conf.json` | Tauri application configuration (e.g., window settings, permissions).   |
| `svelte.config.js`| Svelte-specific configuration for the frontend.                         |
| `vite.config.js`  | Vite configuration for bundling the Svelte app.                         |

---

## Running the Client
1. Install dependencies:
   ```bash
   npm install
   ```
2. Start the development server:
   ```bash
   npm run tauri dev
   ```
3. For production builds:
   ```bash
   npm run tauri build
   ```

---
## 🔧 Configuration
- **Tauri Config**: Modify `tauri.conf.json` to adjust app settings (e.g., window size, permissions).
- **Svelte Config**: Update `svelte.config.js` for frontend-specific settings.
- **Vite Config**: Adjust `vite.config.js` for bundling and optimization.

---
## Dependencies
- **Tauri**: Used for building the desktop/mobile app shell.
- **Svelte**: Frontend framework for UI components.
