# Quorum
Quorum is a privacy-focused, open source application - originally built with the intention of being an alternative to discord.

Quorum is a work in progress and has limited use/functionality. Because Quorum is open source, this means others can host the backend on their own computer or server privately, meaning your conversations and data stay on infrastructure you own and trust (More details about this and more as it's developed)
 
---

## Why Quorum?

Discord harvests and shares user data with third parties, trains AI on your conversations and has been a little too well known for getting hacked or being unsecure. Quorum is built from the ground up with privacy as a non-negotiable — not a setting, not a tier, just how it works.

- **Open source** — read every line of code that handles your data
- **Self-hostable** — run your own private server for your group
- **No ads** — ever
- **No AI training** on any of your data

---

## Tech Stack

| Layer | Technology |
|---|---|
| Desktop app | Tauri 2 + SvelteKit + hand-written CSS |
| Backend | Rust + Axum |
| Database | SurrealDB |
| File storage | MinIO |
| Infrastructure | Docker |

---

## Project Structure

```
Quorum/
├── client/          # Tauri desktop app (Svelte frontend)
├── docker/          # Docker Compose configuration
├── docs/            # Whehre all of quorum's documentation lays
├── ERD              # Entity Relationship Diagram used for visual representation of the database
├── schema/          # SurrealQL database schema
├── server/          # Axum backend (Rust)
├── wireframes/      # UI design sketches
└── .env.example     # Environment variable template
```

---

## Requirements

- [Docker Desktop](https://www.docker.com/products/docker-desktop/)
- [Node.js LTS](https://nodejs.org/)
- [Rust](https://rustup.rs/)

---

## Getting Started

### 1. Clone the repo

```bash
git clone https://github.com/Jamster3000/Quorum.git
cd quorum
```

### 2. Set up environment variables

```bash
cp .env.example .env
```

Open `.env` and fill in your values. The defaults work for local development without changes.

### 3. Start the database and file storage

```bash
cd docker
docker compose up -d
```

This starts:
- SurrealDB on `localhost:8000`
- MinIO on `localhost:9000` (dashboard at `localhost:9001`)

### 4. Apply the database schema
> This happens automatically and all tables in initialize.sqrl are created when the server starts up. SKIP this step if you are going to be running the server. This step is more informational on how to view the database.

1. Go to [app.surrealdb.com](https://app.surrealdb.com)
2. Create a new connection with these details:
   - **Protocol**: WS
   - **Host**: `localhost:8000`
   - **Username**: `root`
   - **Password**: `root`
3. Open the query editor
4. Paste the contents of `schema/initial.surql`
5. Run the query

> You may create an account with [app.surrealdb.com](https://app.surrealdb.com) if you wish but it is **not essential** or required for running surrealDB queries.

### 5. Start the backend

```bash
cd server
cargo run
```
> Or use cargo run dev if you're developing

### 6. Start the desktop app

```bash
cd client
npm install
npm run tauri dev
```

---

## Stopping

```bash
cd docker
docker compose down
```

Data is preserved. To wipe everything and start fresh:

```bash
docker compose down -v
```

---

## Contributing

Contributions are welcome. Please read [CONTRIBUTE.md](CONTRIBUTE.md) before opening a pull request.

---

## License

TBD — license will be added before public release.
