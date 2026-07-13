
# Contribution
  
## Project Structure

```
quorum/
├── client/        # Tauri 2 + SvelteKit frontend
└── server/        # Rust + Axum backend
```

---

## Getting Started

### Prerequisites

| Tool | Purpose |
|---|---|
| Rust (stable) | Backend (`server/`) |
| Node.js + npm/npx | Frontend (`client/`) |

### Running Locally

1. **Start the backend**:
   ```bash
   cd server
   cargo run -p quorum-private
   ```
   > Use `cargo run -p quorum-private --release` for production mode.
   > Or use `cargo run -p quorum-public` to build the public server.

2. **Start the frontend**:
   ```bash
   cd client
   npm install
   npm run tauri dev
   ```

---

## Submitting Changes

Contributions are submitted via **GitHub pull requests**. There is no requirement to open an issue first before a pull request is made.

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
cargo fmt --all
```

Linting with Clippy is also expected:

```bash
cargo clippy -- -W warnings
```

### SurrealQL (`initial.surql`)

The following conventions apply to all `.surql` files, these files are created when there are large amount of queries to run.

**Naming**
- Params: `SCREAMING_SNAKE_CASE` (e.g. `$MAX_USERNAME_BYTES`)
- Tables and fields: `snake_case` (e.g. `user_settings`, `password_hash`)

**File structure**
- Open with a reference block of `--` comments if the file defines params or constants that benefit from explanation (e.g. byte size tables).
- Group related definitions under a section banner:
  ```surql
  -- =========================================================
  -- SECTION NAME
  -- =========================================================
  ```
- Within a section, order each table block as: `DEFINE TABLE` → fields → indexes.
- Leave a blank line between each `DEFINE TABLE`, `DEFINE INDEX` and **two** blank lines between each table definition or block.
- Use `IF NOT EXISTS` on all definitions unless `OVERWRITE` is explicitly needed.

**Example**
```surql
-- Users
DEFINE TABLE IF NOT EXISTS users SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS username ON TABLE users TYPE string
    ASSERT fn::bytes_within($value, $MIN_DEFAULT_BYTES, $MAX_USERNAME_BYTES);
DEFINE FIELD IF NOT EXISTS email ON TABLE users TYPE string
    ASSERT fn::bytes_within($value, $MIN_DEFAULT_BYTES, $MAX_EMAIL_BYTES);

DEFINE INDEX IF NOT EXISTS idx_username ON TABLE users COLUMNS username UNIQUE;
DEFINE INDEX IF NOT EXISTS idx_email ON TABLE users COLUMNS email UNIQUE;
```

---

## Documentation Comments

Documentation comments are required for all Rust code. They are not enforced in Svelte/JS files — comment where it helps, but there is no mandatory format there.

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
