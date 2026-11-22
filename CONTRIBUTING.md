# Contributing to Quorum

## Prerequisites

- Rust
- Node.js (includes npm and npx)

## Running Locally

**Backend** (run from `server`):

```bash
cargo run -p quorum-private   # Private server
cargo run -p quorum-public    # public server
```

**Frontend** (run from `client`):

```bash
npm install   # first time only
npm run tauri dev
```

## Making a Change

### Step 1: get a branch

**If you're a member of the project team**, create a branch in this repo from `main`:

```bash
git checkout main
git pull
git checkout -b <branch-name>
```

> NOTE: A lot of IDE's (e.g. Visual Studio, RustRover, etc.) have built in git functionality that you can use over direct terminal commands.

**If you're an outside contributor**, fork the repo on GitHub, clone your fork, and create a branch from `main` in your fork. You'll open your pull request from the fork back to this repo.

### Step 2: Make your changes and run the checks

- Client: `npm run checks` from `client` (Windows), or `run-checks.sh` (Linux and macOS)
- Sever: `cargo fmt --all` and `cargo clippy -- -W warnings`.

Run these before you push. CI runs the same checks, so this saves you a round trip.

> If you use the RustRover IDE, there are already configuration commands that run most things you may need.

### Step 3: Open a pull request in `main`

Open a pull request against `main` in this repo, with a short description of what you changed and why. CI runs automatically and must pass before the PR can merge.

Every commit does have to go through a pull request.

### Step 4: Keep your branch up to date

If the PR says it's behind `main`, click **Update branch** (or pull `main` into your branch).

CI only runs the checks for the folders that have had changes made in the given branch. a `server` change runs the server checks only, and a `client` change runs the client checks.

## More Detail

Code styles, documentation comment rules, SurrealQL conventions and more are on the [project wiki](https://github.com/jamster3000/Quorum/wiki)