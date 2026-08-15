> This project is under development. It was and is primarily a learning project and there are currently no plans for hosting or making this publicly usable.

# Quorum
Quorum was inspired by Discord, born from community feedback about privacy concerns, security, and performance. This project focuses on three core goals:

- **Privacy**: Collect only what's necessary and let users control their data
- **Security**: Make unauthorized access, data theft and more practically impossible
- **Performance**: Keep everything snappy without compromising security.

> Quorum is in early development.
---

# Why Quorum?

Discord collects extensive user data, shares it with third parties, trains AI models on your conversations, and has suffered repeated security breaches. Of course, this applies to many other platforms, not just discord. Quorum is built differently - privacy isn't a feature you toggle on, it's the foundation.

- **Open source** - All the code that makes up quorum is entirely open source!
- **Self-hostable** - Don't want to create a group on the public server? Self host Quorum as a private server for your group.
- **No tracking** - We limit the data we collect to a very minimal.
- **No AI training** - No A.I. is used and no user data is ever collected for data training.
- **No ads** - We're all sick of ads, we wouldn't want to put ads up to the user.

---

## Wireframes
> We used wireframes to get the structure and layout right. The colors presented **do not** reflect the final design.
> NOTE: A "group" can have one or more "spaces." The "spaces" shown may not reach final production.

---

### Home screen for authenticated user
<img width="650" alt="Home screen for authenticated user" src="https://github.com/user-attachments/assets/a78bc95f-6135-484a-b128-6d881abfe0f4" />

---

### Create an account
<img width="650" alt="Create an account" src="https://github.com/user-attachments/assets/782b2717-862c-496e-95d5-d5f089d4a9c4" />

---
### Login
<img width="650" alt="Login" src="https://github.com/user-attachments/assets/3dcd6923-d13c-42c9-97b3-3a90a96d1c43" />

---
### Account settings
<img width="650" alt="Account settings" src="https://github.com/user-attachments/assets/03344d1e-6dac-4c0c-b52a-f78c9bfdcec7" />

---
### Search for public groups
<img width="650" alt="Search for public groups" src="https://github.com/user-attachments/assets/a677ee8e-beb7-4d7c-8e06-2ff5289a60d3" />

---
### Creating a group (on public server)
<img width="650" alt="Creating a group (on public server)" src="https://github.com/user-attachments/assets/2ce58c89-0acf-4096-90b2-02caa3bf84f0" />

---
### Group
<img width="650" alt="Group" src="https://github.com/user-attachments/assets/1021d59e-7374-4c00-b0e0-e4c47bfbe1c6" />

---
### Group Space - Voice
<img width="650" alt="Group Space - Voice" src="https://github.com/user-attachments/assets/504a182c-20af-43e2-819b-36a5ebc4074e" />

---
### Photo Gallery Space - viewing
<img width="650" alt="Photo Gallery Space - viewing" src="https://github.com/user-attachments/assets/6c4fe29c-9cf5-4f53-bd4d-240942f73c89" />

---
### Photo Gallery Space - Adding/uploading an image
<img width="650" alt="Photo Gallery Space - Adding/uploading an image" src="https://github.com/user-attachments/assets/5a5df9b1-7e26-4900-afef-95f318b81d43" />

---
### IDE/notepad space
<img width="650" alt="IDE/notepad space" src="https://github.com/user-attachments/assets/bfd624a6-76fd-43ed-84ae-bf43b9746219" />

---
### IDE/notepad space - file editing
<img width="650" alt="IDE/notepad space - file editing" src="https://github.com/user-attachments/assets/d050e304-380e-4a22-8839-0ed1cd852235" />

---
### Project Board Space - To Do
<img width="650" alt="Project Board Space - To Do" src="https://github.com/user-attachments/assets/f7617d87-c83c-4922-967f-26aee65055e2" />

---
### Project Board Space - Kanban
<img width="650" alt="Project Board Space - Kanban" src="https://github.com/user-attachments/assets/010deafc-cf28-4e01-8099-7f626a6bc93b" />

---
### Project Board Space - Gantt Chart
<img width="650" alt="Project Board Space - Gantt Chart" src="https://github.com/user-attachments/assets/3553ce94-3930-6506-a9c0-be6d5efe76e1" />

---
### Project Board Space - Calendar
<img width="650" alt="Project Board Space - Calendar" src="https://github.com/user-attachments/assets/067c4732-e3f5-41f6-9d61-482552dda8a3" />

---
### Project Board Space - Table
<img width="650" alt="Project Board Space - Table" src="https://github.com/user-attachments/assets/3d89745d-d470-4a96-9ede-673634b5e399" />
---

## Features

### Core Functionality
- **Authentication** - Create accounts and login with username and password. Sign up with email is optional but suggested for account recovery.
- **Direct Messages** - Send other users on the platform private secure messages (Planned)
- **Message Compression** - All messages use custom text compression, saving server storage and allowing up to 4096 characters per message (Planned)
- **Message Encryption** - All messages are encrypted using Full Homomorphic Encryption, allowing us to filter banned content without ever reading a user's message (Planned)
- **Groups** - Create groups for your friends and set up multiple spaces in each group to organize chat content (Planned)
- **Communities** - Evolve your group into a community, allowing it to be accessed by the public (Planned)
- **Moderation Tools** - Strong, simple moderation tools to help groups and communities rid themselves of spam and rule violations, keeping them friendly and clean (Planned)
- **Custom Banned Words** - Allow group and community moderators to add custom banned words for their spaces (Planned)
- **Voice & Video** - Send voice messages or join calls to speak verbally, with optional video (Planned)

### Server
- **First Time Setup** - Automatically create default configuration settings so you don't need to understand or manually configure them. Settings are encrypted so unauthorized access can't expose them.
- **Passphrase Protection** - Set up a passphrase for critical server actions, ensuring only the administrator can perform sensitive operations securely.
- **Server Commands** - Interact with the database and server using simple commands.
- **Automated Tests** - Enable server tests to run through all functionality and verify everything works correctly.

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
Using icons from Tabler Icons, licensed under [MIT](https://github.com/tabler/tabler-icons/blob/main/LICENSE) License via the package `icons-svelte`
https://tabler.io/icons

## Contributing

Contributions are welcome. Please read [CONTRIBUTING.md](CONTRIBUTING.md) before opening a pull request.

---

## License

Quorum is licensed under the [GNU Affero General Public License v3.0 (AGPL-3.0)](https://www.gnu.org/licenses/agpl-3.0.html).

In short: you're free to use, modify, and self-host Quorum. If you modify it and run it as a service, you must release your changes under the same license. This ensures Quorum stays open and transparent, even in forks.

See the [LICENSE](LICENSE) file for full details.
