﻿# Contribution

---

## Getting Started

### Prerequisites

| Tool |
|---|
| Rust (stable)

### Running locally

Use `cargo run dev` or `cargo run --release` to run this.
---

## Submitting Changes

Contributions are submitted via **GitHub pull requests**. There is no requirement to open an issue first.

1. Fork the repository and create a branch from `Homomorphic-encryption`.
2. Make your changes.
3. Ensure your code is formatted (see [Code Style](#code-style)).
4. Open a pull request with a clear title and description of what you changed and why.

Branch naming is flexible, but something descriptive is appreciated — e.g. `fix/auth-jwt-expiry` or `feat/group-invite-codes`.

---

## Code Style

### Rust (`server/`)

Formatting is enforced with `rustfmt`. Before committing, run:

```bash
cargo fmt --all
```

Linting with Clippy is also expected:

```bash
cargo clippy -- -W warnings
```

## Commit Messages

Use clear, imperative commit messages. No strict convention is enforced, but keep them descriptive enough that the history is readable.

```
# Good
Add JWT expiry validation to auth middleware
Fix group invite code collision on short IDs
Refactor SurrealDB connection into shared adapter trait

# Avoid
fix stuff
wip
changes
```
