# RestroSync — Server (Axum API)

The backend API server for RestroSync, built with [Axum](https://github.com/tokio-rs/axum) in Rust. Handles REST API requests, database operations, authentication, and real-time communication.

---

## Quick Start

### Prerequisites

- **Rust** 1.77+ (install via [rustup](https://rustup.rs/))
- **Docker** & **Docker Compose** (for PostgreSQL, Redis, NATS, Meilisearch)
- A copy of `.env` based on `.env.example` in the project root

### 1. Start Infrastructure Services

From the **project root**:

```bash
docker compose up -d
```

This starts:

| Service      | Port  | Purpose                        |
|--------------|-------|--------------------------------|
| PostgreSQL   | 5432  | Primary database               |
| Redis        | 6379  | Caching & session store        |
| NATS         | 4222  | Message broker (WebSocket relay)|
| Meilisearch  | 7700  | Full-text search engine        |

Verify all services are healthy:

```bash
docker compose ps
```

### 2. Run Database Migrations

```bash
cargo run --bin migrate
```

This creates all tables: `tenants`, `plans`, `outlets`, `users`, `roles`, `permissions`, `menu_categories`, `menu_items`, `orders`, `bills`, `inventory`, and more.

### 3. Seed Test Data

```bash
cargo run --bin seed
```

Creates a sample restaurant ("Spice Garden") with:
- 1 tenant, 1 outlet
- 4 users (Admin, Cashier, Waiter, Kitchen)
- 5 menu categories, 20 menu items
- 2 floors, 10 tables

### 4. Start the Server

```bash
cargo run --bin server
```

The server starts on `http://127.0.0.1:3001` by default.

---

## Environment Variables

| Variable           | Default                                                        | Description                     |
|--------------------|----------------------------------------------------------------|---------------------------------|
| `SERVER_HOST`      | `127.0.0.1`                                                    | Server bind address             |
| `SERVER_PORT`      | `3001`                                                         | Server bind port                |
| `DATABASE_URL`     | `postgres://restrosync:restrosync_dev@localhost:5432/restrosync`| PostgreSQL connection string    |
| `REDIS_URL`        | `redis://localhost:6379`                                       | Redis connection string         |
| `JWT_SECRET`       | `dev-secret-change-in-production`                              | Secret key for JWT signing      |
| `JWT_EXPIRY_HOURS` | `24`                                                           | Token expiry in hours           |
| `RUST_LOG`         | `info,restrosync=debug`                                        | Log level filter (tracing)      |

> ⚠️ **Never use the default `JWT_SECRET` in production.** Generate a strong random secret.

---

## API Endpoints

### Currently Available (Phase 0)

| Method | Path          | Description          |
|--------|---------------|----------------------|
| `GET`  | `/v1/health`  | Health check         |

### Response Format

All API responses follow a consistent envelope:

**Success:**

```json
{
  "success": true,
  "data": { ... },
  "errors": null,
  "meta": {
    "page": 1,
    "per_page": 20,
    "total": 100,
    "total_pages": 5
  }
}
```

**Error:**

```json
{
  "success": false,
  "data": null,
  "errors": [
    {
      "code": "NOT_FOUND",
      "message": "Resource not found",
      "field": null
    }
  ]
}
```

---

## Testing the API

### Health Check

```bash
curl http://localhost:3001/v1/health
```

Expected response:

```json
{
  "status": "ok",
  "version": "0.1.0",
  "service": "restrosync-server"
}
```

### Using HTTPie (Alternative)

```bash
http GET localhost:3001/v1/health
```

### Database Inspection

Connect to PostgreSQL directly:

```bash
docker exec -it restrosync-postgres psql -U restrosync -d restrosync
```

Useful queries after seeding:

```sql
-- List all tables
\dt

-- Check seed data
SELECT name, slug FROM tenants;
SELECT name, code FROM outlets;
SELECT name, phone FROM users;
SELECT mc.name AS category, mi.name AS item, mi.base_price
  FROM menu_items mi
  JOIN menu_categories mc ON mi.category_id = mc.id
  ORDER BY mc.sort_order, mi.sort_order;
SELECT f.name AS floor, rt.number, rt.capacity
  FROM restaurant_tables rt
  JOIN floors f ON rt.floor_id = f.id
  ORDER BY f.sort_order, rt.number;
```

---

## Project Structure

```
apps/server/
├── src/
│   ├── main.rs              # Server entry point, startup, graceful shutdown
│   ├── lib.rs               # Public module exports (for binary crates)
│   ├── bin/
│   │   └── migrate.rs       # Database migration binary
│   ├── config/
│   │   └── mod.rs           # AppConfig — environment variable loading
│   ├── middleware/
│   │   └── mod.rs           # Request middleware (auth, rate limiting — Phase 1)
│   ├── migrations/
│   │   ├── mod.rs           # SeaORM Migrator definition
│   │   ├── m20250101_000001_create_tenants_and_plans.rs
│   │   ├── m20250101_000002_create_outlets.rs
│   │   ├── m20250101_000003_create_users_and_roles.rs
│   │   ├── m20250101_000004_create_menu.rs
│   │   ├── m20250101_000005_create_tables_and_floors.rs
│   │   ├── m20250101_000006_create_orders.rs
│   │   ├── m20250101_000007_create_billing.rs
│   │   └── m20250101_000008_create_inventory.rs
│   ├── models/
│   │   └── mod.rs           # SeaORM entity definitions (Phase 1)
│   ├── routes/
│   │   ├── mod.rs           # Router assembly, CORS, tracing layers
│   │   └── health.rs        # GET /v1/health
│   ├── services/
│   │   └── mod.rs           # Business logic layer (Phase 1)
│   └── utils/
│       ├── mod.rs
│       └── errors.rs        # AppError type with structured JSON responses
└── Cargo.toml
```

---

## Development Commands

From the **project root** using the justfile:

```bash
just dev-server       # Start server in dev mode
just db-migrate       # Run database migrations
just db-seed          # Seed test data
just db-reset         # Full reset: drop, recreate, migrate, seed
just lint             # Run clippy + cargo fmt check
just test             # Run all Rust tests
```

---

## Upcoming (Phase 1)

- Authentication routes (`POST /v1/auth/login`, `/v1/auth/refresh`)
- Menu CRUD (`/v1/menu/categories`, `/v1/menu/items`)
- Table management (`/v1/tables`, `/v1/floors`)
- Order lifecycle (`/v1/orders`)
- Billing & payments (`/v1/bills`)
- Inventory tracking (`/v1/inventory`)
- WebSocket endpoint for real-time updates