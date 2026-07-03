# Quorum

Quorum was inspired by Discord, born from community feedback about privacy concerns, security, and performance. This project focuses on three core goals:

- **Privacy**: Collect only what's necessary and let users control their data
- **Security**: Make unauthorized access and data theft practically impossible
- **Performance**: Keep everything snappy without compromising security
 
---

# Why Quorum?

Discord collects extensive user data, shares it with third parties, trains AI models on your conversations, and has suffered repeated security breaches. Of course, this applies to many other platforms, not just discord. Quorum is built differently - privacy isn't a feature you toggle on, it's the foundation.

- **Open source** - audit every line of code that touches your data
- **Self-hostable** - run your own server for complete control
- **No tracking** - we don't collect data we don't need
- **No AI training** - your conversations stay yours
- **No ads** - ever

---

## Tech Stack

| Layer | Technology |
|---|---|
| App | Tauri + Svelte |
| Backend | Rust + Axum |
| Database | SurrealDB |

---

## Project Structure

```
Quorum/
├── client/          # Tauri app (Svelte frontend)
├── server/          # Axum backend (Rust)
├── wireframes/      # UI design sketches
```

## Tools required

- [Node.js LTS](https://nodejs.org/)
- [Rust](https://rustup.rs/)

---

## Getting Started

### 1. Clone the repo

```bash
git clone https://github.com/Jamster3000/Quorum.git
cd quorum
```

### 2. Start the backend

```bash
cd server
cargo run -p quorum-private
```
`cargo run -p quorum-private` runs in debug by default and needs the --release flag to run in release.

> Or use `cargo run -p quorum-public` to build the public server.

### 6. Start the Tauri application

```bash
cd client
npm install
npm run tauri dev
```

---

## Stopping the server

The server running in debug mode, Ctrl-C works as normal. In release mode (the normal production release that everyone would be using) blocks ctrl-c, requiring an administrator user, to be logged in and use the `server:shutdown` command.

## Wiping database
Shutting the server down preserves all data in the database. To wipe the database, use the `db:delete` command as an administrator.

---

# Icons 
Using icons from Tabler Icons, licensed under MIT License via the package `icons-svelte`
https://tabler.io/icons


## Contributing

Contributions are welcome. Please read [CONTRIBUTE.md](CONTRIBUTE.md) before opening a pull request.

---

## License

TBD — license will be added before public release.
