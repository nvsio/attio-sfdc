# attio-sfdc

A high-performance Rust bridge for bidirectional sync between Attio CRM and Salesforce.

## Features

- **Bidirectional Sync** - Real-time and scheduled sync between Attio and Salesforce
- **Opinionated Defaults** - Sensible mappings for standard objects (Companies↔Account, People↔Contact, Deals↔Opportunity)
- **Custom Object Support** - Sync custom Attio objects to custom Salesforce objects
- **Edge-First** - Designed for Cloudflare Workers with sub-50ms global latency
- **Conflict Resolution** - Multiple strategies: last-write-wins, source-of-truth, or manual
- **Type-Safe** - Strong typing for all API interactions

## Quick Start

### Deploy to Cloudflare Workers

```bash
# Install dependencies
npm install -g wrangler

# Configure secrets
wrangler secret put ATTIO_API_KEY
wrangler secret put SALESFORCE_CLIENT_ID
wrangler secret put SALESFORCE_CLIENT_SECRET
wrangler secret put SALESFORCE_INSTANCE_URL

# Deploy
wrangler deploy
```

### Local Development

```bash
# Build
cargo build --release

# Run tests
cargo test

# Start local dev server
wrangler dev
```

## Configuration

Set these environment variables:

| Variable | Required | Description |
|----------|----------|-------------|
| `ATTIO_API_KEY` | Yes | Your Attio API key |
| `SALESFORCE_CLIENT_ID` | Yes | Salesforce OAuth client ID |
| `SALESFORCE_CLIENT_SECRET` | Yes | Salesforce OAuth client secret |
| `SALESFORCE_INSTANCE_URL` | Yes | Your Salesforce instance URL |
| `SYNC_DIRECTION` | No | `bidirectional`, `attio_to_sf`, or `sf_to_attio` |
| `CONFLICT_RESOLUTION` | No | `last_write`, `attio_wins`, `sf_wins`, or `manual` |

## Default Object Mappings

| Attio | Salesforce | Direction |
|-------|------------|-----------|
| Companies | Account | Bidirectional |
| People | Contact | Bidirectional |
| Deals | Opportunity | Bidirectional |

## API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/health` | GET | Health check |
| `/api/v1/status` | GET | Sync status |
| `/api/v1/sync` | POST | Trigger manual sync |
| `/api/v1/conflicts` | GET | List unresolved conflicts |
| `/webhooks/attio` | POST | Attio webhook receiver |
| `/webhooks/salesforce` | POST | Salesforce webhook receiver |

## Architecture

```
┌─────────────────────────────────────────┐
│         Cloudflare Workers Edge         │
├─────────────────────────────────────────┤
│  Webhook Handler → Sync Engine → APIs   │
├─────────────────────────────────────────┤
│     KV (Mappings) │ D1 (History)        │
└─────────────────────────────────────────┘
         ↓                    ↓
    Attio API          Salesforce API
```

## Development

```bash
# Format code
cargo fmt

# Lint
cargo clippy

# Run specific tests
cargo test sync::engine

# Build for WASM
cargo build --target wasm32-unknown-unknown --release
```

## License

MIT License - see [LICENSE](LICENSE) for details.
