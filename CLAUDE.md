# Claude Development Instructions

attio-sfdc is a **pure Rust, zero-dependency** CRM bridge implementing bidirectional sync between Attio and Salesforce. The core philosophy is type safety, edge deployment, and opinionated defaults.

## Commands

```bash
cargo build --release           # Build release binary
cargo build --target wasm32-unknown-unknown  # Build for Cloudflare Workers
cargo test                      # Run all tests
cargo test --lib                # Run unit tests only
cargo test --test integration   # Run integration tests
cargo fmt && cargo clippy       # Format and lint (run before every commit)
cargo bench                     # Run benchmarks
```

### WASM Build

```bash
wrangler dev                    # Local development server
wrangler deploy                 # Deploy to Cloudflare Workers
```

### Testing with Real APIs

```bash
# Set test credentials first
export ATTIO_API_KEY_TEST=xxx
export SALESFORCE_CLIENT_ID_TEST=xxx
cargo test --features real-api  # Run against real APIs (use sparingly)
```

## Architecture

- **`src/attio/`** - Attio API client, types, and object handlers
- **`src/salesforce/`** - Salesforce API client, OAuth, and object handlers
- **`src/sync/`** - Core sync engine, conflict resolution, batching
- **`src/transform/`** - Field transformation pipeline
- **`src/config/`** - Configuration loading and validation
- **`src/storage/`** - Storage abstraction (KV, D1, memory)
- **`src/worker/`** - Cloudflare Worker entry point and routes
- **`src/error.rs`** - Unified error types

### Key Types

```rust
// Core sync types - understand these first
AttioRecord       // Generic Attio record with dynamic attributes
SalesforceRecord  // Generic Salesforce SObject
SyncDirection     // Attio→SF, SF→Attio, Bidirectional
FieldMapping      // Maps Attio field to Salesforce field with transform
IdMapping         // Links Attio ID ↔ Salesforce ID
SyncCursor        // Tracks sync progress for incremental sync
ConflictRecord    // Records requiring manual resolution
```

### Data Flow

```
Webhook/Trigger → Validate → Transform → Resolve References → Upsert → Store Mapping
```

## Key Patterns

### API Clients

Both API clients follow this pattern:

```rust
impl AttioClient {
    pub async fn get_record(&self, object: &str, id: &str) -> Result<AttioRecord>;
    pub async fn list_records(&self, object: &str, filter: Option<Filter>) -> Result<Vec<AttioRecord>>;
    pub async fn create_record(&self, object: &str, data: Value) -> Result<AttioRecord>;
    pub async fn update_record(&self, object: &str, id: &str, data: Value) -> Result<AttioRecord>;
}
```

### Transformations

Use the transform pipeline for field conversions:

```rust
// Built-in transforms
Transform::Direct           // No transformation
Transform::ExtractFirst     // Get first element of array
Transform::ExtractNested(path)  // Extract nested field
Transform::MapValue(map)    // Map via lookup table
Transform::Custom(fn)       // Custom function

// Usage
let transform = Transform::ExtractNested("primary_location.locality");
let result = transform.apply(&attio_value)?;
```

### Error Handling

Always use the custom error type:

```rust
use crate::error::{Error, Result};

// Prefer ? operator with context
let record = client.get_record(id)
    .await
    .map_err(|e| Error::AttioApi {
        operation: "get_record",
        source: e
    })?;
```

### Configuration

Access config through the typed struct:

```rust
let config = Config::from_env()?;
let mapping = config.get_mapping("companies", "accounts")?;
```

## Development Guidelines

### Before Every Change

1. Understand the existing code first - read related modules
2. Check if similar patterns exist in the codebase
3. Write tests before implementation when possible

### Code Style

- Use `rustfmt` defaults - no custom formatting
- Prefer explicit types over inference for public APIs
- Use `#[derive(Debug, Clone, Serialize, Deserialize)]` for data types
- Document public APIs with `///` doc comments
- Use `// TODO:` for known incomplete implementations

### Testing

- Every module should have a corresponding `_test.rs` file
- Use fixtures in `tests/fixtures/` for API responses
- Mock external APIs - never call real APIs in unit tests
- Integration tests can use `wiremock` for HTTP mocking

### Commits

- Run `cargo fmt && cargo clippy && cargo test` before committing
- Keep commits focused - one logical change per commit
- Use conventional commit messages: `feat:`, `fix:`, `refactor:`, `test:`, `docs:`

## Workflow Rules

**Adding a new object mapping?**
1. Add Attio type in `src/attio/objects/`
2. Add Salesforce type in `src/salesforce/objects/`
3. Add default mapping in `src/config/mappings.rs`
4. Add tests in `tests/unit/mapping_test.rs`
5. Update `docs/MAPPING_GUIDE.md`

**Adding a new transform?**
1. Add to `Transform` enum in `src/transform/mod.rs`
2. Implement in `src/transform/builtin.rs`
3. Add comprehensive tests
4. Document in mapping guide

**Modifying sync behavior?**
1. Update `src/sync/engine.rs`
2. Ensure conflict detection still works
3. Run full integration test suite
4. Update `docs/TROUBLESHOOTING.md` if behavior changes

**Changing API endpoint?**
1. Update route in `src/worker/routes.rs`
2. Update OpenAPI spec if we have one
3. Test with `wrangler dev`

## Object Mapping Reference

### Standard Mappings (Immutable Defaults)

| Attio | Salesforce | Sync |
|-------|------------|------|
| Companies | Account | Bidirectional |
| People | Contact | Bidirectional |
| Deals | Opportunity | Bidirectional |
| Users | User | Read-only (SF→Attio) |

### Field Mapping Priority

1. Custom config overrides (highest)
2. Default field mappings
3. Auto-discovery for matching names (lowest)

## Performance Targets

- Single record sync: <200ms
- Batch sync (100 records): <2s
- Webhook processing: <100ms
- Memory usage: <128MB per worker

## Debugging

```bash
# Enable debug logging
RUST_LOG=debug cargo run

# Test specific sync
cargo test sync::engine::tests::test_bidirectional -- --nocapture

# Check WASM size
wasm-opt -Os target/wasm32-unknown-unknown/release/attio_sfdc.wasm -o optimized.wasm
ls -la optimized.wasm
```

## API Documentation

### Attio API
- Docs: https://docs.attio.com/
- Authentication: Bearer token via `Authorization` header
- Rate limits: 100 requests/minute (respect 429 responses)

### Salesforce API
- Docs: https://developer.salesforce.com/docs/atlas.en-us.api_rest.meta/api_rest/
- Authentication: OAuth 2.0 (JWT Bearer or Web Server flow)
- Rate limits: Varies by org edition (check `Sforce-Limit-Info` header)

## Common Issues

### "Record not found" after sync
- Check ID mapping in storage
- Verify the record wasn't deleted in destination
- Check sync cursor hasn't skipped the record

### OAuth token expired
- Salesforce tokens expire after session timeout
- Implement automatic refresh in `src/salesforce/auth.rs`
- Store refresh token securely

### Webhook signature invalid
- Verify webhook secret matches configuration
- Check timestamp isn't too old (replay protection)
- Ensure raw body is used for signature verification

## File Ownership

When modifying these critical files, be extra careful:

- `src/sync/engine.rs` - Core sync logic, heavily tested
- `src/config/mappings.rs` - Default mappings, affects all users
- `src/salesforce/auth.rs` - OAuth security, audit carefully
- `wrangler.toml` - Deployment configuration
