# Quorum

Quorum is a chat platform inspired by Discord, built around three primary goals:

- **Privacy:** collect only what's needed, and let users control their data.
- **Security:** make unauthorised access and data theft as hard as pratically possible.
- **Performance:** stay fast without comprimising security and keep memory usage low.

The server is designed to run as a single binary with no external dependencies.

## Status

Early development. Most of the app isn't built yet. For what's done and what's planned, see the [wiki](https://github.com/jamster3000/Quorum/wiki).

## Structure

- `client/`: Tauri app (Svelte)
- `server/`: Rust backend (Axum, SurrealDB)

## Getting started

Build and run steps are in [CONTRIBUTING.md](CONTRIBUTING.md). You'll need Rust and Node.js.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) before opening a pull request.

All contributions are welcome to this project.

## License

Quorum is licensed under [AGPL-3.0](https://www.gnu.org/licenses/agpl-3.0.html). See [LICENSE](LICENSE).

Icons from [Tabler Icons](https://tabler.io/icons) (MIT).