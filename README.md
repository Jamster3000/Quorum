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

# Icons 
Using icons from Tabler Icons, licensed under MIT License via the package `icons-svelte`
https://tabler.io/icons


## Contributing

Contributions are welcome. Please read [CONTRIBUTE.md](CONTRIBUTE.md) before opening a pull request.

---

## License

TBD — license will be added before public release.
