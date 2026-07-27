# Contribution

## Getting Started

### Prerequisites

| Tool | Purpose |
|---|---|
| Rust | Server(`Axum`) + Client (`Tauri`) |
| Node.js + npm/npx | Client (`Tauri`) |

### Running Locally

1. **Start the backend**:
   ```bash
   cd server
   cargo run -p quorum-private
   ```
The server is split into two seperate parts. Private and public server.
By default `cargo run ...` will run in debug. Use `--release` flag to build the release version.

   - **Private Server** - `cargo run -p quorum-private`
   - **Public Server** - `cargo run -p quorum-public`

2. **Start the frontend**:
   ```bash
   cd client
   npm install
   npm run tauri dev
   ```
> npm install is only required to run once just to install node based packages.

---

## Submitting Changes

Contributions are submitted via **GitHub pull requests**.

1. Fork the repository and create a branch from `main`.
2. Make your changes.
3. Ensure your code is formatted (see [Code Style](#code-style)).
4. Open a pull request with a clear title and description of what you changed and why.

Branch naming is flexible, but something descriptive is appreciated — e.g. `fix/auth-jwt-expiry` or `feat/group-invite-codes`.

---

## Code Style

### Rust

Formatting is enforced with `rustfmt`. Before making a PR, run:

```bash
cargo fmt --all
```

Linting with Clippy is also expected. Before making a PR, run:

```bash
cargo clippy -- -W warnings
```
Please ensure that you fix any warnings clippy produces in this command **before** making submitting your PR.

### TypeScript

Formatting is handled by SvelteKit and Vite. Before committing, run:

```bash
npm run check
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

Documentation comments are required for all Rust and TypeScript code. They are not enforced in Svelte/HTML files — comment where it helps, but there is no mandatory format there.

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

**Private functions** follow the same structure but omit `# Example`:

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
| One-line summary | always | always |
| Extended description | where helpful | where helpful |
| `# Errors` | if returns `Result` | if returns `Result` |
| `# Panics` | if can panic | if can panic |
| `# Arguments` | always | always |
| `# Return` | always | always |
| `# Example` | always | omit |

### TypeScript — File Headers

Every `.ts` file must begin with a `/**` doc comment block. The first line is a short one-line description. Leave a blank line in the comment, then expand with more detail.

```typescript
/**
 * Auth token management and validation.
 *
 * Provides utilities for storing, retrieving, and validating JWT tokens.
 * Handles token refresh logic, expiry checks with a 5-minute buffer, and
 * secure storage via the Tauri Store plugin.
 */
```

> Svelte component files (`.svelte`) do not require file headers unless they export utility functions.

### TypeScript — Functions

Every exported function must have a `/**` doc comment with all applicable sections. Use imperative mood in the one-liner ("Get", "Validate", "Retrieve").

**Public functions** require all applicable sections:

```typescript
/**
 * Retrieve the currently stored access token.
 *
 * Returns null if no token is stored or if the store operation fails.
 * Does not validate expiry; use `isTokenValid()` to ensure the token is still active.
 *
 * @param username - The username or email to authenticate.
 * @param password - The user's password.
 * @returns The access token string, or null if not found.
 * @throws {Error} If the store operation fails.
 *
 * @example
 * ```typescript
 * const token = await getAccessToken();
 * if (token) {
 *   console.log('Token exists');
 * }
 * ```
 */
export async function getAccessToken(): Promise<string | null> {
  // ...
}
```

**Private functions** follow the same structure but omit `@example`:

```typescript
/**
 * Decode a JWT token string without verification.
 *
 * Extracts and parses the payload segment of a JWT, returning the decoded
 * claims object. This function does not validate the signature or expiry—
 * use `isTokenValid()` for security-critical checks.
 *
 * @param token - The raw JWT string.
 * @returns The decoded claims object, or null if the token is malformed.
 * @throws {Error} If the payload is not valid JSON.
 */
function decodeToken(token: string): Record<string, unknown> | null {
  // ...
}
```

**Section reference:**

| Section | `export` fn | private fn |
|---|---|---|
| One-line summary | always | always |
| Extended description | always | always |
| `@param` | always | always |
| `@returns` | always | always |
| `@throws` | when applicable | when applicable |
| `@example` | always | omit |

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