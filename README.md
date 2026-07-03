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

## Features

### Core Functionality
- **Authentication** - Create accounts and login with username and password. Sign up with email is optional but suggested for account recovery.
- **Direct Messages** - Send other users on the platform private secure messages (Planned)
- **Message Compression** - All messages use custom text compression, saving server storage and allowing up to 4096 characters per message (Planned)
- **Message Encryption** - All messages are encrypted using Full Homomorphic Encryption, allowing us to filter banned content without ever reading message text (Planned)
- **Groups** - Create groups for your friends and set up multiple spaces in each group to organize chat content (Planned)
- **Communities** - Evolve your group into a community, allowing it to be accessed by the public (Planned)
- **Moderation Tools** - Strong, simple moderation tools to help groups and communities rid themselves of spam and rule violations, keeping them friendly and clean (Planned)
- **Custom Banned Words** - Allow group and community moderators to add custom banned words for their spaces (Planned)
- **Voice & Video** - Send voice messages or join calls to speak verbally, with optional video (Planned)

### Server
- **First Time Setup** - Automatically create default configuration settings so you don't need to understand or manually configure them. Settings are encrypted so unauthorized access can't expose them
- **Passphrase Protection** - Set up a passphrase for critical server actions, ensuring only the administrator can perform sensitive operations securely
- **Administrator Setup** - Login with your Quorum account and promote yourself to admin as a one-time-only feature, meaning only your account can perform most server actions
- **Server Commands** - Interact with the database and server using command-line commands
- **Automated Tests** - Enable server tests to run through all functionality and verify everything works correctly

---

### Planned Features
- **Voice & Video** — group calls via SFU relay
- **FHE Moderation** — server-side content filtering without reading message content
- **File Storage** — share files and images within groups
- **Federation** - Host your own private server where your group or community can run independently from the public server. Quorum users can seamlessly navigate between public and private groups

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

### 3. Start the Tauri application

```bash
cd client
npm install
npm run tauri dev
```

---

# Icons 
Using icons from Tabler Icons, licensed under MIT License via the package `icons-svelte`
https://tabler.io/icons


## Contributing

Contributions are welcome. Please read [CONTRIBUTE.md](CONTRIBUTE.md) before opening a pull request.

---

## License

Quorum is licensed under the [GNU Affero General Public License v3.0 (AGPL-3.0)](https://www.gnu.org/licenses/agpl-3.0.html).

In short: you're free to use, modify, and self-host Quorum. If you modify it and run it as a service, you must release your changes under the same license. This ensures Quorum stays open and transparent, even in forks.

See the [LICENSE](LICENSE) file for full details.
