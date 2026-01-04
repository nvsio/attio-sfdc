# Attio-Salesforce Bridge (attio-sfdc)

A high-performance, pure Rust sync engine that bridges Attio CRM and Salesforce with opinionated default mappings and full support for custom objects.

## Project Vision

This project aims to be the definitive open-source solution for bidirectional data synchronization between Attio and Salesforce. Built entirely in Rust for maximum performance and reliability, designed for deployment on Cloudflare Workers (edge-first architecture).

### Core Principles

1. **Zero Runtime Dependencies** - Pure Rust with minimal crates, compiles to WASM for Cloudflare Workers
2. **Opinionated Defaults** - Sensible mappings out of the box, customizable when needed
3. **AI-Assisted Development** - Structured for Claude to build, test, and iterate autonomously
4. **Edge-First** - Designed for Cloudflare Workers, with GCP Cloud Run as fallback
5. **Type-Safe** - Strong typing for all API interactions and data transformations

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                          Cloudflare Workers Edge                             │
├─────────────────────────────────────────────────────────────────────────────┤
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐                   │
│  │   Webhook    │    │    Sync      │    │   Config     │                   │
│  │   Handler    │    │   Engine     │    │   API        │                   │
│  └──────┬───────┘    └──────┬───────┘    └──────┬───────┘                   │
│         │                   │                   │                           │
│         └───────────────────┼───────────────────┘                           │
│                             │                                               │
│                    ┌────────▼────────┐                                      │
│                    │  Mapping Engine │                                      │
│                    │  (Core Logic)   │                                      │
│                    └────────┬────────┘                                      │
│                             │                                               │
│         ┌───────────────────┼───────────────────┐                           │
│         │                   │                   │                           │
│  ┌──────▼───────┐    ┌──────▼───────┐    ┌──────▼───────┐                   │
│  │  Attio API   │    │   Transform  │    │ Salesforce   │                   │
│  │   Client     │    │   Pipeline   │    │  API Client  │                   │
│  └──────────────┘    └──────────────┘    └──────────────┘                   │
├─────────────────────────────────────────────────────────────────────────────┤
│                        Cloudflare KV / D1 / Durable Objects                 │
│                        (State, Mappings, Sync Cursors)                      │
└─────────────────────────────────────────────────────────────────────────────┘
         │                                               │
         ▼                                               ▼
    ┌─────────┐                                    ┌──────────┐
    │  Attio  │                                    │Salesforce│
    │   API   │                                    │   API    │
    └─────────┘                                    └──────────┘
```

---

## Opinionated Data Mapping

### Standard Object Mappings

| Attio Object | Salesforce Object | Notes |
|--------------|-------------------|-------|
| **Companies** | **Account** | Primary business entity |
| **People** | **Contact** | Individual contacts |
| **Deals** | **Opportunity** | Sales opportunities |
| **Users** | **User** | System users (read-only sync) |

### Default Field Mappings

#### Companies → Account

| Attio Field | Salesforce Field | Transform |
|-------------|------------------|-----------|
| `name` | `Name` | Direct |
| `domains[0].domain` | `Website` | Extract primary domain |
| `description` | `Description` | Direct |
| `primary_location.locality` | `BillingCity` | Extract from location |
| `primary_location.region` | `BillingState` | Extract from location |
| `primary_location.country_code` | `BillingCountry` | Country code to name |
| `primary_location.postcode` | `BillingPostalCode` | Direct |
| `categories` | `Industry` | Map to SF industry picklist |
| `employee_range` | `NumberOfEmployees` | Parse range to number |
| `estimated_arr_usd` | `AnnualRevenue` | Direct |

#### People → Contact

| Attio Field | Salesforce Field | Transform |
|-------------|------------------|-----------|
| `name.first_name` | `FirstName` | Direct |
| `name.last_name` | `LastName` | Direct |
| `email_addresses[0].email_address` | `Email` | Primary email |
| `phone_numbers[0].phone_number` | `Phone` | Primary phone |
| `job_title` | `Title` | Direct |
| `primary_location.locality` | `MailingCity` | Extract |
| `primary_location.region` | `MailingState` | Extract |
| `primary_location.country_code` | `MailingCountry` | Map |
| `company` (record reference) | `AccountId` | Lookup via sync mapping |
| `linkedin_url` | `LinkedIn__c` | Custom field (optional) |

#### Deals → Opportunity

| Attio Field | Salesforce Field | Transform |
|-------------|------------------|-----------|
| `name` | `Name` | Direct |
| `value.value` | `Amount` | Extract numeric value |
| `value.currency_code` | `CurrencyIsoCode` | Direct (multi-currency) |
| `status` | `StageName` | Map to SF stages |
| `expected_close_date` | `CloseDate` | Direct |
| `associated_people[0]` | `ContactId` | Primary contact lookup |
| `associated_companies[0]` | `AccountId` | Primary account lookup |
| `deal_source` | `LeadSource` | Map to picklist |
| `probability` | `Probability` | Direct percentage |

### Status → Stage Mapping (Deals)

```rust
// Default status mapping (configurable)
const DEFAULT_STAGE_MAP: &[(&str, &str)] = &[
    ("open", "Prospecting"),
    ("in_progress", "Qualification"),
    ("won", "Closed Won"),
    ("lost", "Closed Lost"),
];
```

### Custom Object Support

The bridge supports syncing custom Attio objects to custom Salesforce objects:

```toml
# config.toml
[[custom_mappings]]
attio_object = "projects"
salesforce_object = "Project__c"

[[custom_mappings.fields]]
attio = "project_name"
salesforce = "Name"

[[custom_mappings.fields]]
attio = "budget.value"
salesforce = "Budget__c"
transform = "currency_to_number"
```

---

## Project Structure

```
attio-sfdc/
├── CLAUDE.md                    # AI development instructions
├── AGENTS.md                    # Agent-specific guidelines
├── Cargo.toml                   # Rust workspace config
├── wrangler.toml                # Cloudflare Workers config
├── config.example.toml          # Example configuration
├── .claude/
│   └── hooks/                   # Claude Code hooks
│       ├── pre-commit.sh
│       ├── post-edit.sh
│       └── test-runner.sh
├── src/
│   ├── lib.rs                   # Library root
│   ├── config/
│   │   ├── mod.rs               # Configuration module
│   │   ├── mappings.rs          # Field mapping definitions
│   │   └── validation.rs        # Config validation
│   ├── attio/
│   │   ├── mod.rs               # Attio module root
│   │   ├── client.rs            # HTTP client for Attio API
│   │   ├── types.rs             # Attio type definitions
│   │   ├── objects/
│   │   │   ├── mod.rs
│   │   │   ├── companies.rs     # Companies object
│   │   │   ├── people.rs        # People object
│   │   │   ├── deals.rs         # Deals object
│   │   │   └── custom.rs        # Custom object handler
│   │   └── webhooks.rs          # Webhook payload parsing
│   ├── salesforce/
│   │   ├── mod.rs               # Salesforce module root
│   │   ├── client.rs            # HTTP client for SF API
│   │   ├── auth.rs              # OAuth 2.0 flow
│   │   ├── types.rs             # Salesforce type definitions
│   │   ├── objects/
│   │   │   ├── mod.rs
│   │   │   ├── account.rs       # Account object
│   │   │   ├── contact.rs       # Contact object
│   │   │   ├── opportunity.rs   # Opportunity object
│   │   │   └── custom.rs        # Custom object handler
│   │   └── bulk.rs              # Bulk API operations
│   ├── sync/
│   │   ├── mod.rs               # Sync engine root
│   │   ├── engine.rs            # Core sync orchestration
│   │   ├── direction.rs         # Sync direction handling
│   │   ├── conflict.rs          # Conflict resolution
│   │   ├── cursor.rs            # Sync cursor management
│   │   └── batch.rs             # Batch processing
│   ├── transform/
│   │   ├── mod.rs               # Transform pipeline
│   │   ├── field.rs             # Field transformers
│   │   ├── reference.rs         # Reference resolution
│   │   └── builtin.rs           # Built-in transforms
│   ├── storage/
│   │   ├── mod.rs               # Storage abstraction
│   │   ├── kv.rs                # Cloudflare KV adapter
│   │   ├── d1.rs                # Cloudflare D1 adapter
│   │   └── memory.rs            # In-memory (testing)
│   ├── worker/
│   │   ├── mod.rs               # Worker entry point
│   │   ├── routes.rs            # HTTP route handlers
│   │   └── middleware.rs        # Auth & logging middleware
│   └── error.rs                 # Error types
├── tests/
│   ├── integration/
│   │   ├── attio_client_test.rs
│   │   ├── salesforce_client_test.rs
│   │   ├── sync_engine_test.rs
│   │   └── transform_test.rs
│   ├── unit/
│   │   ├── mapping_test.rs
│   │   ├── conflict_test.rs
│   │   └── cursor_test.rs
│   └── fixtures/
│       ├── attio/               # Attio API response fixtures
│       └── salesforce/          # Salesforce API response fixtures
├── benches/
│   ├── transform_bench.rs
│   └── sync_bench.rs
├── docs/
│   ├── MAPPING_GUIDE.md         # Field mapping documentation
│   ├── DEPLOYMENT.md            # Deployment instructions
│   ├── CUSTOM_OBJECTS.md        # Custom object setup
│   └── TROUBLESHOOTING.md       # Common issues
└── scripts/
    ├── setup.sh                 # Initial setup script
    ├── deploy-cf.sh             # Cloudflare deployment
    └── test-sync.sh             # Manual sync testing
```

---

## Technology Stack

### Core

- **Language**: Rust 2021 Edition
- **Target**: `wasm32-unknown-unknown` (Cloudflare Workers)
- **HTTP Client**: `reqwest` with WASM support or custom fetch bindings
- **Serialization**: `serde` + `serde_json`
- **Async Runtime**: `tokio` (native) / Workers runtime (WASM)

### Cloudflare Services

- **Workers**: Compute layer (WASM)
- **KV**: Sync cursors, ID mappings, configuration cache
- **D1**: Detailed sync history, audit logs (SQLite)
- **Durable Objects**: Distributed locks, rate limiting state
- **Queues**: Async webhook processing, retry handling

### Dependencies (Minimal)

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
worker = "0.4"                    # Cloudflare Workers SDK
chrono = { version = "0.4", features = ["serde"] }
thiserror = "2.0"
uuid = { version = "1.0", features = ["v4", "serde"] }
base64 = "0.22"
hmac = "0.12"
sha2 = "0.10"

[dev-dependencies]
tokio = { version = "1", features = ["full"] }
wiremock = "0.6"                  # HTTP mocking
pretty_assertions = "1.4"
criterion = "0.5"
```

---

## API Endpoints

### Webhook Endpoints

```
POST /webhooks/attio          # Receive Attio webhooks
POST /webhooks/salesforce     # Receive Salesforce webhooks
```

### Management API

```
GET  /api/v1/status           # Sync status and health
GET  /api/v1/mappings         # Current field mappings
PUT  /api/v1/mappings         # Update field mappings
POST /api/v1/sync             # Trigger manual sync
GET  /api/v1/sync/:id         # Get sync job status
GET  /api/v1/history          # Sync history
GET  /api/v1/conflicts        # Unresolved conflicts
POST /api/v1/conflicts/:id    # Resolve conflict
```

### Configuration API

```
GET  /api/v1/config           # Current configuration
PUT  /api/v1/config           # Update configuration
POST /api/v1/validate         # Validate configuration
```

---

## Deployment Options

### Primary: Cloudflare Workers (Recommended)

```bash
# Deploy to Cloudflare
npm install -g wrangler
wrangler login
wrangler deploy
```

**Advantages:**
- Global edge deployment (300+ locations)
- No cold starts with Workers
- Built-in KV, D1, Durable Objects
- Free tier: 100k requests/day
- Sub-50ms latency globally

### Secondary: Google Cloud Run

```bash
# Build container
docker build -t attio-sfdc .

# Deploy to Cloud Run
gcloud run deploy attio-sfdc \
  --image attio-sfdc \
  --region us-central1 \
  --allow-unauthenticated
```

**Use when:**
- Need longer execution times (>30s)
- Heavy batch processing requirements
- Existing GCP infrastructure

---

## Configuration

### Environment Variables

```bash
# Required
ATTIO_API_KEY=attio_xxx
SALESFORCE_CLIENT_ID=xxx
SALESFORCE_CLIENT_SECRET=xxx
SALESFORCE_INSTANCE_URL=https://yourorg.my.salesforce.com

# Optional
SYNC_DIRECTION=bidirectional    # attio_to_sf, sf_to_attio, bidirectional
CONFLICT_RESOLUTION=last_write  # last_write, attio_wins, sf_wins, manual
BATCH_SIZE=100
LOG_LEVEL=info
```

### Configuration File

```toml
# config.toml
[sync]
direction = "bidirectional"
batch_size = 100
conflict_resolution = "last_write"

[attio]
api_key_env = "ATTIO_API_KEY"
webhook_secret_env = "ATTIO_WEBHOOK_SECRET"

[salesforce]
client_id_env = "SALESFORCE_CLIENT_ID"
client_secret_env = "SALESFORCE_CLIENT_SECRET"
instance_url_env = "SALESFORCE_INSTANCE_URL"

[mappings.companies_to_accounts]
enabled = true
# Override default field mappings
[[mappings.companies_to_accounts.fields]]
attio = "custom_field"
salesforce = "Custom_Field__c"

[mappings.people_to_contacts]
enabled = true

[mappings.deals_to_opportunities]
enabled = true
# Custom stage mapping
[mappings.deals_to_opportunities.stages]
"new" = "Prospecting"
"qualified" = "Qualification"
"proposal" = "Proposal/Price Quote"
"negotiation" = "Negotiation/Review"
"won" = "Closed Won"
"lost" = "Closed Lost"
```

---

## Sync Behavior

### Sync Modes

1. **Real-time (Webhook-driven)**
   - Immediate sync on record changes
   - ~100ms latency
   - Best for active records

2. **Scheduled (Cron)**
   - Periodic full/incremental sync
   - Catches missed webhooks
   - Handles bulk changes

3. **Manual (API-triggered)**
   - On-demand sync for specific records
   - Useful for initial migration
   - Conflict resolution workflows

### Conflict Resolution Strategies

```rust
pub enum ConflictResolution {
    /// Most recent write wins (uses updated_at timestamps)
    LastWrite,
    /// Attio is source of truth
    AttioWins,
    /// Salesforce is source of truth
    SalesforceWins,
    /// Queue for manual resolution
    Manual,
    /// Custom merge function
    Custom(fn(AttioRecord, SalesforceRecord) -> MergedRecord),
}
```

### ID Mapping Storage

```sql
-- D1 Schema for ID mappings
CREATE TABLE id_mappings (
    id TEXT PRIMARY KEY,
    attio_object TEXT NOT NULL,
    attio_id TEXT NOT NULL,
    salesforce_object TEXT NOT NULL,
    salesforce_id TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE(attio_object, attio_id),
    UNIQUE(salesforce_object, salesforce_id)
);

CREATE INDEX idx_attio ON id_mappings(attio_object, attio_id);
CREATE INDEX idx_salesforce ON id_mappings(salesforce_object, salesforce_id);
```

---

## Testing Strategy

### Unit Tests
- Field transformation logic
- Mapping resolution
- Conflict detection
- Configuration validation

### Integration Tests
- API client behavior (mocked)
- Full sync pipeline
- Webhook processing

### End-to-End Tests
- Real API interactions (separate test accounts)
- Full bidirectional sync
- Error recovery

### Test Coverage Target: 85%+

---

## Development Phases

### Phase 1: Foundation (Core Infrastructure)
- [ ] Project scaffolding with Cargo workspace
- [ ] Attio API client with authentication
- [ ] Salesforce API client with OAuth 2.0
- [ ] Basic type definitions for standard objects
- [ ] Configuration loading and validation

### Phase 2: Mapping Engine
- [ ] Field mapping definitions
- [ ] Transform pipeline implementation
- [ ] Reference resolution (foreign keys)
- [ ] Custom field support

### Phase 3: Sync Engine
- [ ] Unidirectional sync (Attio → Salesforce)
- [ ] Unidirectional sync (Salesforce → Attio)
- [ ] Bidirectional sync with conflict detection
- [ ] Cursor-based incremental sync

### Phase 4: Webhook Integration
- [ ] Attio webhook receiver
- [ ] Salesforce Platform Events / Change Data Capture
- [ ] Real-time sync triggering
- [ ] Retry and error handling

### Phase 5: Storage & State
- [ ] Cloudflare KV integration
- [ ] Cloudflare D1 integration
- [ ] Sync history and audit logging
- [ ] ID mapping persistence

### Phase 6: Custom Objects
- [ ] Dynamic object discovery (Attio)
- [ ] Dynamic object discovery (Salesforce)
- [ ] Custom object mapping configuration
- [ ] Schema validation

### Phase 7: Production Hardening
- [ ] Comprehensive error handling
- [ ] Rate limiting and backoff
- [ ] Monitoring and alerting hooks
- [ ] Performance optimization

### Phase 8: Documentation & Polish
- [ ] API documentation
- [ ] Deployment guides
- [ ] Example configurations
- [ ] Troubleshooting guide

---

## Success Metrics

1. **Reliability**: 99.9% sync success rate
2. **Latency**: <500ms for webhook-triggered syncs
3. **Scalability**: Handle 10k+ records/minute
4. **Test Coverage**: 85%+ code coverage
5. **Developer Experience**: <5 minutes to deploy

---

## License

MIT License - Free for commercial and personal use.
