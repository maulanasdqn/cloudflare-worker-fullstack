# Fullstack Cloudflare Worker with Leptos + D1

A fullstack CRUD application built entirely in Rust, running on Cloudflare Workers with a Leptos CSR frontend and D1 SQLite database.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Cloudflare Workers                        │
├─────────────────────────────────────────────────────────────┤
│  worker-api (Rust)          │  Static Assets (worker-ui)    │
│  - Clean Architecture       │  - Leptos CSR WASM bundle     │
│  - Repository Pattern       │  - shadcn-style components    │
│  - D1 database access       │  - Tailwind CSS               │
│  - Rate limiting (KV)       │                               │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
                    ┌─────────────────┐
                    │   Cloudflare D1  │
                    │    (SQLite)      │
                    └─────────────────┘
```

## Tech Stack

| Layer | Technology |
|-------|------------|
| Frontend | Leptos 0.7 (CSR), Tailwind CSS, WASM |
| Backend | Rust, Cloudflare Workers |
| Database | Cloudflare D1 (SQLite) |
| Rate Limiting | Cloudflare KV |
| Validation | zod-rs 0.4 |
| Build Tools | Trunk, worker-build, wrangler |

## Features

- **CRUD Operations** - Create, Read, Update, Delete items
- **Pagination** - Server-side pagination with page controls
- **Search** - Filter items by name or description
- **Sorting** - Click column headers to sort (asc/desc)
- **URL State Sync** - Search, sort, and page state persisted in URL
- **Dark Mode** - shadcn-style light/dark theme toggle
- **Rate Limiting** - KV-backed request limiting (100 req/min)
- **Input Validation** - Schema validation with zod-rs
- **Security Headers** - HSTS, CSP, X-Frame-Options, etc.
- **CORS Support** - Configurable cross-origin requests

## Project Structure

```
cloudflare-worker-fullstack/
├── Cargo.toml                    # Workspace configuration
├── wrangler.toml.example         # Wrangler config template
├── worker-migrations/            # D1 database migrations
│   └── 0001_create_items.sql
├── worker-api/                   # Backend (Clean Architecture)
│   └── src/
│       ├── lib.rs                # Worker entry point
│       ├── errors.rs             # Centralized AppError
│       ├── types.rs              # Response types
│       ├── domain/               # Business entities & traits
│       │   ├── item.rs
│       │   └── repository.rs
│       ├── application/          # Use cases (one per file)
│       │   ├── create_item.rs
│       │   ├── get_item.rs
│       │   ├── list_items.rs
│       │   ├── update_item.rs
│       │   └── delete_item.rs
│       └── infrastructure/
│           ├── http/             # Handlers, DTOs
│           ├── persistence/      # D1 repository impl
│           └── security/         # Rate limit, CORS, validation
└── worker-ui/                    # Frontend (Leptos CSR)
    ├── index.html                # Entry HTML with Tailwind config
    ├── Trunk.toml                # Trunk build config
    └── src/
        ├── main.rs               # WASM entry point
        ├── app.rs                # Main App component
        ├── api.rs                # API client
        ├── types.rs              # Shared types
        └── components/
            ├── item_form.rs      # Create/Edit form
            ├── item_list.rs      # Items table with sorting
            └── ui/               # shadcn-style components
                ├── button.rs
                ├── card.rs
                ├── input.rs
                ├── label.rs
                ├── select.rs
                ├── table.rs
                └── textarea.rs
```

## Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [wasm32-unknown-unknown target](https://rustwasm.github.io/docs/book/): `rustup target add wasm32-unknown-unknown`
- [Trunk](https://trunkrs.dev/): `cargo install trunk`
- [Wrangler](https://developers.cloudflare.com/workers/wrangler/): `npm install -g wrangler`
- Cloudflare account

## Setup

### 1. Clone and configure

```bash
git clone <repo-url>
cd cloudflare-worker-fullstack

# Copy config template
cp wrangler.toml.example wrangler.toml
```

### 2. Create Cloudflare resources

```bash
# Login to Cloudflare
wrangler login

# Create D1 database
wrangler d1 create items-db

# Create KV namespace for rate limiting
wrangler kv namespace create rate-limit-store
```

### 3. Update wrangler.toml

Replace the placeholder IDs with your actual resource IDs:

```toml
[[d1_databases]]
binding = "DB"
database_name = "items-db"
database_id = "<YOUR_D1_DATABASE_ID>"
migrations_dir = "worker-migrations"

[[kv_namespaces]]
binding = "RATE_LIMIT"
id = "<YOUR_KV_NAMESPACE_ID>"
```

### 4. Apply database migrations

```bash
# Local development
wrangler d1 migrations apply items-db --local

# Remote (production)
wrangler d1 migrations apply items-db --remote
```

## Development

### Build frontend

```bash
cd worker-ui
trunk build
```

### Run locally

```bash
# From project root
wrangler dev
```

The app will be available at `http://localhost:8787`

### Watch mode (frontend only)

```bash
cd worker-ui
trunk serve  # Runs on http://localhost:8080
```

## Deployment

### Build and deploy

```bash
# Build frontend first
cd worker-ui && trunk build --release && cd ..

# Deploy to Cloudflare
wrangler deploy
```

### Build command (used by wrangler)

```bash
cargo install -q worker-build && cd worker-api && worker-build --release
```

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/v1/items` | List items (paginated, searchable, sortable) |
| POST | `/api/v1/items` | Create new item |
| GET | `/api/v1/items/:id` | Get single item |
| PUT | `/api/v1/items/:id` | Update item |
| DELETE | `/api/v1/items/:id` | Delete item |

### Query Parameters (GET /api/v1/items)

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| page | u32 | 1 | Page number |
| per_page | u32 | 10 | Items per page |
| search | string | - | Search in name/description |
| sort_by | string | created_at | Sort field (name, created_at) |
| sort_order | string | desc | Sort direction (asc, desc) |

### Example Requests

```bash
# List items
curl https://your-worker.workers.dev/api/v1/items

# Search and sort
curl "https://your-worker.workers.dev/api/v1/items?search=foo&sort_by=name&sort_order=asc"

# Create item
curl -X POST https://your-worker.workers.dev/api/v1/items \
  -H "Content-Type: application/json" \
  -d '{"name": "My Item", "description": "Description"}'

# Update item
curl -X PUT https://your-worker.workers.dev/api/v1/items/1 \
  -H "Content-Type: application/json" \
  -d '{"name": "Updated Name"}'

# Delete item
curl -X DELETE https://your-worker.workers.dev/api/v1/items/1
```

## Clean Architecture Principles

This project follows Clean Architecture with clear separation:

- **Domain Layer** (`domain/`) - Business entities and repository traits. No external dependencies.
- **Application Layer** (`application/`) - Use cases. One file per use case (CreateItem, GetItem, etc.)
- **Infrastructure Layer** (`infrastructure/`) - External concerns (HTTP handlers, D1 repository, security)

### Key Patterns

- **Repository Pattern** - `ItemRepository` trait in domain, `D1ItemRepository` in infrastructure
- **Use Case Pattern** - Single responsibility per use case file
- **Dependency Injection** - `Arc<dyn ItemRepository>` passed to handlers
- **Centralized Error Handling** - `AppError` enum with `IntoResponse` impl

## Security Features

- **Rate Limiting** - 100 requests per minute per IP (KV-backed)
- **Security Headers** - HSTS, X-Content-Type-Options, X-Frame-Options, X-XSS-Protection
- **CORS** - Configurable Access-Control headers
- **Input Validation** - Schema validation with zod-rs
- **CSP** - Content Security Policy for static assets

## License

MIT
