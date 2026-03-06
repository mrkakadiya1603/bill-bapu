# 🗺️ RestroSync — Development Roadmap

## *Complete Step-by-Step Implementation Plan*

> **Document Version:** 1.0.0
> **Created:** June 2025
> **Companion Document:** [PROJECT_BLUEPRINT.md](./PROJECT_BLUEPRINT.md) (3,200+ lines of architecture & design details)
> **Status:** Active — Phase 0

---

## Table of Contents

1. [Roadmap Overview](#1-roadmap-overview)
2. [Pre-Development Checklist](#2-pre-development-checklist)
3. [Phase 0 — Foundation Setup (Weeks 1-2)](#3-phase-0--foundation-setup-weeks-1-2)
4. [Phase 1 — MVP Core (Weeks 3-10)](#4-phase-1--mvp-core-weeks-3-10)
5. [Phase 2 — MVP Complete (Weeks 11-16)](#5-phase-2--mvp-complete-weeks-11-16)
6. [Phase 3 — Growth Features (Weeks 17-24)](#6-phase-3--growth-features-weeks-17-24)
7. [Phase 4 — Scale (Week 25+)](#7-phase-4--scale-week-25)
8. [Daily Development Workflow](#8-daily-development-workflow)
9. [Definition of Done (DoD)](#9-definition-of-done-dod)
10. [Risk Checkpoints](#10-risk-checkpoints)
11. [Key Milestones & Go/No-Go Gates](#11-key-milestones--gono-go-gates)
12. [Dependency Map](#12-dependency-map)

---

## 1. Roadmap Overview

```
TOTAL TIMELINE: 24 weeks (6 months) to Growth-ready product
MVP LAUNCH:     16 weeks (4 months) — Beta with 5-10 restaurants

┌──────────────────────────────────────────────────────────────────────────────┐
│                                                                              │
│  PHASE 0         PHASE 1              PHASE 2              PHASE 3          │
│  Foundation      MVP Core             MVP Complete         Growth           │
│                                                                              │
│  ├─ Week 1-2 ─┤├── Week 3-10 ──────┤├── Week 11-16 ────┤├── Week 17-24 ──┤│
│  │  2 weeks   ││   8 weeks          ││   6 weeks         ││   8 weeks      ││
│  │            ││                    ││                   ││                ││
│  │ • Monorepo ││ Sprint 1: Auth     ││ Sprint 5: Dynamic ││ • KDS Advanced ││
│  │ • Tauri    ││   + Menu           ││   Networking +    ││ • QR Menu      ││
│  │ • Axum    ││ Sprint 2: Tables   ││   Multi-Device    ││ • Integrations ││
│  │ • Schema   ││   + Orders         ││ Sprint 6: Polish  ││ • Analytics    ││
│  │ • CI/CD    ││ Sprint 3: Billing  ││   + Hardware      ││ • i18n         ││
│  │ • Design   ││ Sprint 4: Inventory││ Sprint 7: Test    ││ • WhatsApp     ││
│  │   System   ││   + Reports        ││   + Beta Launch   ││                ││
│  │            ││                    ││                   ││                ││
│  │  ✅ Dev env ││  ✅ Core features   ││  ✅ Beta launch    ││  ✅ Public       ││
│  │  working   ││  working on        ││  with 5-10        ││  launch        ││
│  │            ││  desktop + web     ││  restaurants      ││                ││
│  │            ││                    ││                   ││                ││
│  └────────────┘└────────────────────┘└───────────────────┘└────────────────┘│
│                                                                              │
│                                                        PHASE 4 ─────────▶  │
│                                                        Scale (Week 25+)    │
│                                                        • Mobile Apps       │
│                                                        • Plugin System     │
│                                                        • AI Features       │
│                                                        • Enterprise        │
│                                                                              │
└──────────────────────────────────────────────────────────────────────────────┘
```

### Team Allocation Reference

| Role | Person | Phase 0 | Phase 1 | Phase 2 | Phase 3 |
|------|--------|---------|---------|---------|---------|
| Tech Lead / Founder | TBD | Architecture, Setup | Backend + Review | Sync Engine + Review | Architecture |
| Backend Dev 1 | TBD | Schema, Axum setup | APIs for all modules | Embedded Server, Sync | Integrations |
| Backend Dev 2 (optional) | TBD | — | APIs, Print Engine | Smart Conn Mgr, mDNS | APIs |
| Frontend Dev 1 | TBD | Tauri + React setup | Desktop UI (all modules) | Waiter PWA, Kitchen PWA | Advanced UI |
| Frontend Dev 2 (optional) | TBD | Next.js setup | Web UI, shared components | Device discovery UI | QR Menu, i18n |
| Full-stack Dev | TBD | Docker, CI/CD | Feature dev, bug fixes | Integration, testing | Feature dev |
| UI/UX Designer | TBD | Design system, Figma | Screen designs per sprint | Waiter/Kitchen UX | Growth UX |
| Business / Founder | TBD | Restaurant partnerships | User feedback loop | Beta management | Sales & marketing |

---

## 2. Pre-Development Checklist

**Complete these BEFORE Week 1 begins. Target: 3-5 days.**

### 2.1 Business Setup

| # | Task | Owner | Status |
|---|------|-------|--------|
| B1 | Register company (LLP or Pvt Ltd) | Business Founder | ☐ |
| B2 | Open company bank account | Business Founder | ☐ |
| B3 | Register GST (if applicable) | Business Founder | ☐ |
| B4 | Register domain name (restrosync.com / restrosync.in) | Tech Lead | ☐ |
| B5 | Create company email accounts | Tech Lead | ☐ |
| B6 | Set up communication (Slack / Discord workspace) | Any | ☐ |
| B7 | Trademark application for "RestroSync" | Business Founder | ☐ |
| B8 | Draft basic NDA for team members | Business Founder | ☐ |

### 2.2 Restaurant Partnerships (Critical — Start Immediately)

| # | Task | Owner | Status |
|---|------|-------|--------|
| R1 | Identify 10-15 target restaurants for beta (mix of sizes) | Business Founder | ☐ |
| R2 | Visit 5+ restaurants, interview owners about pain points | Business Founder | ☐ |
| R3 | Get 3-5 verbal commitments for free beta testing | Business Founder | ☐ |
| R4 | Document their current workflow (how they take orders, bill, manage inventory) | Business Founder | ☐ |
| R5 | Photograph their current hardware (PCs, printers, tablets) | Business Founder | ☐ |
| R6 | List their current software (Petpooja, manual, other) and pain points | Business Founder | ☐ |
| R7 | Identify 2-3 "design partner" restaurants willing to give weekly feedback | Business Founder | ☐ |

> **Why this matters:** If you skip this, you'll build features no one needs. These restaurant visits will directly shape Sprint 1-4 priorities.

### 2.3 Team Finalization

| # | Task | Owner | Status |
|---|------|-------|--------|
| T1 | Finalize founding team (who's in, equity split) | All Founders | ☐ |
| T2 | Sign founder agreement (vesting: 4 years, 1-year cliff) | All Founders | ☐ |
| T3 | Hire / commit Backend Dev (Rust) — at least 1 | Tech Lead | ☐ |
| T4 | Hire / commit Frontend Dev (React) — at least 1 | Tech Lead | ☐ |
| T5 | Hire / commit UI/UX Designer (can be part-time/contract) | Tech Lead | ☐ |
| T6 | Set up developer machines (Rust, Node.js, Docker installed) | All Devs | ☐ |

### 2.4 Tools & Accounts Setup

| # | Tool | Purpose | Status |
|---|------|---------|--------|
| A1 | GitHub Organization (private) | Code hosting, CI/CD | ☐ |
| A2 | Figma (Starter plan) | Design system, wireframes | ☐ |
| A3 | Linear / Jira (free tier) | Project management, sprint tracking | ☐ |
| A4 | Sentry (free tier) | Error tracking | ☐ |
| A5 | Cloudflare | DNS, DDoS protection | ☐ |
| A6 | Hetzner / DigitalOcean account | Staging/production servers | ☐ |
| A7 | AWS account | S3, SES (email), future production | ☐ |
| A8 | Notion / Confluence | Internal documentation | ☐ |

### 2.5 Hardware for Testing

| # | Item | Cost (₹) | Status |
|---|------|-----------|--------|
| H1 | Low-end test PC (Windows 10, 2GB RAM, old CPU) | ₹10,000-15,000 | ☐ |
| H2 | Thermal printer (58mm — cheapest Indian model) | ₹2,500-3,500 | ☐ |
| H3 | Thermal printer (80mm — mid-range) | ₹3,500-5,000 | ☐ |
| H4 | Cheap Android tablet (for kitchen display testing) | ₹8,000-10,000 | ☐ |
| H5 | WiFi router (for local network testing) | ₹500-1,500 | ☐ |
| H6 | Cash drawer (USB/RJ11 — basic model) | ₹2,000-3,000 | ☐ |

---

## 3. Phase 0 — Foundation Setup (Weeks 1-2)

### 🎯 Goal
Set up the complete development environment, monorepo structure, base database schema, CI/CD pipeline, and design system so that every team member can build and run the desktop app, web app, and backend server locally from day one.

### 🏁 Phase 0 Exit Criteria
- [x] `pnpm dev` starts desktop app (Tauri + React) with a "Hello World" screen
- [x] `pnpm dev` starts web app (Next.js) with a "Hello World" screen
- [x] `cargo run` starts Axum server and responds to `GET /v1/health` with `200 OK`
- [x] PostgreSQL + Redis + NATS running via Docker Compose
- [x] SQLite database created and accessible from Tauri Rust backend
- [x] SeaORM migrations for core tables (tenants, outlets, users) run successfully
- [x] GitHub Actions CI pipeline passes (lint + build + test)
- [ ] Figma design system has base components (colors, typography, buttons, inputs, cards)
- [ ] Every team member has cloned the repo and run the project locally

---

### Week 1: Core Infrastructure

#### Day 1 (Monday) — Monorepo & Rust Workspace

| # | Task | Details | Owner | Hours |
|---|------|---------|-------|-------|
| 0.1.1 | Initialize monorepo with Turborepo | `npx create-turbo@latest`, configure `turbo.json` with `build`, `dev`, `lint`, `test` pipelines | Tech Lead | 2h |
| 0.1.2 | Configure pnpm workspace | Create `pnpm-workspace.yaml` with `apps/*` and `packages/*` globs | Tech Lead | 0.5h |
| 0.1.3 | Initialize Rust cargo workspace | Create root `Cargo.toml` with members: `apps/server`, `apps/desktop/src-tauri`, `packages/core` | Backend Dev | 1h |
| 0.1.4 | Create `packages/core` Rust library | Empty lib with module stubs: `billing/`, `inventory/`, `menu/`, `orders/`, `gst/`, `models/`, `datasource/` | Backend Dev | 1h |
| 0.1.5 | Create root `.gitignore` | Cover Rust `target/`, Node `node_modules/`, `.env`, SQLite files, IDE files | Tech Lead | 0.5h |
| 0.1.6 | Create `.env.example` | Database URLs, JWT secret, server port, Redis URL, NATS URL placeholders | Tech Lead | 0.5h |
| 0.1.7 | Create `docker-compose.yml` | Services: PostgreSQL 17, Redis 7, NATS, Meilisearch (all with health checks, volumes) | Full-stack | 2h |
| 0.1.8 | Test: all team members run `docker compose up -d` | Verify all services start, ports are accessible | All | 0.5h |

**End of Day 1 Deliverable:** Monorepo skeleton exists. `docker compose up -d` starts all infrastructure.

#### Day 2 (Tuesday) — Axum Server + Database Foundation

| # | Task | Details | Owner | Hours |
|---|------|---------|-------|-------|
| 0.2.1 | Create `apps/server` Axum project | Basic Axum app with `/v1/health` endpoint, CORS, structured logging (tracing), graceful shutdown | Backend Dev | 3h |
| 0.2.2 | Add SeaORM to server | Configure `DATABASE_URL`, connection pool (deadpool), set up migration binary | Backend Dev | 2h |
| 0.2.3 | Create initial database migrations | Tables: `tenants`, `plans`, `outlets`, `users`, `roles`, `permissions`, `user_roles` with proper indexes, constraints, UUID PKs | Backend Dev | 3h |
| 0.2.4 | Run migrations, verify schema | `cargo run --bin migrate`, inspect with `psql` or pgAdmin | Backend Dev | 0.5h |
| 0.2.5 | Create base error handling | Custom `AppError` type implementing `IntoResponse`, error codes, structured JSON error responses | Backend Dev | 1.5h |
| 0.2.6 | Add request validation | Integrate `validator` crate, create validation middleware | Backend Dev | 1h |

**End of Day 2 Deliverable:** `cargo run` starts Axum server. Health check works. Database schema created.

#### Day 3 (Wednesday) — Tauri Desktop App + SQLite

| # | Task | Details | Owner | Hours |
|---|------|---------|-------|-------|
| 0.3.1 | Create `apps/desktop` Tauri 2.x project | `pnpm create tauri-app`, configure with React + Vite | Frontend Dev | 2h |
| 0.3.2 | Configure Tauri for development | `tauri.conf.json`: app name, window size (1280x720 default), dev server URL, allowed APIs | Frontend Dev | 1h |
| 0.3.3 | Set up SQLite in Tauri Rust backend | Add `rusqlite` to `src-tauri/Cargo.toml`, create DB file in app data directory, test basic read/write | Backend Dev | 2h |
| 0.3.4 | Create SQLite migration system | Simple versioned SQL files, migration runner that tracks applied versions in `_migrations` table | Backend Dev | 2h |
| 0.3.5 | Create initial SQLite schema | Mirror core tables from PostgreSQL: `outlets`, `users`, `menu_categories`, `menu_items`, `orders`, `bills` + sync columns (`sync_status`, `cloud_id`, `last_synced_at`) | Backend Dev | 2h |
| 0.3.6 | Create first Tauri command | `#[tauri::command] fn get_app_info()` returning app version and DB status. Call from React frontend to verify IPC works. | Backend Dev | 1h |
| 0.3.7 | Test: Tauri desktop app launches | `pnpm --filter desktop dev` opens a window showing app info from Rust | All | 0.5h |

**End of Day 3 Deliverable:** Tauri desktop app launches. React frontend talks to Rust backend. SQLite is accessible.

#### Day 4 (Thursday) — Next.js Web App + Shared Packages

| # | Task | Details | Owner | Hours |
|---|------|---------|-------|-------|
| 0.4.1 | Create `apps/web` Next.js 15 project | App Router, TypeScript, Tailwind CSS 4, `src/` directory | Frontend Dev | 1.5h |
| 0.4.2 | Create `packages/ui` shared component library | Initialize with Shadcn CLI, install base components: Button, Input, Card, Dialog, Table, Badge, Tabs, Dropdown, Toast, Form | Frontend Dev | 3h |
| 0.4.3 | Create `packages/shared` TypeScript package | Folder structure: `types/`, `constants/`, `utils/`, `validation/`. Add: API response types, role enums, order status enums, payment mode enums | Frontend Dev | 2h |
| 0.4.4 | Configure path aliases | `@restrosync/ui`, `@restrosync/shared` importable from both `desktop` and `web` apps | Frontend Dev | 1h |
| 0.4.5 | Create `ts-rs` type generation setup | Configure `ts-rs` in `packages/core` to auto-generate TypeScript types from Rust structs. Run: `cargo test` generates `.ts` files into `packages/shared/src/types/generated/` | Backend Dev | 2h |
| 0.4.6 | Test cross-package imports | Import a Button from `@restrosync/ui` and a type from `@restrosync/shared` in both desktop and web apps | Frontend Dev | 1h |

**End of Day 4 Deliverable:** Three apps (desktop, web, server) all building. Shared packages working. Types generated from Rust.

#### Day 5 (Friday) — CI/CD, Linting, Developer Experience

| # | Task | Details | Owner | Hours |
|---|------|---------|-------|-------|
| 0.5.1 | Set up GitHub Actions CI | `.github/workflows/ci.yml`: On PR → run `cargo clippy`, `cargo test`, `cargo fmt --check`, `pnpm lint`, `pnpm build`, `pnpm test` | Full-stack | 3h |
| 0.5.2 | Configure ESLint + Prettier | Shared config in root, enforce via CI. Rules: strict TypeScript, import ordering, no unused vars | Frontend Dev | 1.5h |
| 0.5.3 | Configure Clippy (strict) | `clippy.toml` with strict lints, deny warnings in CI | Backend Dev | 0.5h |
| 0.5.4 | Set up pre-commit hooks | `husky` + `lint-staged`: run Prettier on staged `.ts/.tsx`, `cargo fmt` on staged `.rs` | Full-stack | 1h |
| 0.5.5 | Create `Makefile` / `justfile` | Common commands: `make dev`, `make test`, `make build`, `make db-migrate`, `make db-seed`, `make lint` | Tech Lead | 1h |
| 0.5.6 | Write `CONTRIBUTING.md` | Branch naming, commit conventions, PR process, code review expectations | Tech Lead | 1h |
| 0.5.7 | Write `apps/server/README.md` | How to run server, environment variables, API testing instructions | Backend Dev | 0.5h |
| 0.5.8 | Test: Full CI pipeline passes on a test PR | Create a branch, make a small change, open PR, verify all checks pass green | All | 1h |

**End of Day 5 Deliverable:** CI/CD is live. Every PR gets auto-checked. Developer workflow is smooth.

---

### Week 2: Design System + Database Schema Completion

#### Days 6-7 (Monday-Tuesday) — Design System & Figma

| # | Task | Details | Owner | Hours |
|---|------|---------|-------|-------|
| 0.6.1 | Define design tokens | Colors (brand, semantic: success/warning/error/info), typography scale (font sizes, weights, line heights), spacing scale (4px base), border radius, shadows | Designer | 4h |
| 0.6.2 | Implement tokens in Tailwind config | `tailwind.config.ts` with custom colors, extend theme. Create CSS variables for dark mode readiness. | Frontend Dev | 2h |
| 0.6.3 | Design core UI components in Figma | Button (primary, secondary, outline, ghost, sizes), Input (text, number, search), Select, Checkbox, Radio, Toggle, Badge, Card, Modal/Dialog, Toast/Notification | Designer | 6h |
| 0.6.4 | Design app shell layout in Figma | Sidebar navigation, top bar (with ☁️📡💾 status icon), main content area, responsive breakpoints | Designer | 4h |
| 0.6.5 | Design login screen in Figma | Email/password form, "Remember me", forgot password link, branding | Designer | 2h |
| 0.6.6 | Implement app shell layout in React | Sidebar with navigation items (Dashboard, Billing, Menu, Orders, Tables, Inventory, Reports, Settings), top bar, content outlet. Shared between desktop and web. | Frontend Dev | 4h |

#### Days 8-9 (Wednesday-Thursday) — Complete Database Schema

| # | Task | Details | Owner | Hours |
|---|------|---------|-------|-------|
| 0.8.1 | Menu module schema (PostgreSQL) | Migration: `menu_categories` (hierarchical with `parent_id`), `menu_items`, `menu_variants`, `modifier_groups`, `modifiers`, `menu_item_modifier_groups` | Backend Dev | 3h |
| 0.8.2 | Orders module schema (PostgreSQL) | Migration: `orders` (with `order_type` enum), `order_items`, `kots` (kitchen order tickets), `kot_items` | Backend Dev | 3h |
| 0.8.3 | Billing module schema (PostgreSQL) | Migration: `bills`, `bill_items`, `payments`, `discounts` | Backend Dev | 2h |
| 0.8.4 | Inventory module schema (PostgreSQL) | Migration: `raw_materials`, `recipes`, `recipe_ingredients`, `stock_movements`, `vendors`, `purchase_orders`, `purchase_order_items` | Backend Dev | 3h |
| 0.8.5 | Tables module schema (PostgreSQL) | Migration: `floors`, `restaurant_tables` (avoid SQL keyword), `reservations` | Backend Dev | 1.5h |
| 0.8.6 | Reports/analytics support columns | Add necessary indexes on `bills.created_at`, `orders.created_at`, `bill_items.menu_item_id` for report query performance | Backend Dev | 1h |
| 0.8.7 | Mirror all schemas to SQLite | Create equivalent SQLite migrations with sync columns. Use `TEXT` for UUIDs, `REAL` for decimals, add `sync_status`, `cloud_id`, `last_synced_at` on every table | Backend Dev | 3h |
| 0.8.8 | Create database seed script | `cargo run --bin seed` — creates test tenant, outlet, 5 menu categories, 20 menu items with variants, 10 tables across 2 floors, sample users (admin, cashier, waiter, kitchen) | Backend Dev | 3h |

#### Day 10 (Friday) — Integration Test & Phase 0 Review

| # | Task | Details | Owner | Hours |
|---|------|---------|-------|-------|
| 0.10.1 | Full integration test | Every team member: clone fresh, `docker compose up`, `cargo run --bin migrate`, `cargo run --bin seed`, `pnpm dev` — verify everything works | All | 2h |
| 0.10.2 | Fix any issues found | — | All | 2h |
| 0.10.3 | Phase 0 retrospective | What went well? What was harder than expected? Adjust Phase 1 sprint plans if needed. | All | 1h |
| 0.10.4 | Phase 0 demo | Show: desktop app shell, web app shell, server health check, database with seed data, CI passing | Tech Lead | 0.5h |
| 0.10.5 | Plan Sprint 1 in detail | Create tickets in Linear/Jira for every Sprint 1 task. Assign owners. | Tech Lead | 1.5h |

---

### Phase 0 Deliverables Checklist

| # | Deliverable | Status |
|---|-------------|--------|
| ✅ | Monorepo (Turborepo + Cargo workspace) initialized | ✅ |
| ✅ | Tauri 2.x desktop app launches with React UI + Rust backend | ✅ |
| ✅ | Next.js 15 web app runs with shared UI components | ✅ |
| ✅ | Axum server runs with health check endpoint | ✅ |
| ✅ | PostgreSQL schema: all core tables with migrations | ✅ |
| ✅ | SQLite schema: all core tables with sync columns | ✅ |
| ✅ | SeaORM configured with migration runner | ✅ |
| ✅ | Database seed script with test data | ✅ |
| ✅ | Docker Compose: PostgreSQL, Redis, NATS, Meilisearch | ✅ |
| ✅ | Shared UI library (`packages/ui`) with Shadcn components | ✅ |
| ✅ | Shared types (`packages/shared`) with Zod schemas | ✅ |
| ✅ | Shared Rust core (`packages/core`) with module stubs | ✅ |
| ✅ | `ts-rs` generating TypeScript from Rust types | ✅ |
| ✅ | GitHub Actions CI pipeline (lint + build + test) | ✅ |
| ✅ | Pre-commit hooks (format + lint) | ✅ |
| ✅ | Design system in Figma (tokens, base components) | ☐ |
| ✅ | App shell layout (sidebar + topbar) in React | ✅ |
| ✅ | Login screen designed in Figma | ☐ |
| ✅ | `CONTRIBUTING.md` with team workflow | ✅ |
| ✅ | Every team member has run the project locally | ☐ |

---

## 4. Phase 1 — MVP Core (Weeks 3-10)

### 🎯 Goal
Build all 6 core modules (Auth, Menu, Tables, Orders, Billing, Inventory + Reports) as a functional single-PC desktop application with a web dashboard. By the end of Phase 1, a cashier should be able to manage a menu, take orders, generate bills with GST, and track inventory — all on the desktop app.

### 🏁 Phase 1 Exit Criteria
- [ ] User can log in with role-based permissions (Owner, Cashier, Waiter, Kitchen, Viewer)
- [ ] Full menu management (categories, items, variants, modifiers, availability)
- [ ] Floor plan with tables, table status tracking, merge/split/transfer
- [ ] Complete order lifecycle: create → KOT → prepare → ready → serve → bill
- [ ] Billing with GST calculation, discounts, multiple payment modes, thermal print
- [ ] Inventory: raw materials, recipes, auto-deduction on billing, low stock alerts
- [ ] Reports: daily sales, item-wise, category-wise, GST, payment mode
- [ ] All features work on desktop app (Tauri)
- [ ] Basic web dashboard shows reports and allows menu management
- [ ] 50+ Rust unit tests passing, 30+ React component tests passing

---

### Sprint 1 (Weeks 3-4): Authentication + Menu Management

**Sprint Goal:** Users can log in, manage roles, and fully manage the restaurant menu.

#### Week 3: Authentication System

| # | Task | Details | Owner | Est. | Priority |
|---|------|---------|-------|------|----------|
| 1.1.1 | **User model & registration API** | `POST /v1/auth/register` — validate email, hash password (Argon2id), create user in DB, return user object (no password) | Backend | 4h | P0 |
| 1.1.2 | **Login API** | `POST /v1/auth/login` — verify email/password, generate JWT access token (15 min, RS256) + refresh token (7 days, opaque, stored in Redis), return both | Backend | 4h | P0 |
| 1.1.3 | **Token refresh API** | `POST /v1/auth/refresh` — validate refresh token in Redis, rotate it (old one invalidated), return new pair | Backend | 2h | P0 |
| 1.1.4 | **Logout API** | `POST /v1/auth/logout` — delete refresh token from Redis, invalidate session | Backend | 1h | P0 |
| 1.1.5 | **Auth middleware** | Axum middleware: extract JWT from `Authorization: Bearer`, verify signature, decode claims (user_id, tenant_id, role, permissions), inject into request extensions | Backend | 3h | P0 |
| 1.1.6 | **RBAC system with Casbin** | Define role model (Owner, Manager, Cashier, Waiter, Kitchen, Viewer), define permissions (`billing:create`, `menu:update`, etc.), create Casbin policy file, integrate enforcer into middleware | Backend | 6h | P0 |
| 1.1.7 | **User management API** | `GET/POST/PUT/DELETE /v1/users` — CRUD with role assignment, password reset, activate/deactivate. Only Owner/Manager can manage users. | Backend | 4h | P1 |
| 1.1.8 | **Login UI (Desktop)** | Login form (email + password), "Remember me" toggle, error handling (wrong credentials, rate limit), loading states. Store JWT in Tauri secure storage. | Frontend | 4h | P0 |
| 1.1.9 | **Login UI (Web)** | Same design, store JWT in httpOnly cookie or secure localStorage | Frontend | 2h | P0 |
| 1.1.10 | **Auth context & route protection** | React context providing `user`, `isAuthenticated`, `permissions`. `<ProtectedRoute>` component that redirects to login. `usePermission('billing:create')` hook. | Frontend | 3h | P0 |
| 1.1.11 | **User management UI** | Table of users, invite form, role assignment dropdown, activate/deactivate toggle. Only visible to Owner role. | Frontend | 3h | P1 |
| 1.1.12 | **Tauri: Local auth for offline** | Cache user credentials + JWT in local SQLite. If offline, validate against local cache. Sync auth state when cloud becomes available. | Backend | 3h | P1 |

**Sprint 1 — Week 3 Tests:**
- [ ] Login with valid credentials returns JWT + refresh token
- [ ] Login with wrong password returns 401
- [ ] Expired JWT returns 401, refresh flow works
- [ ] RBAC: Cashier cannot access `DELETE /v1/users/:id`
- [ ] RBAC: Owner can access all endpoints
- [ ] UI: Login form shows validation errors, redirects on success

#### Week 4: Menu Management (Complete CRUD)

| # | Task | Details | Owner | Est. | Priority |
|---|------|---------|-------|------|----------|
| 1.2.1 | **Menu categories API** | `GET/POST/PUT/DELETE /v1/menu/categories` — hierarchical (parent_id for sub-categories), sort_order, image_url, is_active. Include query param `?tree=true` for nested tree response. | Backend | 4h | P0 |
| 1.2.2 | **Menu items API** | `GET/POST/PUT/DELETE /v1/menu/items` — with search (name), filter (category, food_type, is_available), pagination (cursor-based). Include: base_price, tax_rate, food_type enum, spice_level, prep_time. | Backend | 5h | P0 |
| 1.2.3 | **Menu variants API** | `GET/POST/PUT/DELETE /v1/menu/items/:id/variants` — variant name ("Half", "Full", "Regular", "Large"), price, is_default, sort_order | Backend | 3h | P0 |
| 1.2.4 | **Modifier groups API** | `GET/POST/PUT/DELETE /v1/menu/modifier-groups` — name, selection_type (Single/Multiple), min_selection, max_selection, is_required | Backend | 3h | P0 |
| 1.2.5 | **Modifiers API** | `GET/POST/PUT/DELETE /v1/menu/modifier-groups/:id/modifiers` — name, price, is_default, is_available | Backend | 2h | P0 |
| 1.2.6 | **Link modifiers to items** | `POST/DELETE /v1/menu/items/:id/modifier-groups/:groupId` — associate modifier groups with menu items | Backend | 2h | P0 |
| 1.2.7 | **Availability toggle API** | `PATCH /v1/menu/items/:id/availability` — quick toggle without full PUT, broadcasts WebSocket event | Backend | 1h | P0 |
| 1.2.8 | **Image upload API** | `POST /v1/upload/image` — accept multipart form, resize to 400x400 and 100x100 (thumbnail), store in MinIO/S3, return URL | Backend | 3h | P1 |
| 1.2.9 | **Category management UI** | Left sidebar: tree view of categories with drag-to-reorder. Click to select → right panel shows items. Add/edit/delete category modal. Sub-category indent. | Frontend | 5h | P0 |
| 1.2.10 | **Menu item list UI** | Grid/list toggle view. Show: image thumbnail, name, price, food_type badge (🟢 Veg, 🔴 Non-veg, 🟡 Egg), availability toggle, edit button. Search bar. Category filter. | Frontend | 5h | P0 |
| 1.2.11 | **Menu item editor UI** | Full-page form or slide-over panel: name, description, category (dropdown), base_price, tax_rate (dropdown: 5%/12%/18%), food_type (radio), spice_level, prep_time, image upload, tags. Save/Cancel. | Frontend | 5h | P0 |
| 1.2.12 | **Variant editor UI** | Within item editor: table of variants (name, price, default toggle). Add row, delete row, reorder. | Frontend | 3h | P0 |
| 1.2.13 | **Modifier group editor UI** | Within item editor: section showing linked modifier groups. Add/remove groups. For each group: configure min/max selection. Inline create new group + modifiers. | Frontend | 4h | P0 |
| 1.2.14 | **Menu on web dashboard** | Replicate menu management on the web app (Next.js). Reuse `@restrosync/ui` components. | Frontend | 3h | P1 |

**Sprint 1 — Week 4 Tests:**
- [ ] Create category → appears in tree
- [ ] Create item with 3 variants → all variants saved correctly
- [ ] Link modifier group to item → shows in item detail
- [ ] Toggle availability → item shows as unavailable in list
- [ ] Search items by name → filters correctly
- [ ] GST rate saved correctly (5.0 / 12.0 / 18.0)
- [ ] Image upload → returns valid URL → displays in UI

**Sprint 1 Review (End of Week 4):**
- Demo: Log in → Create categories → Add items with variants and modifiers → Toggle availability
- Retrospective: What went well? What's blocking? Adjust Sprint 2 if needed.

---

### Sprint 2 (Weeks 5-6): Table Management + Order System

**Sprint Goal:** Visual floor plan editor with table status. Complete order lifecycle from order placement to KOT generation.

#### Week 5: Table Management

| # | Task | Details | Owner | Est. | Priority |
|---|------|---------|-------|------|----------|
| 2.1.1 | **Floors API** | `GET/POST/PUT/DELETE /v1/floors` — name (e.g., "Ground Floor", "Terrace", "AC Section"), sort_order | Backend | 2h | P0 |
| 2.1.2 | **Tables API** | `GET/POST/PUT/DELETE /v1/tables` — floor_id, name ("T1", "T2"), capacity, shape (Round/Square/Rectangle), position_x, position_y, width, height, status enum (Available/Occupied/Reserved/Billing/Cleaning) | Backend | 3h | P0 |
| 2.1.3 | **Table status API** | `PATCH /v1/tables/:id/status` — update status with validation (can't go from Available to Billing directly). Broadcast via WebSocket. | Backend | 2h | P0 |
| 2.1.4 | **Table merge API** | `POST /v1/tables/merge` — body: `{ table_ids: [uuid1, uuid2] }`. Creates a logical merge. Orders from both tables linked. | Backend | 3h | P1 |
| 2.1.5 | **Table transfer API** | `POST /v1/tables/transfer` — body: `{ from_table_id, to_table_id }`. Moves active order to another table. Updates both table statuses. | Backend | 2h | P1 |
| 2.1.6 | **Floor plan editor UI** | Canvas area (using a library like `react-dnd` or custom SVG). Drag tables to position. Resize. Set shape. Grid snap (optional). Save layout via API. | Frontend | 8h | P0 |
| 2.1.7 | **Table status view UI** | Floor plan view (read-only) showing all tables with color-coded status: 🟢 Available, 🔴 Occupied, 🟡 Reserved, 🔵 Billing, ⚫ Cleaning. Show time occupied. Tap to see order details. | Frontend | 5h | P0 |
| 2.1.8 | **Table action menu UI** | Right-click / long-press on table: "Seat Guest", "View Order", "Transfer", "Merge", "Mark Cleaning", "Mark Available". Actions update status via API. | Frontend | 3h | P0 |
| 2.1.9 | **Waiter assignment API** | `PATCH /v1/tables/:id/assign` — assign a waiter (user_id) to a table or section | Backend | 1h | P1 |

#### Week 6: Order System

| # | Task | Details | Owner | Est. | Priority |
|---|------|---------|-------|------|----------|
| 2.2.1 | **Order CRUD API** | `GET/POST/PUT /v1/orders` — order_type enum (DineIn/Takeaway/Delivery), table_id (for DineIn), customer info (optional), status enum (Created/Confirmed/Preparing/Ready/Served/Billed/Completed/Cancelled) | Backend | 5h | P0 |
| 2.2.2 | **Order items API** | `POST /v1/orders/:id/items` — add items to order: menu_item_id, variant_id, quantity, modifiers[], notes. `DELETE /v1/orders/:id/items/:itemId` to remove. `PUT` to update quantity. | Backend | 4h | P0 |
| 2.2.3 | **KOT generation API** | `POST /v1/orders/:id/kot` — generates a KOT from un-sent items. Groups items by kitchen station (if configured). Returns KOT with items + table info. Marks items as `kot_status: Sent`. | Backend | 4h | P0 |
| 2.2.4 | **KOT status API** | `PATCH /v1/kot/:id/status` — update KOT status: Pending → Preparing → Ready. Individual items can be updated too. Broadcast status via WebSocket. | Backend | 3h | P0 |
| 2.2.5 | **Order status API** | `PATCH /v1/orders/:id/status` — update order status with state machine validation (can't skip states). Auto-calculate: if all KOTs ready → order status = Ready. | Backend | 3h | P0 |
| 2.2.6 | **Active orders API** | `GET /v1/orders/active` — returns all orders with status != Completed/Cancelled, including their items and KOT statuses. Optimized for dashboard. | Backend | 2h | P0 |
| 2.2.7 | **WebSocket setup** | Create WebSocket endpoint (`/ws`). Authentication via token query param. Channel subscription by outlet_id. Broadcast order/KOT/table events. | Backend | 4h | P0 |
| 2.2.8 | **Order placement UI** | Full-screen order flow: Left panel = menu items (searchable, category tabs). Right panel = current order (items list, quantities, notes). Bottom = "Send to Kitchen" button. Link to table (for dine-in) or mark as Takeaway/Delivery. | Frontend | 8h | P0 |
| 2.2.9 | **Running orders dashboard UI** | Grid of cards — one per active order. Shows: table number, order type badge, items summary, time elapsed, status badge (color-coded). Auto-updates via WebSocket. Click to expand details. | Frontend | 5h | P0 |
| 2.2.10 | **KOT view UI** | Kitchen-oriented view: list of pending KOTs, each showing table, items, time. "Start Preparing" and "Ready" buttons. Audio bell on new KOT. Color turns yellow after 10 min, red after 20 min. | Frontend | 5h | P0 |

**Sprint 2 Tests:**
- [ ] Create floor → Add 10 tables → Drag to position → Save → Reload → Positions preserved
- [ ] Seat guest at Table 5 → Table turns Occupied
- [ ] Place order (3 items) → Send KOT → KOT appears in kitchen view
- [ ] Kitchen marks "Ready" → Order status updates → Dashboard reflects
- [ ] Transfer order from Table 5 to Table 8 → Both tables update correctly
- [ ] WebSocket: open two browser tabs, action on one reflects on other instantly
- [ ] Active orders API returns correct count and details

**Sprint 2 Review (End of Week 6):**
- Demo: Floor plan → Seat guest → Place order → KOT in kitchen → Mark ready → Full flow
- Check: Is the order flow intuitive enough for a real waiter? Get restaurant partner feedback.

---

### Sprint 3 (Weeks 7-8): Billing System

**Sprint Goal:** Complete billing with GST, discounts, multiple payment modes, and thermal printing. This is the most critical module — it must be accurate to the paisa.

#### Week 7: Billing Engine (Backend)

| # | Task | Details | Owner | Est. | Priority |
|---|------|---------|-------|------|----------|
| 3.1.1 | **GST calculation engine** | `packages/core/src/gst/` — Pure Rust library. Functions: `calculate_gst(base_amount, tax_rate, is_inclusive) -> GstBreakdown { cgst, sgst, igst, cess, total_tax }`. Handle inter-state (IGST) vs intra-state (CGST+SGST). Configurable per outlet's state. | Backend | 5h | P0 |
| 3.1.2 | **Discount engine** | `packages/core/src/billing/discount.rs` — Types: Flat (₹50 off), Percentage (10% off), ItemLevel (₹20 off on specific item), BillLevel (applied to subtotal). Apply before or after tax (configurable). Validate: discount can't exceed item/bill total. | Backend | 4h | P0 |
| 3.1.3 | **Bill generation API** | `POST /v1/bills` — from an order_id: pull all order items, apply discounts, calculate GST per item (different items can have different rates: 5%/12%/18%), calculate service charge (%), round-off (configurable: nearest ₹1/₹0.5), generate unique bill number (RS-{YEAR}-{6-digit-seq}), create bill record. | Backend | 6h | P0 |
| 3.1.4 | **Bill finalization API** | `POST /v1/bills/:id/finalize` — lock bill (no further edits), update order status to "Billed", update table status to "Billing". Bill becomes immutable after this. Trigger inventory deduction (async). | Backend | 3h | P0 |
| 3.1.5 | **Payment recording API** | `POST /v1/bills/:id/payments` — body: `{ mode: "Cash"/"Card"/"UPI"/"Wallet"/"Credit", amount, reference }`. Support split payment (multiple modes on one bill). When total paid >= bill total → bill status = "Paid", table → "Cleaning". | Backend | 4h | P0 |
| 3.1.6 | **Bill void API** | `POST /v1/bills/:id/void` — only Owner/Manager role. Requires reason. Creates audit log. Reverses inventory deduction. | Backend | 3h | P0 |
| 3.1.7 | **Bill print data API** | `GET /v1/bills/:id/print` — returns structured JSON optimized for print rendering: header (restaurant name, GSTIN, FSSAI, address), items table, tax breakdown, totals, payment info, footer. Accepts query `?format=thermal` or `?format=a4`. | Backend | 3h | P0 |
| 3.1.8 | **Bill history/search API** | `GET /v1/bills` — paginated, filter by: date range, payment_status, order_type, created_by, min/max amount. Sort by date (default desc). | Backend | 2h | P0 |

**⚠️ Critical: GST Accuracy Tests (Backend):**
```
Test 1: Item ₹100 @ 5% GST → CGST ₹2.50, SGST ₹2.50, Total ₹105.00
Test 2: Item ₹250 @ 12% GST → CGST ₹15.00, SGST ₹15.00, Total ₹280.00
Test 3: Item ₹500 @ 18% GST → CGST ₹45.00, SGST ₹45.00, Total ₹590.00
Test 4: Inter-state → IGST ₹90 (no CGST/SGST split)
Test 5: Mixed cart (5% + 12% + 18% items) → Each taxed at own rate
Test 6: 10% discount on ₹500 item @ 18% → Discount ₹50, Taxable ₹450, GST ₹81, Total ₹531
Test 7: Round-off ₹531.47 → ₹531 (round to nearest ₹1)
Test 8: Split payment: ₹300 Cash + ₹231 UPI = ₹531 → Bill Paid
Test 9: Service charge 5% on subtotal ₹450 = ₹22.50 (charged before GST or after, per config)
Test 10: Bill with 0% GST items (packaged goods exemption)
```

#### Week 8: Billing UI + Print Engine

| # | Task | Details | Owner | Est. | Priority |
|---|------|---------|-------|------|----------|
| 3.2.1 | **Bill review UI** | From running order → "Generate Bill" button. Shows: item table (name, qty, rate, tax%, amount), subtotal, discount row (editable), service charge, tax breakdown (CGST + SGST per rate), round-off, grand total. | Frontend | 6h | P0 |
| 3.2.2 | **Discount application UI** | In bill review: "Add Discount" button. Options: Flat amount, Percentage, Per-item. Show discount amount and new total in real-time. | Frontend | 3h | P0 |
| 3.2.3 | **Payment UI** | Multi-mode payment screen. Show pending amount. Payment mode buttons (Cash, Card, UPI, Wallet). For Cash: tender amount input, auto-calculate change. For UPI: reference number input. Support split: add multiple payments until total is covered. "Finalize & Print" button. | Frontend | 5h | P0 |
| 3.2.4 | **Thermal print engine** | Rust module in `src-tauri/src/print/` — ESC/POS protocol. Functions: discover printers (USB + serial), format bill into ESC/POS commands (bold headers, column alignment, line cuts, barcode/QR), send to printer. Support 58mm and 80mm widths. | Backend | 8h | P0 |
| 3.2.5 | **Print preview UI** | Show print preview before printing (receipt-style visual). "Print" button calls Tauri command → Rust print engine → physical printer. | Frontend | 3h | P0 |
| 3.2.6 | **A4 invoice generation** | Rust PDF generation (`printpdf` crate) for full-page GST invoice: restaurant logo, GSTIN, FSSAI, itemized table, HSN codes, tax summary, payment details, terms. | Backend | 5h | P1 |
| 3.2.7 | **Bill history UI** | Searchable list/table: bill number, date/time, table/order type, total, payment status, staff. Click to view full bill. "Reprint" button. Date range filter. | Frontend | 4h | P0 |
| 3.2.8 | **Quick billing mode** | For takeaway/counter: skip table selection. Fast item search + add. Quick generate + pay + print in one flow. Optimized for speed (< 30 seconds total). | Frontend | 4h | P0 |

**Sprint 3 Tests:**
- [ ] Generate bill from order → GST calculated correctly per item
- [ ] Apply 10% discount → subtotal and tax recalculated
- [ ] Split payment (₹500 Cash + ₹200 UPI) → bill marked Paid
- [ ] Print thermal receipt → receipt is formatted correctly (tested on actual printer)
- [ ] Void bill → audit log created, inventory reversed
- [ ] Bill history → search by date → find yesterday's bills

**Sprint 3 Review (End of Week 8):**
- Demo: Full flow — Order → Bill → Discount → Payment → Print. Show on ACTUAL thermal printer.
- **Critical:** Get restaurant partner to review bill format. Does it match what they expect?

---

### Sprint 4 (Weeks 9-10): Inventory + Core Reports

**Sprint Goal:** Recipe-level inventory tracking with auto-deduction. Core reports for daily operations.

#### Week 9: Inventory Management

| # | Task | Details | Owner | Est. | Priority |
|---|------|---------|-------|------|----------|
| 4.1.1 | **Raw materials API** | `GET/POST/PUT/DELETE /v1/inventory/materials` — name, category (Vegetables, Spices, Dairy, Meat, etc.), unit (Kg/Gram/Litre/ML/Piece), current_stock, min_stock_threshold, cost_per_unit, is_active | Backend | 3h | P0 |
| 4.1.2 | **Recipe API** | `GET/POST/PUT/DELETE /v1/inventory/recipes` — links menu_item_id + variant_id to a list of ingredients: `[{ raw_material_id, quantity, unit }]`. Example: "Butter Chicken Full" = 200g Chicken + 50g Butter + 30ml Cream + 10g Spice Mix | Backend | 4h | P0 |
| 4.1.3 | **Stock movement API** | `POST /v1/inventory/stock/adjust` — type: PurchaseIn, Consumption, WastageOut, Adjustment, Transfer. Each creates a `stock_movements` record AND updates `raw_materials.current_stock`. | Backend | 4h | P0 |
| 4.1.4 | **Auto stock deduction on billing** | When bill is finalized → for each bill item → lookup recipe → calculate ingredient quantities (recipe_qty × item_qty) → create Consumption stock movements → update current_stock → check thresholds → trigger alerts if low. **This runs async (background task).** | Backend | 5h | P0 |
| 4.1.5 | **Low stock alerts** | When `current_stock < min_stock_threshold` → create alert record → broadcast WebSocket event → show notification in UI. `GET /v1/inventory/alerts` returns all active alerts. | Backend | 2h | P0 |
| 4.1.6 | **Vendor API** | `GET/POST/PUT/DELETE /v1/inventory/vendors` — name, phone, email, GSTIN, address, materials_supplied (list), payment_terms | Backend | 2h | P0 |
| 4.1.7 | **Purchase order API** | `GET/POST/PUT /v1/inventory/purchases` — vendor_id, items: [{ raw_material_id, quantity, unit_cost }], status (Draft/Sent/PartialReceived/Received/Cancelled), expected_delivery. On receive → auto stock-in. | Backend | 4h | P1 |
| 4.1.8 | **Raw materials UI** | Table: name, category, current stock (with unit), threshold, cost/unit, status. Color-code: red if below threshold. Add/edit modal. | Frontend | 4h | P0 |
| 4.1.9 | **Recipe editor UI** | Per menu item: "Manage Recipe" button. Shows ingredient list with quantity + unit. Add ingredient (search raw material), set quantity. Auto-calculate cost per serving. | Frontend | 4h | P0 |
| 4.1.10 | **Stock adjustment UI** | Quick form: select material → type (Purchase/Wastage/Adjustment) → quantity → notes → submit. Shows movement history per material. | Frontend | 3h | P0 |
| 4.1.11 | **Purchase order UI** | Create PO: select vendor → add items (material, qty, rate) → total auto-calculated → save as Draft → mark Sent → Receive (with partial receive support). | Frontend | 4h | P1 |

#### Week 10: Core Reports

| # | Task | Details | Owner | Est. | Priority |
|---|------|---------|-------|------|----------|
| 4.2.1 | **Daily sales report API** | `GET /v1/reports/sales/daily?date=2025-06-15` — total_sales, bill_count, avg_bill_value, total_discount, total_tax, net_sales. Breakdown by hour. Comparison with previous day. | Backend | 3h | P0 |
| 4.2.2 | **Item-wise sales report API** | `GET /v1/reports/sales/itemwise?from=&to=` — for each item: qty_sold, revenue, percentage_of_total. Sort by revenue desc. Top 10 + bottom 10 highlight. | Backend | 3h | P0 |
| 4.2.3 | **Category-wise sales report API** | Same as above but grouped by menu category | Backend | 1.5h | P0 |
| 4.2.4 | **Payment mode report API** | `GET /v1/reports/payments?from=&to=` — breakdown: Cash total, Card total, UPI total, Wallet total. Count + amount per mode. | Backend | 2h | P0 |
| 4.2.5 | **GST report API** | `GET /v1/reports/gst?from=&to=` — per tax rate: taxable_amount, CGST, SGST, total_tax. Summary row. GSTR-1 compatible format. | Backend | 3h | P0 |
| 4.2.6 | **Inventory report API** | `GET /v1/reports/inventory` — current stock levels, value (qty × cost), items below threshold, recent movements summary | Backend | 2h | P0 |
| 4.2.7 | **Staff performance report API** | `GET /v1/reports/staff?from=&to=` — per staff: bills_processed, total_amount, avg_bill_value, discounts_given | Backend | 2h | P1 |
| 4.2.8 | **Report export engine** | `POST /v1/reports/export` — body: `{ report_type, format: "pdf"/"excel", params }`. Generate PDF (printpdf) or Excel (calamine/rust_xlsxwriter). Return download URL. | Backend | 5h | P1 |
| 4.2.9 | **Dashboard UI** | Main dashboard screen: today's sales (big number), bills count, top 5 items (mini chart), hourly sales bar chart, recent orders list, low stock alerts. Auto-refresh every 60 seconds. | Frontend | 6h | P0 |
| 4.2.10 | **Reports page UI** | Report selector (tabs or dropdown). Date range picker. Report table with sorting. "Export PDF" / "Export Excel" buttons. Charts for visual reports (use `recharts` library). | Frontend | 6h | P0 |
| 4.2.11 | **Inventory dashboard UI** | Stock overview: materials at risk (below threshold, highlighted red), recent stock movements, stock value summary. Link to full inventory management. | Frontend | 3h | P0 |

**Sprint 4 Tests:**
- [ ] Create recipe for "Butter Chicken Full" with 4 ingredients
- [ ] Bill 2x Butter Chicken Full → stock deducted: 400g chicken, 100g butter, 60ml cream, 20g spice
- [ ] Stock goes below threshold → alert appears in UI
- [ ] Daily sales report matches sum of all bills for the day
- [ ] GST report: CGST + SGST totals match bill-level tax amounts
- [ ] Export daily report as PDF → opens correctly, data matches
- [ ] Dashboard auto-refreshes and shows live data

**Sprint 4 Review (End of Week 10) — MAJOR MILESTONE:**
- 🎉 Demo: **COMPLETE RESTAURANT FLOW** — Menu setup → Seat guest → Order → KOT → Ready → Bill → Payment → Print → Inventory deducted → Reports show data
- Get 2-3 restaurant partners to watch the demo. Collect feedback.
- Review: Is the UX fast enough for a busy restaurant? Target: order → bill < 2 minutes.

---

### Phase 1 Deliverables Checklist

| # | Deliverable | Status |
|---|-------------|--------|
| ✅ | Authentication (login, JWT, RBAC with 6 roles) | ☐ |
| ✅ | Menu management (categories, items, variants, modifiers, images) | ☐ |
| ✅ | Table management (floor plan editor, status tracking, merge/split/transfer) | ☐ |
| ✅ | Order system (dine-in/takeaway/delivery, KOT, status tracking, WebSocket) | ☐ |
| ✅ | Billing (GST engine, discounts, multi-payment, bill history) | ☐ |
| ✅ | Thermal printing (ESC/POS, 58mm + 80mm, bill + KOT) | ☐ |
| ✅ | Inventory (materials, recipes, auto-deduction, stock alerts, vendors) | ☐ |
| ✅ | Reports (daily sales, item-wise, GST, payments, staff, export) | ☐ |
| ✅ | Dashboard (live sales, top items, alerts) | ☐ |
| ✅ | Desktop app: all features functional | ☐ |
| ✅ | Web app: reports + menu management functional | ☐ |
| ✅ | 50+ backend tests passing | ☐ |
| ✅ | 30+ frontend component tests passing | ☐ |
| ✅ | Restaurant partner feedback collected (at least 3 sessions) | ☐ |

---

## 5. Phase 2 — MVP Complete (Weeks 11-16)

### 🎯 Goal
Add the fully dynamic networking architecture (Cloud ↔ Local WiFi ↔ Standalone auto-switching), multi-device support (waiter phones, kitchen tablets), sync engine, and hardware polish. By the end of Phase 2, the app is **beta-ready for real restaurants**.

### 🏁 Phase 2 Exit Criteria
- [ ] Smart Connection Manager auto-switches between Cloud, Local, and Standalone modes
- [ ] Embedded Axum server in Tauri serves waiter phones and kitchen tablets on local WiFi
- [ ] Waiter PWA works on phone browser: browse menu, take orders, see table status
- [ ] Kitchen Display PWA works on tablet: see KOTs, update status, audio alert on new KOT
- [ ] mDNS auto-discovery: waiter phone finds Cashier PC without manual IP entry
- [ ] Sync engine: offline changes sync to cloud when connection returns
- [ ] Thermal printer, cash drawer, barcode reader working on real hardware
- [ ] Windows installer (.msi) with auto-update
- [ ] Beta launched with 5-10 real restaurants

---

### Sprint 5 (Weeks 11-12): Dynamic Networking + Multi-Device

**Sprint Goal:** This is the make-or-break sprint. Build the embedded local server, Smart Connection Manager, waiter PWA, and kitchen display.

#### Week 11: Embedded Local Server + Smart Connection Manager

| # | Task | Details | Owner | Est. | Priority |
|---|------|---------|-------|------|----------|
| 5.1.1 | **Embedded Axum server in Tauri** | Create `src-tauri/src/server/` module. On app startup, spawn a lightweight Axum HTTP server on `0.0.0.0:8080` + WebSocket server on `0.0.0.0:8081`. Serve the **same route handlers** as cloud server but backed by local SQLite via `DataSource` trait. | Backend | 8h | P0 |
| 5.1.2 | **DataSource trait implementation** | `packages/core/src/datasource/` — define `trait DataSource { async fn create_order(...), async fn get_active_orders(...), ... }`. Implement `PostgresSource` and `SqliteSource`. Refactor all route handlers to use `DataSource` instead of direct DB calls. | Backend | 8h | P0 |
| 5.1.3 | **Smart Connection Manager** | `src-tauri/src/connection/` — background thread that runs every 5 seconds: (1) ping cloud server (2 sec timeout), (2) check local network interfaces. Set `current_mode: Cloud / Local / Standalone`. Expose via Tauri command + broadcast mode changes via WebSocket. | Backend | 5h | P0 |
| 5.1.4 | **mDNS service broadcast** | On embedded server startup, broadcast `_restrosync._tcp.local` via mDNS (mdns-sd crate). Include: outlet name, server version, IP:port in TXT records. | Backend | 3h | P0 |
| 5.1.5 | **Serve static Waiter/Kitchen PWA files** | Embedded server serves pre-built PWA files at `/waiter/` and `/kitchen/` paths. These are bundled into the Tauri app during build. | Backend | 2h | P0 |
| 5.1.6 | **Mode status UI indicator** | Top-right corner of desktop app: icon showing ☁️ (cloud), 📡 (local), 💾 (standalone), 🔄 (syncing). Tooltip shows details. Click to see connection info (IP, connected devices, sync status). | Frontend | 3h | P0 |
| 5.1.7 | **QR code for device setup** | Desktop app settings page: "Connect Devices" section shows QR code containing `http://192.168.x.x:8080/waiter/`. Waiter scans with phone camera → opens PWA. | Frontend | 2h | P0 |
| 5.1.8 | **Local auth for waiter/kitchen** | Embedded server authenticates waiter/kitchen devices with a simple PIN or the same JWT system. Tokens issued locally are valid only for the local server session. | Backend | 3h | P0 |

#### Week 12: Waiter PWA + Kitchen Display PWA

| # | Task | Details | Owner | Est. | Priority |
|---|------|---------|-------|------|----------|
| 5.2.1 | **Smart API Client library** | `packages/shared/src/api-client.ts` — wrapper around fetch. On init: check mDNS for local server → try cloud URL → fallback to local IP. Cache active server URL. On request failure: retry with alternate server. All transparent to calling code. | Frontend | 5h | P0 |
| 5.2.2 | **Waiter PWA — app shell** | Vite PWA project (or Next.js static export). Minimal shell: bottom nav (Tables, Orders, Menu, Profile). Service worker for caching. Installable on Android home screen. | Frontend | 4h | P0 |
| 5.2.3 | **Waiter PWA — floor plan view** | Read-only floor plan showing all tables with color-coded status. Tap table to see order details or "Seat Guest". | Frontend | 4h | P0 |
| 5.2.4 | **Waiter PWA — order taking** | Tap occupied table → "Add Items". Menu browser with category tabs + search. Add items with variants + modifiers + quantity + notes. "Send to Kitchen" button → creates order + KOT via API. | Frontend | 6h | P0 |
| 5.2.5 | **Waiter PWA — notifications** | WebSocket connection. Show toast notification when kitchen marks item "Ready" for waiter's tables. Vibrate phone. | Frontend | 3h | P0 |
| 5.2.6 | **Waiter PWA — bill request** | "Request Bill" button for table → sends WebSocket event to Cashier PC. Cashier sees notification. | Frontend | 1.5h | P0 |
| 5.2.7 | **Kitchen Display PWA — app shell** | Full-screen landscape layout optimized for wall-mounted tablet. No navigation bar (single-purpose view). | Frontend | 2h | P0 |
| 5.2.8 | **Kitchen Display PWA — KOT board** | Shows all pending/in-progress KOTs as cards in a grid/column layout. Each card: table number, items list with qty, time elapsed (color changes: green < 10m, yellow 10-20m, red > 20m). "Start" and "Ready" buttons. Audio beep on new KOT. | Frontend | 6h | P0 |
| 5.2.9 | **Kitchen Display PWA — item status** | Tap individual item to mark "Preparing" or "Ready". When all items in KOT ready → KOT card moves to "Done" column (or disappears). | Frontend | 3h | P0 |
| 5.2.10 | **End-to-end multi-device test** | Set up: Cashier PC + waiter phone + kitchen tablet on same WiFi. Test complete flow. Disconnect internet mid-flow. Verify everything works on local network. | All | 4h | P0 |

**Sprint 5 Tests:**
- [ ] Embedded server starts and responds to `GET /v1/health` from another device on WiFi
- [ ] mDNS: phone discovers server without manual IP
- [ ] QR code scan opens waiter PWA correctly
- [ ] Waiter places order on phone → KOT appears on kitchen tablet within 2 seconds
- [ ] Kitchen marks "Ready" → waiter phone shows notification
- [ ] Disconnect internet → all devices continue working on local WiFi
- [ ] Reconnect internet → mode switches to Cloud (status icon changes)
- [ ] Smart API Client fails over correctly during mode switch
- [ ] Waiter PWA loads on low-end Android phone (test on ₹8,000 phone)

---

### Sprint 6 (Weeks 13-14): Hardware Integration + Polish

**Sprint Goal:** Hardware support (printers, cash drawer, barcode), advanced table/billing features, and UX polish based on restaurant feedback.

| # | Task | Details | Owner | Est. | Priority |
|---|------|---------|-------|------|----------|
| 6.1 | **Thermal printer ESC/POS refinement** | Test with 5+ Indian printer models. Handle paper width detection. Add restaurant logo print. QR code on receipt (for digital bill lookup). | Backend | 5h | P0 |
| 6.2 | **KOT print to kitchen printer** | Separate printer for KOT. Auto-print KOT when "Send to Kitchen" is pressed. Format: large table number, items with notes highlighted, timestamp. | Backend | 3h | P0 |
| 6.3 | **Cash drawer integration** | Open cash drawer command via ESC/POS (most drawers are printer-connected). Trigger on Cash payment finalization. | Backend | 2h | P1 |
| 6.4 | **Barcode reader integration** | USB barcode reader acts as keyboard input. Create a "Scan" mode in item search that captures barcode input and looks up item by barcode field. | Backend | 2h | P1 |
| 6.5 | **Reservation system** | `POST /v1/reservations` — date, time, party_size, customer_name, phone, table_id (optional), notes. Show on floor plan as yellow tables at reserved time. Auto-notify 15 min before. | Full-stack | 4h | P1 |
| 6.6 | **Discount management UI** | Settings page: create named discounts (Happy Hour 20%, Staff Meal 50%, Coupon ABC123). Apply by name during billing. Track usage. | Full-stack | 3h | P1 |
| 6.7 | **Settings & configuration page** | Outlet details (name, address, GSTIN, FSSAI), tax settings, service charge toggle + %, round-off rules, print settings (header, footer, logo), bill number prefix. | Frontend | 5h | P0 |
| 6.8 | **Sync engine — change tracking** | Every SQLite write also inserts into `sync_queue` table: `{ id, table_name, operation (INSERT/UPDATE/DELETE), row_data (JSON), created_at }`. | Backend | 4h | P0 |
| 6.9 | **Sync engine — push/pull** | On Cloud mode detected: (1) `POST /v1/sync/push` sends batched sync_queue entries. (2) `POST /v1/sync/pull` gets server changes since last_sync_timestamp. (3) Apply server changes to local SQLite. (4) Resolve conflicts (last-write-wins for most, server-wins for bills). (5) Clear synced entries from queue. | Backend | 8h | P0 |
| 6.10 | **Sync status UI** | Show sync queue depth (e.g., "12 changes pending sync"). Show last sync time. Manual "Sync Now" button. | Frontend | 2h | P0 |
| 6.11 | **Backup/Restore (local)** | Tauri command: backup SQLite DB to a user-chosen location. Restore from backup file. Automatic daily backup to app data folder (keep last 7). | Backend | 3h | P1 |
| 6.12 | **UX polish pass** | Keyboard shortcuts (F1-F6 for modules, F9 for quick bill, Esc to cancel), loading states, empty states, error toasts, confirmation dialogs for destructive actions. | Frontend | 5h | P0 |

---

### Sprint 7 (Weeks 15-16): Testing, Bug Fixing & Beta Launch

**Sprint Goal:** Thorough testing on real hardware, fix all P0 bugs, create Windows installer, deploy cloud server, launch beta with 5-10 restaurants.

#### Week 15: Testing & Bug Fixing

| # | Task | Details | Owner | Est. | Priority |
|---|------|---------|-------|------|----------|
| 7.1 | **End-to-end test suite** | Write Playwright tests for 10 critical flows: login, create menu item, place order, generate bill, print, void bill, take order from waiter phone, KOT in kitchen, sync test, report generation | QA / Full-stack | 8h | P0 |
| 7.2 | **Performance testing** | Test on low-end PC (2GB RAM, old CPU): app startup < 3s, bill generation < 100ms, 1000 bills search < 500ms, 500 menu items load < 200ms. Optimize bottlenecks. | Backend | 6h | P0 |
| 7.3 | **Real hardware test session** | Full day at a restaurant partner's location. Set up their hardware (their PC, their printer). Run through 50+ orders. Document all issues. | All | 8h | P0 |
| 7.4 | **Security audit** | Check: SQL injection (parameterized queries), XSS (React handles most), CSRF, auth bypass, rate limiting, file upload validation, sensitive data exposure in API responses | Backend | 6h | P0 |
| 7.5 | **Bug fixing sprint** | Fix all P0 bugs from testing. P1 bugs deferred if not blocking beta. | All | 16h | P0 |
| 7.6 | **Accessibility basics** | Keyboard navigation through all forms, proper ARIA labels on interactive elements, sufficient color contrast | Frontend | 4h | P1 |

#### Week 16: Installer, Deployment & Beta Launch

| # | Task | Details | Owner | Est. | Priority |
|---|------|---------|-------|------|----------|
| 7.7 | **Windows installer (MSI/NSIS)** | Tauri bundle configuration. Include: app binary, SQLite, bundled PWA files, printer drivers (optional). Code signing certificate (important: Windows SmartScreen blocks unsigned apps). | Backend | 4h | P0 |
| 7.8 | **Auto-update system** | Tauri's built-in updater. Update check on app startup + every 4 hours. Download in background. Prompt user to restart. Rollback on failure. | Backend | 3h | P0 |
| 7.9 | **Cloud server deployment** | Deploy Axum server to Hetzner/DigitalOcean. Docker Compose with: Axum API, PostgreSQL, Redis, NATS, Nginx (reverse proxy + SSL via Let's Encrypt). Set up: auto-restart, log rotation, automated backups. | Full-stack | 6h | P0 |
| 7.10 | **Web app deployment** | Deploy Next.js to same server (or Vercel). Point `app.restrosync.com` → web app. Point `api.restrosync.com` → Axum API. | Full-stack | 3h | P0 |
| 7.11 | **Monitoring setup** | Grafana + Prometheus for server metrics (CPU, RAM, request rate, error rate). Sentry for error tracking (both desktop and server). PagerDuty/Telegram alerts for downtime. | Full-stack | 4h | P1 |
| 7.12 | **User documentation** | Quick start guide (PDF): installation, first login, menu setup, taking orders, billing, reports. Max 10 pages with screenshots. Print copies for beta restaurants. | All | 6h | P0 |
| 7.13 | **Beta launch: install at 5-10 restaurants** | Visit each restaurant. Install app. Set up their menu, tables. Train 1-2 staff. Set up their printer. Run first 5 bills together. Collect first-day feedback. Set up WhatsApp group for support. | All + Business | Ongoing | P0 |

---

### Phase 2 Deliverables Checklist

| # | Deliverable | Status |
|---|-------------|--------|
| ✅ | Smart Connection Manager (Cloud ↔ Local ↔ Standalone auto-switch) | ☐ |
| ✅ | Embedded local Axum server in Tauri (serves devices on WiFi) | ☐ |
| ✅ | DataSource trait (PostgreSQL + SQLite abstraction) | ☐ |
| ✅ | mDNS auto-discovery for waiter/kitchen devices | ☐ |
| ✅ | Waiter PWA (order taking, table view, notifications) | ☐ |
| ✅ | Kitchen Display PWA (KOT board, status updates, audio alerts) | ☐ |
| ✅ | Smart API Client (auto-routes Cloud ↔ Local) | ☐ |
| ✅ | Sync engine (change tracking, push/pull, conflict resolution) | ☐ |
| ✅ | Thermal printer working with 3+ Indian models | ☐ |
| ✅ | Cash drawer + barcode reader support | ☐ |
| ✅ | Reservation system | ☐ |
| ✅ | Windows installer (.msi) with auto-update | ☐ |
| ✅ | Cloud server deployed (API + Web) | ☐ |
| ✅ | Monitoring (Grafana + Sentry) | ☐ |
| ✅ | User documentation (quick start guide) | ☐ |
| ✅ | Beta live at 5-10 restaurants | ☐ |
| ✅ | 80%+ Rust code coverage on core modules | ☐ |
| ✅ | 10 E2E Playwright tests passing | ☐ |

---

## 6. Phase 3 — Growth Features (Weeks 17-24)

### 🎯 Goal
Based on beta feedback, add the most requested features. Prepare for public launch. Scale to 50-100 restaurants.

### 🏁 Phase 3 Exit Criteria
- [ ] 50+ restaurants using the platform
- [ ] At least 2 aggregator integrations (Swiggy/Zomato)
- [ ] Multi-language support (English + Hindi + 2 regional)
- [ ] Public launch on website with self-serve signup
- [ ] Revenue: first paying customers

---

### Sprint 8-9 (Weeks 17-20): Top User Requests + Integrations

| # | Feature | Details | Duration | Priority |
|---|---------|---------|----------|----------|
| 8.1 | **Advanced KDS** | Multiple kitchen stations (Bar, Hot Kitchen, Cold Kitchen). Route KOT items to correct station. Station-specific display. | 5 days | P0 |
| 8.2 | **Customer-facing QR menu** | Generate QR code per table. Customer scans → sees menu (read-only) on their phone. Can browse, see prices, images. Phase 2: customer can place order from their phone. | 5 days | P0 |
| 8.3 | **Swiggy integration** | OAuth + API integration. Pull incoming Swiggy orders into RestroSync. Auto-create order, auto-print KOT. Update Swiggy status from RestroSync (Accepted, Preparing, Ready). | 5 days | P0 |
| 8.4 | **Zomato integration** | Same as Swiggy but for Zomato's API. Handle differences in order format, status mapping, menu sync. | 5 days | P0 |
| 8.5 | **WhatsApp bill sharing** | After bill finalization, "Send on WhatsApp" button. Uses WhatsApp Business API (or wa.me link as MVP). Share bill summary + PDF attachment. | 3 days | P1 |
| 8.6 | **SMS bill sharing** | "Send SMS" button. Use Twilio/MSG91. Send bill summary with a link to digital receipt. | 2 days | P1 |
| 8.7 | **Customer database** | `POST /v1/customers` — name, phone, email, visits count, total spent, preferences, notes. Link to bills. Auto-create on first bill (by phone number). | 3 days | P1 |
| 8.8 | **Loyalty program (basic)** | Configurable: earn X points per ₹100 spent. Redeem Y points = ₹Z discount. Show points on receipt. | 5 days | P2 |

### Sprint 10-11 (Weeks 21-24): Analytics + Multi-language + Public Launch

| # | Feature | Details | Duration | Priority |
|---|---------|---------|----------|----------|
| 9.1 | **Advanced analytics dashboard** | Interactive charts: sales trend (line), top items (bar), payment modes (pie), peak hours (heatmap), table turnover rate. Date range comparison (this week vs last week). | 5 days | P0 |
| 9.2 | **Custom report builder** | User selects: data source (bills/orders/inventory), columns, filters, grouping, date range. Generate table + chart. Save as template. Schedule for daily email. | 5 days | P2 |
| 9.3 | **Multi-language (i18n)** | Implement `i18next` for frontend, `fluent-rs` for backend. Translate to: Hindi, Tamil, Telugu, Kannada (start with Hindi + 1 regional). Menu items stay in original language — only UI labels translate. | 5 days | P0 |
| 9.4 | **Public website** | Marketing website: feature overview, pricing page, demo video, signup form, blog. SEO optimized. Built with Next.js (separate from app). | 5 days | P0 |
| 9.5 | **Self-serve onboarding** | New user signs up → creates tenant → creates first outlet → guided wizard: add menu (import from Excel/CSV), set up tables, configure GST, invite staff. Desktop download link. | 8 days | P0 |
| 9.6 | **Payment gateway** | Razorpay/Stripe integration for RestroSync subscription billing. Free tier → Starter → Professional → Business. Auto-activate features based on plan. | 5 days | P0 |
| 9.7 | **Customer feedback system** | After bill: show QR code → customer scans → rates experience (1-5 stars) + optional comment. Owner sees feedback in dashboard. | 3 days | P2 |
| 9.8 | **Multi-outlet support** | Owner with 3 outlets: switch between outlets in app. Shared menu templates across outlets. Consolidated reports across outlets. | 5 days | P1 |

---

## 7. Phase 4 — Scale (Week 25+)

### 🎯 Goal
Enterprise features, mobile apps, AI capabilities, and platform play. Scale to 1,000+ restaurants.

This phase is planned at a higher level — specific sprint plans will be created based on Phase 3 learnings and user data.

### Feature Priorities (Ordered by Expected Impact)

| # | Feature | Est. Duration | Business Value |
|---|---------|---------------|----------------|
| 4.1 | **Android app (React Native or Tauri Mobile)** | 6-8 weeks | High — mobile-first waiter experience |
| 4.2 | **iOS app** | 4-6 weeks (after Android) | Medium — smaller market in India but needed |
| 4.3 | **Plugin / extension system** | 6-8 weeks | High — ecosystem moat, third-party innovation |
| 4.4 | **AI demand forecasting** | 4-6 weeks | High — unique feature, reduces food waste |
| 4.5 | **Recipe cost optimization AI** | 3-4 weeks | Medium — suggests cheaper ingredient substitutions |
| 4.6 | **Dynamic pricing** | 3-4 weeks | Medium — auto-adjust prices by time/demand |
| 4.7 | **Multi-brand management** | 4-5 weeks | High — cloud kitchen and chain use case |
| 4.8 | **Franchise management** | 4-6 weeks | High — centralized control + local flexibility |
| 4.9 | **Employee management** | 3-4 weeks | Medium — scheduling, attendance, payroll hooks |
| 4.10 | **Accounting integration (Tally, Zoho)** | 3-4 weeks | High — reduces manual bookkeeping |
| 4.11 | **White-label solution** | 4-6 weeks | High — reseller/partner revenue channel |
| 4.12 | **Public API + marketplace** | 6-8 weeks | High — platform play, developer ecosystem |
| 4.13 | **Migrate to microservices (if needed)** | 8-12 weeks | Depends on scale bottlenecks |

---

## 8. Daily Development Workflow

### For Every Developer, Every Day

```
Morning (15 min):
├── Pull latest from main
├── Check CI status on any open PRs
├── Check Linear/Jira — pick your tasks for the day
└── Quick standup (async in Slack or 10-min video call)
    ├── What did you complete yesterday?
    ├── What are you working on today?
    └── Any blockers?

During Development:
├── Create feature branch: feat/billing-gst-engine
├── Write code + tests together (not tests after)
├── Commit frequently with descriptive messages
│   ├── "feat(billing): add GST calculation engine"
│   ├── "test(billing): add 10 GST calculation test cases"
│   └── "fix(billing): handle inter-state IGST correctly"
├── Run locally before pushing:
│   ├── cargo test --workspace
│   ├── cargo clippy --workspace
│   └── pnpm test (for frontend changes)
└── Push branch, create PR

Code Review (same day if possible):
├── PR must have: description, screenshots (UI changes), test evidence
├── Reviewer checks: correctness, tests, edge cases, performance
├── Review within 4 hours (max 24 hours)
├── Max 1 round of feedback → merge
└── CI must be green before merge

End of Day:
├── Update task status in Linear/Jira
├── Push any WIP to branch (don't leave uncommitted code overnight)
└── If blocked, post in Slack with details
```

### Sprint Ceremonies

| Ceremony | When | Duration | What |
|----------|------|----------|------|
| **Sprint Planning** | Monday morning, start of sprint | 1-2 hours | Review sprint goal, break into tasks, assign, estimate |
| **Daily Standup** | Every day (async in Slack or 10-min call) | 10 min | Yesterday / Today / Blockers |
| **Mid-Sprint Check** | Wednesday or Thursday of each week | 30 min | Are we on track? Any scope cuts needed? |
| **Sprint Review / Demo** | Friday, end of sprint | 1 hour | Demo working features. Show to restaurant partners if possible. |
| **Retrospective** | After sprint review | 30 min | What went well? What to improve? Action items. |

---

## 9. Definition of Done (DoD)

A task is "Done" only when **ALL** of these are true:

### For Backend Tasks
- [ ] Code compiles with zero warnings (`cargo clippy` clean)
- [ ] Unit tests written and passing (minimum: happy path + 2 edge cases)
- [ ] API endpoint tested manually (use `curl` or Hurl or Bruno)
- [ ] API response format matches the standard (success/data/errors/meta)
- [ ] Error cases return appropriate HTTP codes and error messages
- [ ] Database migrations are reversible (up + down)
- [ ] No sensitive data in logs (no passwords, tokens, etc.)
- [ ] Code reviewed and approved by at least 1 team member
- [ ] Merged to `main` with CI green

### For Frontend Tasks
- [ ] Component renders correctly (visual check + Vitest)
- [ ] Loading states and error states implemented
- [ ] Empty states implemented (e.g., "No menu items yet. Create your first item.")
- [ ] Responsive: works at 1280px width minimum (desktop), 375px minimum (PWA)
- [ ] Keyboard navigation works for forms
- [ ] TypeScript: no `any` types, all props typed
- [ ] Code reviewed and approved
- [ ] Merged to `main` with CI green

### For Full Features (Backend + Frontend)
- [ ] All of the above
- [ ] Feature works end-to-end (not just in isolation)
- [ ] Tested on desktop app (Tauri)
- [ ] Tested on web app (if applicable)
- [ ] Regression: existing features still work (run full test suite)
- [ ] Documentation updated (API docs auto-generated, README if needed)

---

## 10. Risk Checkpoints

At these specific points, the team must stop and evaluate risks:

### Checkpoint 1: End of Phase 0 (Week 2)
| Question | Green ✅ | Yellow ⚠️ | Red 🔴 |
|----------|---------|-----------|--------|
| Can all team members run the project locally? | Yes | Most can, 1-2 issues | Major setup problems |
| Is the Rust hiring gap a concern? | Team is comfortable with Rust | Some struggling, but learning | Team can't write Rust productively |
| Are restaurant partners engaged? | 3+ committed for beta | 1-2 maybe | None interested |

**If Red:** Consider switching server language to Go/TypeScript. Re-evaluate restaurant targeting.

### Checkpoint 2: End of Sprint 3 (Week 8)
| Question | Green ✅ | Yellow ⚠️ | Red 🔴 |
|----------|---------|-----------|--------|
| Is billing GST accurate? | All 10 test cases pass | 8/10 pass | < 8 pass |
| Does thermal printing work? | Works on 3+ models | Works on 1 model | Doesn't work reliably |
| Is the flow fast enough? | Order→Bill < 2 min | 2-5 min | > 5 min |

**If Red:** Billing accuracy must be fixed before proceeding. Consider hiring a CA to validate GST logic.

### Checkpoint 3: End of Phase 1 (Week 10)
| Question | Green ✅ | Yellow ⚠️ | Red 🔴 |
|----------|---------|-----------|--------|
| Can a restaurant operate using our app for a full day? | Yes (tested at partner) | With some workarounds | Missing critical features |
| Is the codebase maintainable? | Clean, well-tested | Some technical debt | Spaghetti code, hard to add features |
| Are we on budget? | Within 10% of budget | 10-30% over | > 30% over |

**If Red:** Pause new features. Refactor. Reduce scope. Consider raising funding sooner.

### Checkpoint 4: End of Phase 2 (Week 16) — Pre-Beta Gate
| Question | Green ✅ | Yellow ⚠️ | Red 🔴 |
|----------|---------|-----------|--------|
| Does auto-switching work reliably? | < 1% failure rate | 1-5% failure rate | > 5% failure rate |
| Can waiter phones connect to local server? | Works on 5+ phone models | Works on 3 models | Intermittent |
| Are beta restaurants ready? | 5+ restaurants installed | 2-3 installed | < 2 installed |

**If Red:** Delay beta by 2 weeks. Fix critical issues. Simplify (e.g., QR code only instead of mDNS).

---

## 11. Key Milestones & Go/No-Go Gates

```
Week 2  ──── MILESTONE: Development Environment Ready
              Go/No-Go: Can the team develop productively?

Week 4  ──── MILESTONE: Auth + Menu Complete
              Go/No-Go: Are the APIs well-designed? Is the pattern replicable?

Week 8  ──── MILESTONE: Complete Restaurant Flow (single device)
              Go/No-Go: Can a real restaurant cashier operate this?
              ⭐ FIRST REAL USER TEST — bring restaurant partner to office

Week 10 ──── MILESTONE: All 6 Modules Working
              Go/No-Go: Is the foundation solid for Phase 2?
              ⭐ DEMO to all restaurant partners

Week 12 ──── MILESTONE: Multi-Device Working
              Go/No-Go: Can waiter phones and kitchen tablets connect?
              ⭐ TEST at a real restaurant during service hours

Week 16 ──── MILESTONE: Beta Launch 🚀
              Go/No-Go: Is the app stable enough for real daily use?
              ⭐ INSTALL at 5-10 restaurants, train staff

Week 20 ──── MILESTONE: Integrations + i18n
              Go/No-Go: Are beta users happy? What's the retention rate?

Week 24 ──── MILESTONE: Public Launch 🎉
              Go/No-Go: Unit economics work? Conversion rate?
              ⭐ LAUNCH publicly, start marketing
```

---

## 12. Dependency Map

Understanding what depends on what prevents blocked developers:

```
PHASE 0: No dependencies (everything can be done in parallel by different people)

PHASE 1:
┌─────────────┐
│ Auth System  │ ◄── EVERYTHING depends on this (build first)
└──────┬──────┘
       │
       ├──────────────────┬───────────────────┬──────────────────┐
       ▼                  ▼                   ▼                  ▼
┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐
│ Menu Module  │  │ Table Module │  │ User Mgmt    │  │ Settings     │
│ (Sprint 1)   │  │ (Sprint 2)   │  │ (Sprint 1)   │  │ (Sprint 6)   │
└──────┬───────┘  └──────┬───────┘  └──────────────┘  └──────────────┘
       │                 │
       │     ┌───────────┘
       ▼     ▼
┌─────────────────┐
│  Order Module    │ ◄── Requires: Menu (to select items) + Tables (for dine-in)
│  (Sprint 2)      │
└───────┬─────────┘
        │
        ▼
┌─────────────────┐
│ Billing Module   │ ◄── Requires: Orders (to generate bill from)
│ (Sprint 3)       │
└───────┬─────────┘
        │
        ├─────────────────────┐
        ▼                     ▼
┌──────────────────┐  ┌─────────────────┐
│ Inventory Module │  │ Reports Module  │
│ (Sprint 4)       │  │ (Sprint 4)      │
│                  │  │                 │
│ Requires:        │  │ Requires:       │
│ • Menu (recipes) │  │ • Bills (data)  │
│ • Billing (auto  │  │ • Orders (data) │
│   deduction)     │  │ • Inventory     │
└──────────────────┘  └─────────────────┘

PHASE 2:
┌───────────────────────┐
│ DataSource Trait       │ ◄── Must be done FIRST in Phase 2
│ (refactor to abstract) │     All route handlers become DB-agnostic
└───────────┬───────────┘
            │
    ┌───────┴────────┐
    ▼                ▼
┌─────────────┐  ┌──────────────────────┐
│ Embedded    │  │ Smart Connection     │
│ Local Server│  │ Manager              │
│ (Axum)      │  │ (auto-detect mode)   │
└──────┬──────┘  └──────────┬───────────┘
       │                    │
       ├────────────────────┘
       ▼
┌─────────────────────┐
│ mDNS + QR Discovery │
└──────────┬──────────┘
           │
    ┌──────┴──────┐
    ▼             ▼
┌──────────┐  ┌──────────────┐
│ Waiter   │  │ Kitchen      │
│ PWA      │  │ Display PWA  │
└──────────┘  └──────────────┘
    │             │
    └──────┬──────┘
           ▼
┌──────────────────────┐
│ Sync Engine          │ ◄── Can be built in parallel with PWAs
│ (change tracking +   │     but needs DataSource trait done first
│  push/pull)          │
└──────────────────────┘

PARALLEL WORK OPPORTUNITIES (assign to different devs):
├── Backend Dev 1: DataSource trait → Embedded Server → Sync Engine
├── Backend Dev 2: Smart Conn Mgr → mDNS → Local Auth → Hardware
├── Frontend Dev 1: Waiter PWA (all screens)
├── Frontend Dev 2: Kitchen Display PWA + QR code + Mode Status UI
└── Full-stack: Settings polish + Reservation + Discount mgmt
```

---

## Quick Reference: What to Build Each Week

| Week | Backend Focus | Frontend Focus | Business Focus |
|------|--------------|---------------|----------------|
| **1** | Monorepo, Axum server, Docker, DB migrations | Tauri setup, React shell, shared packages | Restaurant visits, partnerships |
| **2** | Complete DB schema (all modules), seed data, CI | Design system, app shell layout, Figma | Beta restaurant commitments |
| **3** | Auth API (login, JWT, RBAC, Casbin) | Login UI, auth context, route protection | Collect menu data from partners |
| **4** | Menu CRUD APIs (categories, items, variants, modifiers) | Menu management UI (tree, editor, search) | Demo menu flow to partners |
| **5** | Table + Floor APIs, table status | Floor plan editor, table status view | Observe real restaurant workflows |
| **6** | Order CRUD, KOT generation, WebSocket | Order placement UI, running orders, KOT view | Demo order flow to partners |
| **7** | GST engine, billing API, payment API | — (let backend get billing right) | Validate GST with accountant |
| **8** | Print engine (thermal + A4) | Billing UI, payment UI, print preview | **Test with real printer** |
| **9** | Inventory CRUD, recipes, auto-deduction | Inventory UI, recipe editor, stock view | Partners: review their ingredient lists |
| **10** | Reports APIs (sales, GST, inventory, staff) | Dashboard, reports page, charts | **Full demo to all partners** |
| **11** | Embedded local server, DataSource trait, Smart Conn Mgr | Mode status UI, QR code generation | Prepare beta install plan |
| **12** | mDNS, local auth, serve PWA files | Waiter PWA, Kitchen Display PWA | Test multi-device at restaurant |
| **13** | Printer refinement, cash drawer, sync engine (tracking) | Hardware config UI, settings page, reservations | Validate hardware compatibility |
| **14** | Sync engine (push/pull), conflict resolution | Sync status UI, UX polish, keyboard shortcuts | Prepare user documentation |
| **15** | E2E tests, performance tests, security audit | Bug fixes, accessibility pass | **Real-world test at restaurant** |
| **16** | Windows installer, auto-update, cloud deployment | Bug fixes, final polish | **🚀 BETA LAUNCH** |
| **17-18** | Advanced KDS, QR menu API | Advanced KDS UI, QR menu frontend | Collect beta feedback |
| **19-20** | Swiggy + Zomato integration APIs | Integration config UI, WhatsApp/SMS sharing | Aggregator partnerships |
| **21-22** | Analytics APIs, custom report builder | Advanced charts, dashboard customization | Multi-language testing |
| **23-24** | Multi-language backend, self-serve onboarding API | i18n implementation, public website, signup flow | **🎉 PUBLIC LAUNCH** |

---

## Final Notes

### How to Use This Document

1. **Sprint Planning:** At the start of each sprint, read the relevant sprint section. Create tickets for each numbered task.
2. **Daily:** Reference the task list to know what to work on next. Check dependencies.
3. **Risk Checkpoints:** At each checkpoint week, honestly answer the evaluation questions.
4. **Scope Changes:** If user feedback says "Feature X is critical, Feature Y is not" — adjust. This roadmap is a plan, not a contract.

### What's NOT in This Roadmap (By Design)

- **Detailed UI wireframes** — That's in Figma (Designer creates these per sprint)
- **API request/response schemas** — That's in `PROJECT_BLUEPRINT.md` Section 11-12
- **Database schemas** — That's in `PROJECT_BLUEPRINT.md` Section 6-7
- **Infrastructure details** — That's in `PROJECT_BLUEPRINT.md` Section 9-10
- **Cost estimates** — That's in `PROJECT_BLUEPRINT.md` Section 14

### The Most Important Thing

> **Talk to restaurant owners every single week.** No amount of architecture or code quality matters if you build the wrong thing. The restaurants will tell you what they need. Listen.

---

*This roadmap is a living document. Update it after every sprint retrospective. Add learnings, adjust timelines, and refine priorities based on real-world feedback.*

**Last Updated:** June 2025
**Next Review:** End of Phase 0 (Week 2)
