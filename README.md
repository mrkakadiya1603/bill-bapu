<p align="center">
  <img src="https://img.shields.io/badge/Status-In%20Development-orange?style=for-the-badge" alt="Status" />
  <img src="https://img.shields.io/badge/Version-0.1.0--alpha-blue?style=for-the-badge" alt="Version" />
  <img src="https://img.shields.io/badge/License-Proprietary-red?style=for-the-badge" alt="License" />
</p>

# 🍽️ RestroSync

### *Next-Generation Restaurant Management Platform*

> **Empowering every restaurant — from a single chai stall to a 50-outlet chain — with enterprise-grade management software that works everywhere, online or offline, without compromising speed, privacy, or simplicity.**

RestroSync is a full-featured restaurant management platform built to directly compete with and surpass [Petpooja](https://www.petpooja.com/) in the Indian market. It features **fully dynamic networking** that auto-switches between Cloud, Local WiFi, and Standalone modes — something no competitor offers today.

---

## 📋 Table of Contents

- [Why RestroSync?](#-why-restrosync)
- [Key Features](#-key-features)
- [How Dynamic Networking Works](#-how-dynamic-networking-works)
- [Multi-Device Flow](#-multi-device-flow)
- [Tech Stack](#-tech-stack)
- [Architecture Overview](#-architecture-overview)
- [Project Structure](#-project-structure)
- [Getting Started](#-getting-started)
- [Core Modules](#-core-modules)
- [Hardware Requirements](#-hardware-requirements)
- [Roadmap](#-roadmap)
- [Contributing](#-contributing)
- [Documentation](#-documentation)
- [License](#-license)

---

## 🤔 Why RestroSync?

| Problem with existing solutions | RestroSync's answer |
|---|---|
| 🔴 **Petpooja breaks without internet** — waiters can't take orders, kitchen can't see KOTs | ☁️📡💾 **Auto-switches** between Cloud, Local WiFi, and Standalone. Never breaks. |
| 🔴 **No multi-device without internet** — waiter phones stop working when ISP goes down | 📡 **Embedded local server** on Cashier PC — waiters & kitchen work on local WiFi, zero internet needed |
| 🔴 **Slow on old hardware** — Electron-based apps eat RAM and CPU | ⚡ **Rust + Tauri** — 10x smaller, 5x less RAM, instant startup |
| 🔴 **No privacy option** — all your data goes to their cloud | 🔒 **Full local-only mode** — your data never leaves your PC |
| 🔴 **Expensive** — ₹1,000-5,000/month even for small restaurants | 💰 **Free tier** + affordable paid plans |
| 🔴 **Rigid UI** — one-size-fits-all | 🎨 **Adaptive interface** — simple for a tea stall, powerful for a chain |

---

## ✨ Key Features

### 🧾 Billing System
- Quick billing with GST auto-calculation (CGST/SGST/IGST)
- Bill splitting (by amount, percentage, items, per person)
- Multiple payment modes (Cash, Card, UPI, Wallet, Split)
- Thermal printer support (58mm/80mm) + A4 invoice
- Discounts (flat, percentage, item-level, bill-level, coupon)
- KOT (Kitchen Order Ticket) auto-generation
- Credit/advance management
- Configurable service charge & round-off

### 📦 Inventory Management
- Raw material tracking with unit conversions
- **Recipe-level inventory** — auto-deduct stock when a bill is generated
- Low stock alerts with configurable thresholds
- Vendor management & purchase orders
- Wastage tracking & stock valuation (FIFO / Weighted Average)
- Manual stock adjustments with audit trail

### 🍕 Menu Management
- Hierarchical categories & sub-categories
- Variants (Half/Full, S/M/L) with separate pricing
- Modifier groups & add-ons (Extra Cheese, Choose Crust, etc.)
- Veg/Non-veg/Egg/Vegan labels
- Real-time availability toggle
- Item images & preparation time estimates
- GST rate per item (5% / 12% / 18%)

### 📋 Order Management
- Dine-in, Takeaway, and Delivery order types
- **Multi-device ordering** — waiters take orders on their phones
- **Kitchen Display System** — kitchen tablet shows live KOTs
- Real-time order status: Placed → Preparing → Ready → Served → Billed
- Multiple KOTs per order
- Order modification with audit log

### 📊 Reporting & Analytics
- Daily/weekly/monthly sales reports
- Item-wise & category-wise sales analysis
- Payment mode breakdown
- GST reports (GSTR-1, GSTR-3B compatible)
- Inventory & purchase reports
- Staff performance reports
- Export to PDF & Excel

### 🪑 Table Management
- Visual drag-and-drop floor plan editor
- Multiple floors/sections (AC, Outdoor, VIP)
- Real-time table status (Available → Occupied → Billing → Cleaning)
- Table merge, split, and transfer
- Basic reservation system
- Waiter assignment to tables

### 🔐 Authentication & RBAC
- Role-based access control (Owner, Manager, Cashier, Waiter, Kitchen Staff, Viewer)
- 50+ granular permissions
- JWT + Argon2 authentication
- Optional 2FA (TOTP)

---

## 🌐 How Dynamic Networking Works

RestroSync's **killer feature** — the app auto-detects your connectivity and seamlessly switches between three modes. **The user never configures anything.**

```
App starts → Checks connectivity every 5 seconds
     │
     ├── Can reach Cloud server? ──YES──→ ☁️  CLOUD MODE
     │                                        All devices talk to our cloud
     │                                        Owner can monitor from home
     │                                        Local SQLite keeps a fast copy
     │
     ├── WiFi available? ──YES──→ 📡 LOCAL NETWORK MODE
     │                               Cashier PC becomes the server
     │                               Waiters & kitchen connect via WiFi
     │                               NO internet needed
     │                               Changes queued for cloud sync later
     │
     └── Nothing? ──→ 💾 STANDALONE MODE
                         Single device operation
                         Everything saved locally
                         Syncs when any connection returns
```

### What the user sees

| Icon | Mode | Meaning |
|------|------|---------|
| ☁️ 🟢 | Cloud | Connected to cloud. Full features. |
| 📡 🔵 | Local Network | No internet. Working on local WiFi. All devices connected. |
| 💾 🟡 | Standalone | No network. Single device mode. |
| ☁️ 🔄 | Syncing | Internet returned. Syncing queued data in background. |

> **The Golden Rule:** Every write goes to **local SQLite first** (instant, never fails). Cloud sync is a bonus layer. This guarantees < 100ms response time and zero downtime.

### Real-World Example

```
Cafe in Mumbai — has internet, WiFi, 3 waiters, 1 kitchen display

  9:00 AM  →  Internet working  →  ☁️ CLOUD MODE
              Owner checks sales from home ✅

  2:15 PM  →  ISP goes down     →  📡 LOCAL NETWORK MODE (auto-switch in < 2 sec)
              Waiters keep taking orders on their phones
              Kitchen keeps seeing KOTs
              Nobody notices the switch ⚡

  3:45 PM  →  Internet returns   →  ☁️ CLOUD MODE + background sync
              90 minutes of data syncs to cloud silently
              Owner sees all data appear on web dashboard
```

---

## 📱 Multi-Device Flow

```
┌──────────────┐     ┌──────────────────────┐     ┌───────────────────┐
│   WAITER      │     │     SERVER            │     │   KITCHEN         │
│   (Phone)     │     │  (Cloud or Cashier PC)│     │   (Tablet)        │
│               │     │                       │     │                   │
│  Browses menu │     │                       │     │                   │
│  Selects items│────▶│  Creates order        │────▶│  🔔 NEW KOT      │
│  Taps "Send"  │     │  Generates KOT        │     │  Table 5:         │
│               │     │  Saves to DB          │     │  1x Butter Chicken│
│               │     │                       │     │  2x Garlic Naan   │
│               │     │                       │     │                   │
│               │     │                       │  ┌──│  Chef taps READY  │
│  🟢 "Butter   │◀────│  Pushes status update │◀─┘  │                   │
│  Chicken READY│     │  via WebSocket        │     │                   │
│  — Table 5"   │     │                       │     │                   │
│               │     │                       │     │                   │
│  Serves food  │     │     ┌────────────┐    │     │                   │
│  Taps "Served"│────▶│     │ CASHIER PC │    │     │                   │
│               │     │     │ Generates  │    │     │                   │
│               │     │     │ bill &     │    │     │                   │
│               │     │     │ prints     │    │     │                   │
│               │     │     └────────────┘    │     │                   │
└──────────────┘     └──────────────────────┘     └───────────────────┘

⚡ This flow works IDENTICALLY whether on Cloud or Local WiFi
```

### Device Discovery (Zero Config)

Waiter phones and kitchen tablets find the Cashier PC automatically:

1. **mDNS Auto-Discovery** (primary) — Cashier PC broadcasts `_restrosync._tcp.local`, devices find it automatically
2. **QR Code Scan** (fallback) — Cashier PC shows a QR code, waiter scans with phone camera
3. **Manual URL** (last resort) — Type `http://192.168.x.x:8080` in phone browser

First-time setup takes ~30 seconds. After that, devices auto-connect.

---

## 🛠️ Tech Stack

### Desktop Application (Cashier PC)

| Component | Technology | Why |
|-----------|-----------|-----|
| Shell/Runtime | **Tauri 2.x** | 10x smaller than Electron (~3MB vs ~150MB), native webview, Rust backend |
| Frontend | **React 19 + TypeScript** | Massive ecosystem, shared code with web/waiter/kitchen UIs |
| UI Framework | **Shadcn/UI + Tailwind CSS 4** | Beautiful, accessible, fully customizable |
| State | **Zustand + TanStack Query** | Lightweight, performant, great for offline-first |
| Local Database | **SQLite (rusqlite)** | Zero-config, battle-tested, handles millions of records |
| Backend Core | **Rust** | Memory safety, blazing performance, compiles to native binary |
| Embedded Server | **Axum** | Serves waiter/kitchen devices on local WiFi |
| Device Discovery | **mDNS (mdns-sd)** | Waiter phones auto-discover Cashier PC |
| Real-time | **WebSocket (tokio-tungstenite)** | Instant KOT push to kitchen over local WiFi |
| Build Tool | **Vite 6** | Fastest frontend build tool |

### Web Application / Waiter & Kitchen UI

| Component | Technology |
|-----------|-----------|
| Framework | **Next.js 15** (App Router) |
| Frontend | **React 19 + TypeScript** (code shared with desktop) |
| PWA | **Vite PWA** — installable on waiter phones & kitchen tablets |
| Smart API Client | Custom fetch wrapper — auto-routes to cloud or local server |

### Backend / Cloud Server

| Component | Technology |
|-----------|-----------|
| Language | **Rust** |
| Web Framework | **Axum** (same as embedded local server — shared codebase) |
| ORM | **SeaORM** (async, supports PostgreSQL + SQLite) |
| Auth | **JWT + Argon2** |
| Authorization | **Casbin-RS** (RBAC/ABAC) |
| API Docs | **utoipa** (auto-generated OpenAPI/Swagger) |
| PDF Generation | **printpdf** |

### Databases & Storage

| Type | Technology | Purpose |
|------|-----------|---------|
| Primary (Cloud) | **PostgreSQL 17** | Main cloud database |
| Local (Desktop) | **SQLite 3** | Offline storage, embedded in Tauri |
| Cache | **Redis 7 / Valkey** | Sessions, hot data, rate limiting |
| Analytics | **ClickHouse** | OLAP for reports & dashboards (Phase 2) |
| Search | **Meilisearch** | Menu search, order search |
| File Storage | **MinIO / S3** | Images, exports, backups |
| Message Queue | **NATS** | Lightweight pub/sub for inter-service communication |

### Infrastructure

| Purpose | Technology |
|---------|-----------|
| Containerization | **Docker** |
| Orchestration | **Docker Compose** (MVP) → **Kubernetes** (scale) |
| CI/CD | **GitHub Actions** |
| Cloud | **AWS** (production) + **Hetzner** (dev/staging — 80% cheaper) |
| CDN | **CloudFront / Cloudflare** |
| Monitoring | **Grafana + Prometheus** |
| Logging | **Loki** |
| Reverse Proxy | **Traefik** |
| Error Tracking | **Sentry** |

### Why Rust?

- **10-100x faster** than Node.js/Python for billing calculations and report generation
- **50MB RAM** for 10,000 req/s vs 500MB for Node.js
- **Memory safe** — no null pointers, no data races, fewer production crashes
- **Shared core** — same Rust business logic runs on desktop (Tauri) AND server (Axum)
- **Lower cloud bills** — less CPU/RAM = less money at scale

### Why Tauri over Electron?

| Factor | Electron | Tauri |
|--------|----------|-------|
| Bundle Size | ~150MB | **~3-5MB** |
| RAM Usage | 200-500MB | **30-80MB** |
| Startup Time | 3-5 sec | **< 1 sec** |
| CPU at Idle | 5-15% | **< 1%** |
| Backend | Node.js | **Rust** |

For a POS that runs all day on potentially old hardware, this matters enormously.

---

## 🏗️ Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────────┐
│                           CLIENT DEVICES                                 │
│                                                                          │
│  ┌─────────────────┐  ┌────────────┐  ┌──────────────┐  ┌───────────┐ │
│  │  Cashier PC      │  │ Waiter     │  │ Kitchen      │  │ Owner     │ │
│  │  (Tauri Desktop) │  │ (Phone)    │  │ (Tablet)     │  │ (Web App) │ │
│  │                   │  │ (PWA)      │  │ (PWA)        │  │           │ │
│  │  + Embedded       │  └─────┬──────┘  └──────┬───────┘  └─────┬─────┘ │
│  │    Local Server   │◄───────┘               │               │       │
│  │    (Axum)         │◄────────────────────────┘               │       │
│  │                   │         (via Local WiFi)                │       │
│  │  + SQLite DB      │                                         │       │
│  │  + Sync Engine    │                                         │       │
│  │  + Smart Conn Mgr │                                         │       │
│  └────────┬──────────┘                                         │       │
│           │                                                     │       │
│           │  ┌──────────────────────────────┐                   │       │
│           │  │  SMART CONNECTION MANAGER     │                   │       │
│           │  │  Auto-detects: ☁️ 📡 💾        │                   │       │
│           │  └──────────────┬───────────────┘                   │       │
│           │                 │                                    │       │
└───────────┼─────────────────┼────────────────────────────────────┼───────┘
            │                 │                                    │
            ▼                 ▼                                    ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                     CLOUD (when internet available)                       │
│                                                                          │
│  ┌───────────┐  ┌──────────────────────────────────────────────────┐   │
│  │ API       │  │ Services: Billing │ Inventory │ Menu │ Orders │  │   │
│  │ Gateway   │──│ Tables │ Reports │ Auth │ Sync │ Notifications  │   │
│  │ (Traefik) │  └──────────────────────────────────────────────────┘   │
│  └───────────┘                                                          │
│                                                                          │
│  ┌────────────┐  ┌──────────┐  ┌──────┐  ┌───────────┐  ┌──────────┐ │
│  │ PostgreSQL │  │  Redis   │  │ NATS │  │Meilisearch│  │ MinIO/S3 │ │
│  └────────────┘  └──────────┘  └──────┘  └───────────┘  └──────────┘ │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

### Unified API Contract

Both the cloud server and the embedded local server implement the **exact same API**. Waiter/kitchen devices don't know or care which one they're talking to.

```rust
// Shared trait — implemented by both PostgreSQL (cloud) and SQLite (local)
trait DataSource {
    async fn create_order(&self, order: NewOrder) -> Order;
    async fn get_active_orders(&self) -> Vec<Order>;
    async fn update_kot_status(&self, id: Uuid, status: KotStatus);
    // ... same interface, different storage backend
}

struct PostgresSource { pool: PgPool }     // Cloud
struct SqliteSource  { conn: SqlitePool }  // Local

// Route handlers use DataSource — don't care which backend
```

---

## 📁 Project Structure

```
restrosync/
├── .github/
│   └── workflows/
│       ├── ci.yml                # Runs on every PR
│       ├── release-desktop.yml   # Build & publish desktop app
│       ├── release-web.yml       # Build & deploy web app
│       └── release-server.yml    # Build & deploy cloud server
│
├── apps/
│   ├── desktop/                  # 🖥️ Tauri desktop app (Cashier PC)
│   │   ├── src-tauri/            # Rust backend
│   │   │   ├── src/
│   │   │   │   ├── main.rs
│   │   │   │   ├── commands/     # Tauri IPC commands
│   │   │   │   ├── db/           # SQLite operations
│   │   │   │   ├── server/       # Embedded local HTTP + WS server
│   │   │   │   ├── sync/         # Sync engine
│   │   │   │   ├── discovery/    # mDNS device discovery
│   │   │   │   └── connection/   # Smart Connection Manager
│   │   │   ├── Cargo.toml
│   │   │   └── tauri.conf.json
│   │   ├── src/                  # React frontend
│   │   │   ├── components/
│   │   │   ├── pages/
│   │   │   ├── hooks/
│   │   │   └── stores/
│   │   ├── package.json
│   │   └── vite.config.ts
│   │
│   ├── web/                      # 🌐 Next.js web app (Owner dashboard)
│   │   ├── src/
│   │   │   ├── app/              # App Router pages
│   │   │   ├── components/
│   │   │   └── lib/
│   │   ├── package.json
│   │   └── next.config.js
│   │
│   └── server/                   # ☁️ Axum cloud API server
│       ├── src/
│       │   ├── main.rs
│       │   ├── config/
│       │   ├── routes/
│       │   │   ├── mod.rs
│       │   │   ├── auth.rs
│       │   │   ├── billing.rs
│       │   │   ├── inventory.rs
│       │   │   ├── menu.rs
│       │   │   ├── orders.rs
│       │   │   ├── kot.rs
│       │   │   ├── reports.rs
│       │   │   ├── tables.rs
│       │   │   └── sync.rs
│       │   ├── services/         # Business logic
│       │   ├── models/           # Database models
│       │   ├── middleware/
│       │   └── utils/
│       ├── migrations/
│       └── Cargo.toml
│
├── packages/
│   ├── ui/                       # 🎨 Shared UI components (Shadcn)
│   │   ├── src/components/
│   │   │   ├── ui/               # Base (button, input, dialog, etc.)
│   │   │   ├── billing/          # Billing components
│   │   │   ├── menu/             # Menu components
│   │   │   ├── orders/           # Order components
│   │   │   ├── tables/           # Table components
│   │   │   └── reports/          # Report components
│   │   └── package.json
│   │
│   ├── shared/                   # 📦 Shared TS types, utils, constants
│   │   ├── src/
│   │   │   ├── types/
│   │   │   ├── constants/
│   │   │   ├── utils/
│   │   │   └── validation/       # Zod schemas
│   │   └── package.json
│   │
│   └── core/                     # ⚙️ Shared Rust core library
│       ├── src/
│       │   ├── lib.rs
│       │   ├── billing/          # Billing logic (desktop + server)
│       │   ├── inventory/
│       │   ├── menu/
│       │   ├── orders/
│       │   ├── gst/              # GST calculation engine
│       │   ├── datasource/       # DataSource trait (PG + SQLite)
│       │   └── models/           # Shared Rust types
│       └── Cargo.toml
│
├── tools/
│   ├── db-seed/                  # Database seeding scripts
│   └── scripts/                  # Utility scripts
│
├── Cargo.toml                    # Rust workspace root
├── turbo.json                    # Turborepo config
├── package.json                  # Root package.json
├── pnpm-workspace.yaml
├── docker-compose.yml            # Local dev services (PG, Redis, etc.)
├── docker-compose.prod.yml       # Production services
├── Dockerfile.server
├── Dockerfile.web
├── .env.example
├── PROJECT_BLUEPRINT.md          # 📘 Detailed project blueprint (3000+ lines)
└── README.md                     # 👈 You are here
```

---

## 🚀 Getting Started

### Prerequisites

| Tool | Version | Install |
|------|---------|---------|
| **Rust** | 1.80+ | [rustup.rs](https://rustup.rs/) |
| **Node.js** | 20 LTS+ | [nodejs.org](https://nodejs.org/) |
| **pnpm** | 9+ | `npm install -g pnpm` |
| **Docker** | 24+ | [docker.com](https://www.docker.com/) |
| **Tauri CLI** | 2.x | `cargo install tauri-cli` |

### Setup

```bash
# 1. Clone the repository
git clone https://github.com/your-org/restrosync.git
cd restrosync

# 2. Install JavaScript dependencies
pnpm install

# 3. Start infrastructure (PostgreSQL, Redis, NATS)
docker compose up -d

# 4. Run database migrations
cd apps/server
cargo run --bin migrate
cd ../..

# 5. Seed development data (optional)
cargo run --bin seed

# 6. Start development (all apps in parallel)
pnpm dev
```

### Running Individual Apps

```bash
# Desktop app (Tauri + React)
pnpm --filter desktop dev

# Web app (Next.js)
pnpm --filter web dev

# Cloud server (Axum)
cd apps/server && cargo run

# Run all Rust tests
cargo test --workspace

# Run all JS tests
pnpm test

# Lint everything
pnpm lint && cargo clippy --workspace
```

### Building for Production

```bash
# Build desktop app (Windows installer)
pnpm --filter desktop build
# Output: apps/desktop/src-tauri/target/release/bundle/

# Build web app
pnpm --filter web build

# Build server (release binary)
cd apps/server && cargo build --release
# Output: target/release/restrosync-server (~15MB static binary)

# Build Docker images
docker build -f Dockerfile.server -t restrosync-api .
docker build -f Dockerfile.web -t restrosync-web .
```

---

## 📦 Core Modules

| Module | Description | Status |
|--------|-------------|--------|
| 🧾 **Billing** | GST calculation, discounts, split bills, multi-payment, thermal print, KOT | 🔨 In Progress |
| 📦 **Inventory** | Raw materials, recipes, auto-deduction, stock alerts, vendors, purchase orders | 🔨 In Progress |
| 🍕 **Menu** | Categories, items, variants, modifiers, pricing, availability, images | 🔨 In Progress |
| 📋 **Orders** | Dine-in/takeaway/delivery, multi-device KOT flow, status tracking | 🔨 In Progress |
| 📊 **Reports** | Sales, GST, inventory, payments, staff performance, PDF/Excel export | 🔨 In Progress |
| 🪑 **Tables** | Floor plan editor, status management, merge/split/transfer, reservations | 🔨 In Progress |
| 🔐 **Auth & RBAC** | JWT auth, role-based permissions, multi-outlet support | 🔨 In Progress |
| 🌐 **Dynamic Networking** | Smart Connection Manager, embedded local server, sync engine | 🔨 In Progress |
| 📱 **Waiter PWA** | Phone-based order taking, table status, food-ready notifications | 🔨 In Progress |
| 👨‍🍳 **Kitchen Display** | Tablet KOT view, status updates, audio alerts | 🔨 In Progress |

---

## 💻 Hardware Requirements

### For the Restaurant

| Item | Required? | Estimated Cost | Notes |
|------|-----------|---------------|-------|
| **PC / Laptop** | ✅ Required | ₹15,000+ | Runs desktop app + local server |
| **Thermal Printer** | ✅ Required | ₹3,000 - 5,000 | Bill printing (58mm/80mm) |
| **WiFi Router** | 📡 For multi-device | ₹500 - 2,000 | Most restaurants already have one |
| **Waiter Phones** | 📡 For multi-device | ₹0 | Staff use their own phones |
| **Kitchen Tablet** | 📡 For KDS | ₹8,000 - 10,000 | Any Android tablet mounted in kitchen |
| **Internet** | ☁️ For cloud access | Varies | Not required for local operation |

**Total extra cost: ₹0 - ₹10,000** — most things already exist in a restaurant.

### Minimum PC Specifications

| Component | Minimum | Recommended |
|-----------|---------|-------------|
| OS | Windows 10 (64-bit) | Windows 11 |
| CPU | Dual-core 1.5GHz | Quad-core 2.0GHz+ |
| RAM | 2 GB | 4 GB+ |
| Storage | 500 MB free | 2 GB+ free |
| Display | 1280×720 | 1920×1080 |

---

## 🗺️ Roadmap

### Phase 0: Foundation (Weeks 1-2) `🔨 Current`
- [x] Project blueprint & architecture design
- [ ] Monorepo setup (Turborepo + Cargo workspace)
- [ ] Tauri + React desktop shell
- [ ] Axum server with basic middleware
- [ ] PostgreSQL + SQLite schema design
- [ ] CI/CD pipeline (GitHub Actions)
- [ ] Design system (Figma + Shadcn)

### Phase 1: MVP Core (Weeks 3-10)
- [ ] Authentication + RBAC
- [ ] Menu Management (categories, items, variants, modifiers)
- [ ] Table Management (floor plan, status, merge/split)
- [ ] Order Management (dine-in, takeaway, KOT)
- [ ] Billing System (GST, discounts, payments, print)
- [ ] Inventory (materials, recipes, auto-deduction, vendors)
- [ ] Core Reports (daily sales, item-wise, GST, payments)

### Phase 2: Dynamic Networking + Multi-Device (Weeks 11-16)
- [ ] Smart Connection Manager (auto-detect Cloud/Local/Standalone)
- [ ] Embedded local HTTP + WebSocket server in Tauri
- [ ] mDNS device discovery
- [ ] Waiter PWA (order taking on phone)
- [ ] Kitchen Display PWA (KOT view on tablet)
- [ ] Sync engine (change tracking, push/pull, conflict resolution)
- [ ] Thermal printer & cash drawer integration
- [ ] Testing, security audit, performance optimization
- [ ] **🚀 Beta launch with 5-10 restaurants**

### Phase 3: Growth Features (Weeks 17-24)
- [ ] Kitchen Display System (advanced)
- [ ] QR code digital menu (customer-facing)
- [ ] Swiggy / Zomato integration
- [ ] Loyalty & rewards program
- [ ] Advanced analytics dashboard
- [ ] Multi-language support (12+ Indian languages)
- [ ] WhatsApp bill sharing
- [ ] Customer feedback system

### Phase 4: Scale (Week 25+)
- [ ] Android / iOS mobile apps
- [ ] Plugin / extension system
- [ ] AI-powered demand forecasting
- [ ] Multi-brand & franchise management
- [ ] White-label solution
- [ ] Public API & marketplace

---

## 🤝 Contributing

This is currently a **private project** in early development. Contributing guidelines will be published once we reach public beta.

### For Team Members

1. Create a feature branch from `main`: `git checkout -b feat/billing-engine`
2. Follow the coding standards:
   - **Rust:** `cargo clippy` must pass, `cargo fmt` for formatting
   - **TypeScript:** `eslint` must pass, `prettier` for formatting
3. Write tests for new functionality
4. Create a Pull Request with a clear description
5. At least 1 code review approval required before merge

### Branch Naming

```
feat/    → New features         (feat/billing-engine)
fix/     → Bug fixes            (fix/gst-rounding-error)
refactor/→ Code refactoring     (refactor/order-service)
docs/    → Documentation        (docs/api-endpoints)
test/    → Adding tests         (test/inventory-deduction)
chore/   → Maintenance tasks    (chore/update-dependencies)
```

---

## 📖 Documentation

| Document | Description |
|----------|-------------|
| 📘 **[PROJECT_BLUEPRINT.md](./PROJECT_BLUEPRINT.md)** | Comprehensive 3,000+ line technical blueprint covering architecture, tech stack, all 6 modules with data models, dynamic networking architecture, multi-device flow, database design, security, API endpoints, development timeline, cost estimates, and more |
| 📗 **API Documentation** | Auto-generated Swagger/OpenAPI docs (available at `/api/docs` when server is running) |
| 📙 **User Guide** | Coming soon (Phase 2) |

---

## 📊 Performance Targets

| Metric | Target |
|--------|--------|
| Bill generation | < 50ms |
| Menu item search | < 30ms |
| Daily report generation | < 200ms |
| Desktop app startup | < 2 seconds |
| Desktop RAM usage (idle) | < 100MB |
| Desktop installer size | < 15MB |
| API response (P95) | < 200ms |
| WebSocket latency (local WiFi) | < 50ms |
| Mode switch time | < 2 seconds |
| Sync (100 records) | < 3 seconds |

---

## 💰 Pricing Model (Planned)

| Plan | Price | Target |
|------|-------|--------|
| **Free** | ₹0/month | 1 outlet, basic features, 100 bills/day |
| **Starter** | ₹499/month | 1 outlet, all core features |
| **Professional** | ₹999/month | Up to 3 outlets, reports, cloud sync |
| **Business** | ₹1,999/month | Up to 10 outlets, all features |
| **Enterprise** | Custom | Unlimited outlets, SLA, dedicated support |

---

## 📄 License

**Proprietary** — All rights reserved.

This software is the proprietary product of RestroSync. Unauthorized copying, distribution, or modification is strictly prohibited.

---

<p align="center">
  <strong>RestroSync</strong> — Restaurant management that never goes down. 🚀
  <br />
  <sub>Built with ❤️ using Rust, React, and Tauri</sub>
</p>