Quorum has three separate things running at the same time during development:

| Piece | What it is | How you run it | Default address |
|---|---|---|---|
| SurrealDB | The database | Docker | `localhost:8000` |
| MinIO | File/image storage | Docker | `localhost:9000` |
| Axum backend | Your Rust server | `cargo run` | `localhost:3000` |

The Tauri frontend is a desktop app — it doesn't "run" on a port, it just opens a window.

---

## How They Connect

```
Tauri desktop app
      |
      | HTTP requests
      v
Axum backend (localhost:3000)
      |
      |--- SQL queries --------> SurrealDB (localhost:8000)
      |
      |--- file upload/fetch ---> MinIO (localhost:9000)
```

- The **frontend** never talks to the database directly. It only talks to the backend.
- The **backend** talks to SurrealDB and MinIO on your behalf.
- The **database and file storage** never talk to the frontend at all.

---

## Docker's Job

Docker runs SurrealDB and MinIO in isolated containers on your machine.
They expose ports (8000 and 9000) so your backend can reach them over the network.

The actual database file lives inside a Docker-managed volume — not in your project folder.
This is normal. You never need to touch the file directly.

```bash
# Start everything
docker compose up -d

# Stop everything (data is preserved)
docker compose down

# Stop everything AND wipe all data (fresh start)
docker compose down -v
```

---

## The Backend's Job

The Axum backend is a Rust HTTP server. It:
- Receives requests from the Tauri frontend
- Validates and processes them
- Queries SurrealDB or reads/writes files in MinIO
- Returns a response to the frontend

During development you run it directly:
```bash
cd server
cargo run
```

It connects to Docker's SurrealDB via the URL in your `.env` file:
```
SURREAL_URL=ws://localhost:8000
```

---

## First Time Setup

1. Install Docker Desktop
2. `cd docker && docker compose up -d`
3. Go to `https://app.surrealdb.com`, connect to your local instance and run `schema/initial.surql`
4. `cd server && cargo run`
5. `cd client && npm run tauri dev`