# Contribution

---

## Table of Contents

- [Project Structure](#project-structure)
- [Getting Started](#getting-started)
- [Submitting Changes](#submitting-changes)
- [Code Style](#code-style)
- [Documentation Comments](#documentation-comments)
- [Commit Messages](#commit-messages)

---

## Project Structure

```
quorum/
├── client/        # Tauri 2 + SvelteKit frontend
├── docker/        # Docker Compose for SurrealDB and MinIO
├── server/        # Rust + Axum backend
├── schema/        # SurrealQL schema files
└── docker/        # Docker Compose for SurrealDB and MinIO
```

---

## Getting Started

### Prerequisites

| Tool | Purpose |
|---|---|
| Rust (stable) | Backend (`server/`) |
| Node.js + npm/npx | Frontend (`client/`) |
| Docker + Docker Compose | SurrealDB + MinIO (`docker/`) |

### Running locally

1. Start the database and file storage:
   ```bash
   cd docker
   docker compose up -d
   ```

2. Start the backend:
   ```bash
   cd server
   cargo run
   ```
   OR
   ```bash
    cd server
    cargo run dev
    ```

3. Start the frontend:
   ```bash
   cd client
   npm run tauri
   ```
   OR
   ```bash
   cd client
   npm run tauri dev
   ```

---

## Submitting Changes

Contributions are submitted via **GitHub pull requests**. There is no requirement to open an issue first.

1. Fork the repository and create a branch from `main`.
2. Make your changes.
3. Ensure your code is formatted (see [Code Style](#code-style)).
4. Open a pull request with a clear title and description of what you changed and why.

Branch naming is flexible, but something descriptive is appreciated — e.g. `fix/auth-jwt-expiry` or `feat/group-invite-codes`.

---

## Code Style

### Rust (`server/`)

Formatting is enforced with `rustfmt`. Before committing, run:

```bash
cargo fmt
```

Linting with Clippy is also expected:

```bash
cargo clippy -- -D warnings
```

### SurrealQL (`schema/`)

There is no automated formatter for SurrealQL. Keep definitions consistently indented (2 spaces) and add a blank line between each `DEFINE` statement.

### Docker / infra (`docker/`)

Keep `docker-compose.yml` clean and commented where non-obvious configuration is used.

---

## Documentation Comments

Documentation comments are required in the Rust backend. They are not enforced in Svelte/JS files — comment where it helps, but there is no mandatory format there.

### Rust — File Headers

Every `.rs` file (excluding `mod.rs`) must begin with a `//!` doc comment block. The first line is a short one-line description. Leave a blank `//!` on the second line, then expand with more detail from the third line onward.

```rust
//! Auth middleware: validate and extract JWT claims from incoming requests.
//!
//! This module provides an Axum extractor that reads the `Authorization` header,
//! verifies the JWT signature against the configured secret, and rejects requests
//! with expired or malformed tokens before they reach route handlers.
```

### Rust — Functions

Every function must have a `///` doc comment. The structure depends on whether the function is `pub` or not.

**Public functions** require all applicable sections:

```rust
/// Verify a JWT token string and return the decoded claims.
///
/// Decodes and validates the token signature, expiry, and issuer claim.
/// Returns the inner `Claims` struct on success so callers can access
/// the user ID and granted scopes without re-parsing the token.
///
/// # Errors
///
/// Returns an `Err` when:
/// - the token is malformed or cannot be decoded,
/// - the signature does not match the configured secret,
/// - the token has expired,
/// - or required claims (`sub`, `exp`) are missing.
///
/// # Panics
///
/// Does not panic under normal operation. Will panic if the JWT secret
/// has not been loaded into the application state (misconfiguration).
///
/// # Arguments
/// * `token` - The raw JWT string, typically extracted from the `Authorization` header.
/// * `secret` - The HMAC secret used to verify the token signature.
///
/// # Return
/// Returns `Claims` on success containing the user ID and token metadata.
/// Returns an error if validation fails for any reason.
///
/// # Example
/// ```rust
/// let claims = verify_token(&token, &secret)?;
/// println!("Authenticated user: {}", claims.sub);
/// ```
pub fn verify_token(token: &str, secret: &[u8]) -> Result<Claims, AuthError> {
    // ...
}
```

**Private functions** follow the same structure but omit `# Example` (and `# Safety` unless the function is `unsafe`):

```rust
/// Strip the `Bearer ` prefix from a raw Authorization header value.
///
/// Returns the token string slice without the prefix, or an error
/// if the header value does not start with `Bearer `.
///
/// # Errors
///
/// Returns an `Err` if the header value is missing the `Bearer ` prefix
/// or is otherwise malformed.
///
/// # Arguments
/// * `header` - The raw value of the `Authorization` header.
///
/// # Return
/// Returns a `&str` slice pointing to the token portion of the header.
fn extract_bearer(header: &str) -> Result<&str, AuthError> {
    // ...
}
```

**Section reference:**

| Section | `pub` fn | private fn |
|---|---|---|
| One-line summary | ✅ always | ✅ always |
| Extended description | where helpful | where helpful |
| `# Errors` | if returns `Result` | if returns `Result` |
| `# Panics` | if can panic | if can panic |
| `# Safety` | if `unsafe` | if `unsafe` |
| `# Arguments` | ✅ always | ✅ always |
| `# Return` | ✅ always | ✅ always |
| `# Example` | ✅ always | ❌ omit |

---

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