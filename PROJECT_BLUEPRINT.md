# 🍽️ RestroSync — Project Blueprint & Technical Roadmap

## *A Next-Generation Restaurant Management Platform*

> **Document Version:** 1.1.0
> **Created:** June 2025
> **Last Updated:** June 2025 — Added Fully Dynamic Networking Architecture (Auto-Switching Modes)
> **Status:** Planning & Architecture Phase
> **Confidentiality:** Internal — Founders Only

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [Product Vision & Philosophy](#2-product-vision--philosophy)
3. [Competitive Analysis — Petpooja & Market Gaps](#3-competitive-analysis--petpooja--market-gaps)
4. [System Architecture](#4-system-architecture)
5. [Technology Stack — Detailed Breakdown](#5-technology-stack--detailed-breakdown)
6. [Core Modules — MVP Scope](#6-core-modules--mvp-scope)
7. [Database Design Philosophy](#7-database-design-philosophy)
8. [Fully Dynamic Networking — Auto-Switching Architecture](#8-fully-dynamic-networking--auto-switching-architecture)
9. [Multi-Device Communication — Waiter, Kitchen & Cashier Flow](#9-multi-device-communication--waiter-kitchen--cashier-flow)
10. [Cloud Architecture & Infrastructure](#10-cloud-architecture--infrastructure)
11. [Security Architecture](#11-security-architecture)
12. [API Design & Communication](#12-api-design--communication)
13. [Development Phases & Timeline](#13-development-phases--timeline)
14. [Team Structure & Roles](#14-team-structure--roles)
15. [Cost Estimation — Detailed Breakdown](#15-cost-estimation--detailed-breakdown)
16. [DevOps & CI/CD Pipeline](#16-devops--cicd-pipeline)
17. [Testing Strategy](#17-testing-strategy)
18. [Performance & Scalability Targets](#18-performance--scalability-targets)
19. [Future Expansion Roadmap](#19-future-expansion-roadmap)
20. [Risk Assessment & Mitigation](#20-risk-assessment--mitigation)
21. [Key Architectural Decisions Record (ADR)](#21-key-architectural-decisions-record-adr)
22. [Conclusion](#22-conclusion)

---

## 1. Executive Summary

**RestroSync** is a next-generation, full-featured restaurant management platform designed to directly compete with — and surpass — Petpooja in the Indian market.

The platform will support:

- **Windows Desktop Application** (primary interface for restaurant staff / cashier)
- **Web Application** (for owners, managers, and remote access)
- **Waiter Devices** (phones/tablets accessing the system via browser/PWA)
- **Kitchen Display** (tablet/screen showing live KOTs)
- **Fully Dynamic Networking** — the app **auto-detects** available connectivity and seamlessly switches between Cloud Mode, Local Network Mode, and Standalone Mode with **zero user intervention**

We are building with the philosophy of **"start lean, scale infinitely."** The MVP will cover the 6 core modules (Billing, Inventory, Menu, Orders, Reporting, Tables), but the architecture will be designed to absorb dozens of future modules without refactoring the foundation.

### Key Differentiators

| Feature | Petpooja | RestroSync |
|---|---|---|
| Offline-first with local storage | Limited | Full support — SQLite local DB |
| True hybrid (local + cloud sync) | No | Yes — seamless toggle |
| **Fully dynamic networking** | No — breaks without internet | **Auto-switches** Cloud ↔ Local WiFi ↔ Standalone |
| **Embedded local server on cashier PC** | No | Yes — Cashier PC serves waiters & kitchen via WiFi |
| Multi-device without internet | ❌ Not possible | ✅ Waiters, kitchen, cashier all work on local WiFi |
| Open plugin/extension system | No | Planned (Phase 3) |
| Performance on low-end hardware | Average | Optimized — Rust core engine |
| Privacy-first data option | No | Yes — fully local mode |
| Multi-language UI | Limited | Full i18n from day one |
| Real-time sync across devices | Basic | WebSocket + CRDT-based sync |
| Cost to restaurant | ₹1,000–5,000/mo | Competitive + free tier |

---

## 2. Product Vision & Philosophy

### 2.1 Vision Statement

> *"Empower every restaurant — from a single chai stall to a 50-outlet chain — with enterprise-grade management software that works everywhere, online or offline, without compromising speed, privacy, or simplicity."*

### 2.2 Core Principles

1. **Fully Dynamic Networking** — The app auto-detects connectivity and seamlessly switches between Cloud, Local Network (WiFi), and Standalone modes. The user never has to choose or configure a mode — it just works.

2. **Local-First, Cloud-Ready** — Every write goes to the local database first. Cloud is a bonus layer. This guarantees instant speed and zero downtime regardless of internet status.

3. **Speed is a Feature** — Billing and order operations must feel instant (< 100ms response). Restaurant staff cannot wait.

4. **Multi-Device Without Internet** — Waiters (phone/tablet), kitchen display, and cashier PC all communicate over the restaurant's local WiFi. No internet needed for multi-device operation.

5. **Privacy as a Choice** — Restaurants that want to keep their data local should be able to do so without any penalty in functionality.

6. **Modular by Design** — Every feature is a module. Modules can be enabled/disabled, updated independently, and extended.

7. **Indian Market First** — Designed for Indian GST compliance, UPI payments, regional languages, and low-bandwidth environments.

8. **Progressive Complexity** — Simple for a small restaurant, powerful for a chain. The UI adapts to the user's scale.

### 2.3 Target Users

| Segment | Description | Priority |
|---|---|---|
| Small Restaurants | 1-2 outlets, 1-5 staff | P0 (MVP) |
| Mid-size Restaurants | 3-10 outlets, dine-in + delivery | P0 (MVP) |
| Restaurant Chains | 10-50+ outlets, centralized management | P1 (Post-MVP) |
| Cloud Kitchens | Delivery-only, multi-brand | P1 |
| Food Courts | Shared billing, multiple vendors | P2 |
| Cafes & Bakeries | Simplified flow, counter billing | P0 (MVP) |

---

## 3. Competitive Analysis — Petpooja & Market Gaps

### 3.1 Petpooja Strengths

- Established brand with 75,000+ restaurants
- Comprehensive feature set
- Strong integrations with Zomato, Swiggy, Dunzo
- Reliable POS hardware partnerships
- Good customer support network

### 3.2 Petpooja Weaknesses & Market Gaps

| Gap | Impact | Our Opportunity |
|---|---|---|
| **No true offline mode** — Requires internet for most operations | Restaurants in areas with poor connectivity lose sales | Full offline-first architecture |
| **Slow on low-end hardware** — Electron/web-based bloat | Many small restaurants use older PCs | Rust + native rendering = 10x faster |
| **No local data option** — All data goes to their cloud | Privacy-conscious owners have no choice | Full local-only mode |
| **Rigid UI** — One-size-fits-all interface | Different restaurant types need different workflows | Adaptive, configurable UI |
| **Expensive for small restaurants** — ₹1,000-5,000+/month | Chai stalls and small dhabas can't afford it | Free tier + affordable paid plans |
| **Limited customization** — Can't add custom fields, reports | Chain restaurants need custom analytics | Plugin system + custom report builder |
| **Poor multi-language support** — Primarily English/Hindi | South Indian, Northeast markets underserved | Full i18n — 12+ Indian languages |
| **Basic inventory management** — No recipe-level tracking | Restaurants can't track food cost accurately | Recipe-level inventory with waste tracking |
| **No self-hosting option** — SaaS only | Some businesses want full control | Docker-based self-hosting option |
| **Vendor lock-in** — Hard to export data | Restaurants feel trapped | Open data format + easy export |

### 3.3 Other Competitors

| Competitor | Strength | Weakness |
|---|---|---|
| **POSist** | Enterprise features | Expensive, complex setup |
| **Torqus** | Good UI | Limited offline, small team |
| **Jeavio NuznInfotech** | Budget-friendly | Limited features |
| **LimeTray** | Online ordering focus | Weak POS |
| **Rista** | Indian-focused | Limited integrations |

### 3.4 Our Competitive Moat

1. **Hybrid Architecture** — No competitor offers true local-first + cloud sync
2. **Performance** — Rust-powered core will outperform all Electron/web-based competitors
3. **Open & Extensible** — Plugin system will create an ecosystem
4. **Privacy Option** — Unique selling point for privacy-conscious businesses
5. **Price Disruption** — Free tier to capture market share

---

## 4. System Architecture

### 4.1 High-Level Architecture Overview — Fully Dynamic Networking

The system is built around a **Smart Connection Manager** that auto-detects network conditions and routes all communication through the best available path — **Cloud, Local WiFi, or Standalone** — without any user configuration.

```
┌──────────────────────────────────────────────────────────────────────┐
│                        CLIENT LAYER                                  │
│                                                                      │
│  ┌─────────────────────┐  ┌────────────┐  ┌──────────────────────┐ │
│  │   Windows Desktop    │  │  Waiter     │  │  Kitchen Display     │ │
│  │   (Tauri + React)    │  │  Phone/Tab  │  │  (Tablet/Screen)     │ │
│  │                      │  │  (Browser/  │  │  (Browser/PWA)       │ │
│  │  ┌────────────────┐  │  │   PWA)      │  │                      │ │
│  │  │  Local SQLite   │  │  └──────┬─────┘  └───────────┬──────────┘ │
│  │  │  Database       │  │         │                     │           │
│  │  └────────────────┘  │         │                     │           │
│  │  ┌────────────────┐  │         │                     │           │
│  │  │  Rust Core      │  │         │                     │           │
│  │  │  Engine         │  │         │                     │           │
│  │  │  + EMBEDDED     │  │         │                     │           │
│  │  │  LOCAL SERVER   │◄─┼─────────┴─────────────────────┘           │
│  │  │  (serves waiter │  │   (connect via local WiFi when           │
│  │  │   & kitchen on  │  │    internet is down)                     │
│  │  │   local WiFi)   │  │                                          │
│  │  └────────────────┘  │                                          │
│  └──────────┬──────────┘  ┌──────────────────────────────────┐     │
│             │              │        Web Application           │     │
│             │              │     (React + Next.js)            │     │
│             │              │  (for owners — remote access)    │     │
│             │              └──────────────┬───────────────────┘     │
│             │                             │                         │
└─────────────┼─────────────────────────────┼─────────────────────────┘
              │                             │
              │  ┌────────────────────────┐ │
              │  │  SMART CONNECTION MGR  │ │
              │  │                        │ │
              │  │  Detects auto:         │ │
              │  │  ☁️ Cloud available?    │ │
              │  │  📡 Local WiFi only?   │ │
              │  │  💾 Standalone?        │ │
              │  │                        │ │
              │  │  Switches seamlessly   │ │
              │  │  User never notices    │ │
              │  └──────────┬─────────────┘ │
              │             │               │
              ▼             ▼               ▼
┌──────────────────────────────────────────────────────────────────────┐
│                 CLOUD LAYER (when internet available)                 │
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │                      API GATEWAY                              │   │
│  │               (Kong / Traefik / Custom Rust)                  │   │
│  │  ┌──────────┐ ┌───────────┐ ┌───────────┐ ┌──────────────┐  │   │
│  │  │  Auth    │ │  Rate     │ │  Load     │ │  Request     │  │   │
│  │  │  Verify  │ │  Limiting │ │  Balancer │ │  Routing     │  │   │
│  │  └──────────┘ └───────────┘ └───────────┘ └──────────────┘  │   │
│  └──────────────────────────────┬───────────────────────────────┘   │
│                                 │                                    │
│  ┌──────────────────────────────┼──────────────────────────────┐    │
│  │               SERVICES LAYER (Modular Monolith → μServices) │    │
│  │                              │                               │    │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐      │    │
│  │  │ Billing  │ │Inventory │ │  Menu    │ │  Order   │      │    │
│  │  │ Service  │ │ Service  │ │ Service  │ │ Service  │      │    │
│  │  └──────────┘ └──────────┘ └──────────┘ └──────────┘      │    │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐      │    │
│  │  │  Table   │ │   Auth   │ │  Sync    │ │  Report  │      │    │
│  │  │ Service  │ │ Service  │ │ Service  │ │ Service  │      │    │
│  │  └──────────┘ └──────────┘ └──────────┘ └──────────┘      │    │
│  └────────────────────────────────────────────────────────────┘    │
│                                                                      │
│  ┌────────────────────────────────────────────────────────────────┐ │
│  │                         DATA LAYER                              │ │
│  │  ┌──────────────┐ ┌──────────────┐ ┌──────────────────────┐   │ │
│  │  │  PostgreSQL   │ │    Redis      │ │   NATS (MQ)          │   │ │
│  │  │  (Primary DB) │ │  (Cache)      │ │   (Pub/Sub)          │   │ │
│  │  └──────────────┘ └──────────────┘ └──────────────────────┘   │ │
│  │  ┌──────────────┐ ┌──────────────┐ ┌──────────────────────┐   │ │
│  │  │  ClickHouse   │ │ MinIO / S3   │ │   Meilisearch        │   │ │
│  │  │  (Analytics)  │ │ (Files)      │ │   (Search)           │   │ │
│  │  └──────────────┘ └──────────────┘ └──────────────────────┘   │ │
│  └────────────────────────────────────────────────────────────────┘ │
│                                                                      │
└──────────────────────────────────────────────────────────────────────┘
```

### 4.2 Architecture Pattern — Modular Monolith → Microservices

**Phase 1 (MVP):** Start as a **Modular Monolith**
- Single deployable Rust binary with internal module boundaries
- Shared database with schema-level separation
- Internal function calls between modules (no network overhead)
- Faster to develop, test, and deploy

**Phase 2 (Growth):** Extract into **Microservices**
- Split modules into independent services as team and traffic grow
- Introduce message queue for async communication
- Independent deployment and scaling per service

**Why this approach?**
- Avoids "distributed monolith" anti-pattern
- Gives startup speed of a monolith with future flexibility of microservices
- Martin Fowler's "Monolith First" principle

### 4.3 Desktop Application Architecture (with Embedded Local Server)

The Cashier PC runs the Tauri desktop app which contains **two roles**:
1. **Desktop App UI** — for the cashier to operate billing, menu, etc.
2. **Embedded Local HTTP/WebSocket Server** — serves waiter phones, kitchen tablets, and any other device on the local WiFi.

```
┌────────────────────────────────────────────────────────────────┐
│                   Tauri Shell (Cashier PC)                       │
│              (Lightweight Webview + Embedded Server)             │
│                                                                  │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │            React Frontend (Cashier UI)                     │   │
│  │                                                            │   │
│  │  ┌─────────┐ ┌─────────┐ ┌───────────┐ ┌─────────────┐  │   │
│  │  │ Billing │ │  Menu   │ │  Orders   │ │   Tables    │  │   │
│  │  │   UI    │ │   UI    │ │    UI     │ │     UI      │  │   │
│  │  └────┬────┘ └────┬────┘ └─────┬─────┘ └──────┬──────┘  │   │
│  │       │           │            │               │          │   │
│  │  ┌────┴───────────┴────────────┴───────────────┴──────┐  │   │
│  │  │            Tauri Command Bridge                      │  │   │
│  │  │         (invoke Rust from JS)                        │  │   │
│  │  └───────────────────────┬──────────────────────────────┘  │   │
│  └──────────────────────────┼─────────────────────────────────┘   │
│                             │                                      │
│  ┌──────────────────────────┼─────────────────────────────────┐   │
│  │           Rust Backend (Core Engine)                         │   │
│  │                          │                                   │   │
│  │  ┌──────────────────────┴────────────────────────────┐     │   │
│  │  │            Business Logic Layer                     │     │   │
│  │  │  ┌─────────┐ ┌────────┐ ┌─────────┐ ┌──────────┐ │     │   │
│  │  │  │Billing  │ │ Menu   │ │ Order   │ │Inventory │ │     │   │
│  │  │  │Engine   │ │Engine  │ │Engine   │ │Engine    │ │     │   │
│  │  │  └─────────┘ └────────┘ └─────────┘ └──────────┘ │     │   │
│  │  └───────────────────────┬───────────────────────────┘     │   │
│  │                          │                                   │   │
│  │  ┌───────────────────────┴───────────────────────────┐     │   │
│  │  │            Data Access Layer                        │     │   │
│  │  │                                                     │     │   │
│  │  │  ┌──────────┐  ┌──────────────┐  ┌──────────────┐ │     │   │
│  │  │  │  SQLite   │  │  Sync Engine │  │  Connection  │ │     │   │
│  │  │  │  (Local)  │  │ (Cloud↔Local)│  │  Manager     │ │     │   │
│  │  │  └──────────┘  └──────────────┘  │ (Auto-Mode)  │ │     │   │
│  │  │                                   └──────────────┘ │     │   │
│  │  └────────────────────────────────────────────────────┘     │   │
│  │                                                              │   │
│  │  ┌──────────────────────────────────────────────────────┐   │   │
│  │  │      ⭐ EMBEDDED LOCAL HTTP + WEBSOCKET SERVER ⭐     │   │   │
│  │  │                                                        │   │   │
│  │  │  Runs on: http://192.168.x.x:8080 (local WiFi IP)    │   │   │
│  │  │                                                        │   │   │
│  │  │  Serves:                                               │   │   │
│  │  │  ├── Waiter UI (phone/tablet browser)                 │   │   │
│  │  │  ├── Kitchen Display (tablet browser)                 │   │   │
│  │  │  ├── REST API for all operations                      │   │   │
│  │  │  ├── WebSocket for real-time KOT/order updates        │   │   │
│  │  │  └── mDNS discovery (devices auto-find server)        │   │   │
│  │  │                                                        │   │   │
│  │  │  Same API contract as Cloud server — devices don't    │   │   │
│  │  │  know or care if they're talking to local or cloud    │   │   │
│  │  └──────────────────────────────────────────────────────┘   │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                                                                      │
└──────────────────────────────────────────────────────────────────────┘
```

### 4.4 The Three Connectivity Modes (Auto-Detected)

```
┌─────────────────────────────────────────────────────────────────────┐
│              APP AUTO-DETECTION FLOW (runs every 5 seconds)          │
│                                                                      │
│  App Starts / Connectivity Change Detected                           │
│       │                                                              │
│       ▼                                                              │
│  ┌─────────────────────────────────────┐                            │
│  │  Can I reach Cloud Server?           │                            │
│  │  (ping api.restrosync.com, <2 sec)  │                            │
│  └──────────┬──────────────────────────┘                            │
│             │                                                        │
│        YES  │                         NO                             │
│             ▼                          │                              │
│  ┌──────────────────────┐              ▼                              │
│  │  ☁️ MODE 1: CLOUD     │    ┌──────────────────────────────┐       │
│  │                       │    │  Is there a Local Server     │       │
│  │  • All devices talk   │    │  on WiFi? (mDNS / scan)     │       │
│  │    to cloud API       │    └──────────┬───────────────────┘       │
│  │  • Real-time via      │               │                           │
│  │    cloud WebSocket    │          YES  │             NO             │
│  │  • Owner can access   │               ▼              │             │
│  │    from anywhere      │    ┌──────────────────┐      ▼             │
│  │  • Local SQLite still │    │ 📡 MODE 2: LOCAL  │  ┌──────────────┐│
│  │    gets a copy        │    │    NETWORK        │  │ 💾 MODE 3:   ││
│  │    (fast reads)       │    │                   │  │  STANDALONE  ││
│  │                       │    │ • Waiter/Kitchen  │  │              ││
│  │  Status: ☁️ Green      │    │   connect to      │  │ • Single     ││
│  └──────────────────────┘    │   Cashier PC      │  │   device     ││
│                               │   (192.168.x.x)  │  │ • All data   ││
│                               │ • Full function   │  │   local      ││
│                               │ • No internet     │  │ • Queues all ││
│                               │   needed          │  │   changes    ││
│                               │ • Queues cloud    │  │   for sync   ││
│                               │   sync for later  │  │              ││
│                               │                   │  │ Status: 💾   ││
│                               │ Status: 📡 Blue   │  │  Yellow      ││
│                               └──────────────────┘  └──────────────┘│
│                                                                      │
│  ⚡ SWITCHING IS INVISIBLE — takes < 2 seconds                       │
│  ⚡ All modes use the SAME API contract                              │
│  ⚡ Data is ALWAYS written to local SQLite first                     │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### 4.5 What Happens During Mode Transitions (Real-World)

```
SCENARIO: Cafe in Mumbai (has internet, WiFi, 3 waiters, 1 kitchen display)
────────────────────────────────────────────────────────────────────────────

9:00 AM — Cafe opens. Internet is working.
          App detects → ☁️ CLOUD MODE
          Waiters connect to cloud API
          Kitchen display connects to cloud WebSocket
          Owner at home sees live dashboard ✅

2:15 PM — ISP goes down. Internet lost.
          App detects in < 2 seconds
          Auto-switches → 📡 LOCAL NETWORK MODE
          Waiters now connect to Cashier PC (192.168.1.5:8080)
          Kitchen display reconnects to Cashier PC WebSocket
          ⚡ Waiter placing order mid-switch: ORDER STILL GOES THROUGH
          ⚡ No one notices the switch
          All new data queues for cloud sync

3:45 PM — Internet comes back.
          App detects → ☁️ CLOUD MODE
          Background sync pushes 90 minutes of queued data to cloud
          Owner at home sees all data appear on dashboard
          Waiters seamlessly back on cloud API

─────────────────────────────────────────────────────────────────────────

SCENARIO: Highway Dhaba (no internet ever, has WiFi router, 2 waiters)
─────────────────────────────────────────────────────────────────────────

Always → 📡 LOCAL NETWORK MODE
Cashier PC runs embedded server
Waiters use phone browser → http://192.168.1.1:8080
Kitchen sees orders on mounted tablet
All data in local SQLite on Cashier PC
Owner occasionally uses phone hotspot → data syncs to cloud

─────────────────────────────────────────────────────────────────────────

SCENARIO: Tiny chai stall (1 person, no WiFi, just a PC)
─────────────────────────────────────────────────────────────────────────

Always → 💾 STANDALONE MODE
Everything on one PC
Bills, menu, inventory — all local
Connects to WiFi at home → syncs if they want cloud backup
```

### 4.6 Key Architecture Properties

| Property | Implementation |
|---|---|
| **Fully Dynamic Networking** | Smart Connection Manager auto-switches Cloud ↔ Local WiFi ↔ Standalone |
| **Embedded Local Server** | Cashier PC serves waiter & kitchen devices on local WiFi (Axum) |
| **Local-First Writes** | Every write hits local SQLite first, then propagates to cloud |
| **Scalability** | Horizontal scaling via container orchestration (K8s) for cloud |
| **Resilience** | Circuit breakers, retry logic, graceful degradation, mode fallback |
| **Performance** | Rust core, Redis caching, connection pooling |
| **Multi-Device (No Internet)** | Waiters + Kitchen + Cashier all work on local WiFi |
| **Multi-tenancy** | Schema-per-tenant in PostgreSQL (cloud) |
| **Extensibility** | Plugin architecture with WASM support |
| **Observability** | Structured logging, metrics, distributed tracing |

---

## 5. Technology Stack — Detailed Breakdown

### 5.1 Desktop Application (Cashier PC — Also Acts as Local Server)

| Layer | Technology | Why |
|---|---|---|
| **Shell/Runtime** | **Tauri 2.x** | 10x smaller than Electron, uses native webview, Rust backend. ~3MB vs Electron's ~150MB. |
| **Frontend** | **React 19 + TypeScript** | Massive ecosystem, easy hiring, fast development |
| **UI Framework** | **Shadcn/UI + Tailwind CSS 4** | Beautiful, accessible components, fully customizable |
| **State Management** | **Zustand + TanStack Query** | Lightweight, performant, great for offline-first |
| **Local Database** | **SQLite (via rusqlite)** | Battle-tested, zero-config, handles millions of records |
| **Backend Core** | **Rust** | Memory safety, blazing performance, compiles to native binary |
| **Embedded Local Server** | **Axum (lightweight instance)** | Serves waiter/kitchen devices on local WiFi. Same framework as cloud server. |
| **Local Device Discovery** | **mDNS (mdns-sd crate)** | Waiter phones auto-discover Cashier PC on WiFi without manual IP entry |
| **Local Real-time** | **WebSocket (tokio-tungstenite)** | Instant KOT push to kitchen, order updates to waiters — all over local WiFi |
| **Build Tool** | **Vite 6** | Fastest frontend build tool |
| **IPC** | **Tauri Command System** | Type-safe communication between JS frontend and Rust backend |

### 5.2 Web Application / Waiter & Kitchen UI

| Layer | Technology | Why |
|---|---|---|
| **Framework** | **Next.js 15 (App Router)** | SSR/SSG for owner dashboard; static export for waiter/kitchen UI |
| **Frontend** | **React 19 + TypeScript** | Code sharing with desktop app |
| **UI Framework** | **Same Shadcn/UI + Tailwind** | Consistent look across desktop, web, waiter, kitchen |
| **State Management** | **Zustand + TanStack Query** | Same as desktop for code reuse |
| **Real-time** | **WebSocket (tokio-tungstenite)** | Live order updates, KOT push — works on both cloud and local WiFi |
| **PWA** | **next-pwa / Vite PWA** | Installable on waiter phones and kitchen tablets. Works offline-capable. |
| **Smart API Client** | **Custom fetch wrapper** | Auto-routes requests to cloud URL or local server IP based on current mode |
| **Device Discovery** | **mDNS client (browser)** | Waiter phone auto-discovers local server. Fallback: manual IP or QR code scan. |

### 5.3 Backend / Server

| Layer | Technology | Why |
|---|---|---|
| **Language** | **Rust** | Performance, safety, low resource usage (save on server costs) |
| **Web Framework** | **Axum** | Built on Tokio, modular, great middleware system, production-ready |
| **ORM** | **SeaORM** | Async, supports PostgreSQL + SQLite, migration system |
| **Authentication** | **Custom JWT + Argon2** | Industry-standard, no third-party dependency for core auth |
| **Authorization** | **Casbin-RS (RBAC/ABAC)** | Flexible policy-based access control |
| **API Documentation** | **utoipa (OpenAPI/Swagger)** | Auto-generated API docs from Rust code |
| **Task Scheduling** | **Tokio Cron Scheduler** | Background jobs (reports, sync, cleanup) |
| **PDF Generation** | **printpdf + custom engine** | Bills, invoices, reports — all generated in Rust |
| **Email** | **lettre** | Transactional emails |

### 5.4 Databases & Storage

| Type | Technology | Purpose |
|---|---|---|
| **Primary Database** | **PostgreSQL 17** | Main cloud database — ACID, JSON support, great indexing |
| **Local Database** | **SQLite 3** | Desktop offline storage — embedded, zero-config |
| **Cache** | **Redis 7 / Valkey** | Session cache, hot data, rate limiting, pub/sub |
| **Analytics DB** | **ClickHouse** | OLAP — fast aggregations for reports & dashboards (Phase 2) |
| **Search** | **Meilisearch** | Menu search, order search — lightweight alternative to Elasticsearch |
| **File Storage** | **MinIO (self-host) / S3** | Images, exports, backups |
| **Message Queue** | **NATS** | Lightweight, fast pub/sub for inter-service communication |

### 5.5 Infrastructure & DevOps

| Purpose | Technology | Why |
|---|---|---|
| **Containerization** | **Docker** | Consistent environments |
| **Orchestration** | **Docker Compose (MVP) → K8s (Scale)** | Start simple, scale when needed |
| **CI/CD** | **GitHub Actions** | Free for open-source, great ecosystem |
| **Cloud Provider** | **AWS (primary) + Hetzner (cost-saving)** | AWS for production, Hetzner for dev/staging (80% cheaper) |
| **CDN** | **CloudFront / Cloudflare** | Static asset delivery |
| **Monitoring** | **Grafana + Prometheus** | Metrics and dashboards |
| **Logging** | **Loki + structured logging** | Centralized log aggregation |
| **Tracing** | **Jaeger / OpenTelemetry** | Distributed request tracing |
| **Reverse Proxy** | **Traefik** | Auto SSL, service discovery, load balancing |
| **DNS** | **Cloudflare** | Fast DNS, DDoS protection |

### 5.6 Shared / Cross-Cutting

| Purpose | Technology |
|---|---|
| **Monorepo Management** | Turborepo |
| **Code Formatting** | Prettier (JS/TS) + rustfmt (Rust) |
| **Linting** | ESLint + Clippy |
| **Type Sharing** | ts-rs (generates TypeScript types from Rust structs) |
| **Internationalization** | i18next (frontend) + fluent-rs (backend) |
| **Date/Time** | chrono (Rust) + date-fns (JS) |
| **Validation** | Zod (frontend) + validator (Rust) |
| **Error Tracking** | Sentry |

### 5.7 Why Rust? (Detailed Justification)

Choosing Rust as the core language is a strategic decision:

1. **Performance** — Rust compiles to native machine code. Billing calculations, report generation, and data processing will be 10-100x faster than Node.js/Python.

2. **Memory Efficiency** — No garbage collector means predictable performance and lower server costs. A Rust server handling 10,000 req/s might use 50MB RAM vs 500MB for Node.js.

3. **Safety** — Rust's ownership system prevents memory bugs, data races, and null pointer exceptions at compile time. Fewer production crashes.

4. **Shared Core** — The same Rust business logic runs on the desktop (Tauri) and the server (Axum). Write once, run everywhere.

5. **Desktop + Server** — Tauri uses Rust natively. Having the server also in Rust means one language for the entire backend stack.

6. **Cost Savings** — Lower server resources = lower cloud bills. At scale (10,000+ restaurants), this saves lakhs per month.

7. **Hiring** — While Rust developers are fewer, the quality is consistently high. Rust developers tend to write better-architected code.

### 5.8 Why Not Electron?

| Factor | Electron | Tauri |
|---|---|---|
| Bundle Size | ~150MB | ~3-5MB |
| RAM Usage | 200-500MB | 30-80MB |
| Startup Time | 3-5 seconds | < 1 second |
| CPU at Idle | 5-15% | < 1% |
| Backend | Node.js | Rust |
| Security | Chromium attack surface | Minimal attack surface |
| Auto-update | Built-in | Built-in |

For a POS system that runs all day on potentially old hardware, Tauri's efficiency is a massive advantage.

---

## 6. Core Modules — MVP Scope

### 6.1 Module Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    RESTROSYNC MVP MODULES                     │
│                                                              │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐ │
│  │  BILLING    │  │  INVENTORY  │  │  MENU MANAGEMENT    │ │
│  │             │  │             │  │                     │ │
│  │ • Quick Bill│  │ • Stock In  │  │ • Categories        │ │
│  │ • GST Calc  │  │ • Stock Out │  │ • Items + Variants  │ │
│  │ • Split Bill│  │ • Low Stock │  │ • Pricing           │ │
│  │ • Discounts │  │ • Recipes   │  │ • Modifiers         │ │
│  │ • Payments  │  │ • Wastage   │  │ • Availability      │ │
│  │ • Print     │  │ • Vendors   │  │ • Images            │ │
│  └─────────────┘  └─────────────┘  └─────────────────────┘ │
│                                                              │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐ │
│  │   ORDER     │  │  REPORTING  │  │  TABLE MANAGEMENT   │ │
│  │ MANAGEMENT  │  │             │  │                     │ │
│  │             │  │ • Sales     │  │ • Floor Plan        │ │
│  │ • Dine-in   │  │ • Inventory │  │ • Table Status      │ │
│  │ • Takeaway  │  │ • Tax       │  │ • Reservations      │ │
│  │ • Delivery  │  │ • Item-wise │  │ • Merge/Split       │ │
│  │ • KOT       │  │ • Staff     │  │ • Transfer          │ │
│  │ • Status    │  │ • Custom    │  │ • Capacity          │ │
│  └─────────────┘  └─────────────┘  └─────────────────────┘ │
│                                                              │
│  ┌──────────────────────────────────────────────────────┐   │
│  │              CROSS-CUTTING CONCERNS                    │   │
│  │                                                        │   │
│  │  Auth & RBAC  │  Multi-outlet  │  Sync Engine         │   │
│  │  GST Engine   │  Print Engine  │  Backup/Restore      │   │
│  └──────────────────────────────────────────────────────┘   │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### 6.2 Module: Billing System

**Priority:** P0 — This is the heart of the product.

#### Features

| Feature | Description | MVP? |
|---|---|---|
| Quick Billing | Fast item selection, quantity, generate bill | ✅ |
| GST Calculation | CGST/SGST/IGST auto-calculation based on location | ✅ |
| Bill Splitting | Split by amount, percentage, items, or per person | ✅ |
| Discounts | Flat, percentage, item-level, bill-level, coupon | ✅ |
| Multiple Payment Modes | Cash, Card, UPI, Wallet, Split Payment | ✅ |
| Bill Print (Thermal) | 58mm/80mm thermal printer support | ✅ |
| Bill Print (A4) | Full-page invoice for GST compliance | ✅ |
| KOT (Kitchen Order Ticket) | Auto-print to kitchen on order confirmation | ✅ |
| Bill History | Search, filter, reprint past bills | ✅ |
| Credit/Advance | Customer credit system | ✅ |
| Bill Modification | Edit bill before finalization (with audit log) | ✅ |
| Round-off | Configurable rounding rules | ✅ |
| Service Charge | Configurable service charge percentage | ✅ |
| Happy Hour Pricing | Time-based automatic pricing | Phase 2 |
| Loyalty Points | Earn and redeem points | Phase 2 |

#### Billing Flow

```
Customer Seated/Walk-in
        │
        ▼
  Create New Order
  (Link to table or takeaway/delivery)
        │
        ▼
  Add Items to Order
  (Search, category browse, favorites)
        │
        ├──▶ Send KOT to Kitchen ──▶ Print KOT
        │
        ▼
  Review Order
  (Modify quantities, add notes)
        │
        ▼
  Apply Discounts/Offers
        │
        ▼
  Calculate Totals
  (Subtotal + GST + Service Charge - Discounts)
        │
        ▼
  Select Payment Mode(s)
  (Cash / Card / UPI / Split)
        │
        ▼
  Finalize Bill
  (Generate bill number, update inventory)
        │
        ├──▶ Print Bill ──▶ Thermal/A4
        ├──▶ Send Digital Bill (WhatsApp/SMS/Email)
        │
        ▼
  Update Reports & Analytics
```

#### Data Model (Core)

```
Bill {
    id: UUID
    bill_number: String (formatted: "RS-2025-000001")
    outlet_id: UUID
    table_id: Option<UUID>
    order_type: Enum (DineIn, Takeaway, Delivery)
    customer_id: Option<UUID>
    
    subtotal: Decimal
    discount_amount: Decimal
    tax_amount: Decimal
    service_charge: Decimal
    round_off: Decimal
    total_amount: Decimal
    
    payment_status: Enum (Pending, Partial, Paid, Refunded)
    payments: Vec<Payment>
    
    items: Vec<BillItem>
    
    created_by: UUID (staff)
    created_at: DateTime
    finalized_at: Option<DateTime>
    
    gstin: Option<String>
    customer_gstin: Option<String>
    
    metadata: JSON
}

BillItem {
    id: UUID
    bill_id: UUID
    menu_item_id: UUID
    variant_id: Option<UUID>
    name: String
    quantity: Decimal
    unit_price: Decimal
    discount: Decimal
    tax_rate: Decimal
    tax_amount: Decimal
    total: Decimal
    modifiers: Vec<Modifier>
    notes: Option<String>
    kot_status: Enum (Pending, Sent, Preparing, Ready, Served)
}

Payment {
    id: UUID
    bill_id: UUID
    mode: Enum (Cash, Card, UPI, Wallet, Credit)
    amount: Decimal
    reference: Option<String>
    status: Enum (Success, Failed, Pending)
    processed_at: DateTime
}
```

### 6.3 Module: Inventory Management

**Priority:** P0

#### Features

| Feature | Description | MVP? |
|---|---|---|
| Raw Material Management | Add/edit raw materials with units | ✅ |
| Stock In (Purchase) | Record purchases from vendors | ✅ |
| Stock Out (Consumption) | Auto-deduct on billing via recipes | ✅ |
| Manual Stock Adjustment | For wastage, theft, damage | ✅ |
| Low Stock Alerts | Configurable thresholds per item | ✅ |
| Recipe Management | Define ingredients per menu item | ✅ |
| Vendor Management | Vendor profiles, contacts, history | ✅ |
| Purchase Orders | Create and track POs | ✅ |
| Stock Valuation | FIFO/Weighted Average costing | ✅ |
| Wastage Tracking | Record and categorize wastage | ✅ |
| Unit Conversion | kg↔g, L↔mL, etc. | ✅ |
| Batch Tracking | Track batches for perishables | Phase 2 |
| Expiry Management | Track expiry dates | Phase 2 |
| Barcode Scanning | Scan items during stock in/audit | Phase 2 |
| Auto-reorder | Automatic PO when stock hits threshold | Phase 2 |

#### Inventory Deduction Flow

```
Bill Finalized
      │
      ▼
For each BillItem:
      │
      ▼
Lookup Recipe for menu_item + variant
      │
      ▼
For each ingredient in recipe:
      │
      ├── Calculate: quantity_used = recipe_qty × bill_item_qty
      │
      ├── Deduct from current stock
      │
      ├── If stock < threshold → Trigger Low Stock Alert
      │
      └── Log StockMovement (type: AUTO_CONSUMPTION)
      │
      ▼
Update real-time stock dashboard
```

#### Data Model (Core)

```
RawMaterial {
    id: UUID
    outlet_id: UUID
    name: String
    category: String
    unit: Enum (Kg, Gram, Litre, ML, Piece, Dozen, ...)
    current_stock: Decimal
    min_stock_threshold: Decimal
    cost_per_unit: Decimal
    is_active: Boolean
}

Recipe {
    id: UUID
    menu_item_id: UUID
    variant_id: Option<UUID>
    ingredients: Vec<RecipeIngredient>
    yield_quantity: Decimal
    preparation_notes: Option<String>
}

RecipeIngredient {
    raw_material_id: UUID
    quantity: Decimal
    unit: Enum
    is_optional: Boolean
}

StockMovement {
    id: UUID
    raw_material_id: UUID
    outlet_id: UUID
    movement_type: Enum (PurchaseIn, Consumption, WastageOut, Adjustment, Transfer)
    quantity: Decimal
    unit_cost: Option<Decimal>
    reference_type: Option<String> ("bill", "purchase_order", "manual")
    reference_id: Option<UUID>
    notes: Option<String>
    performed_by: UUID
    performed_at: DateTime
}

Vendor {
    id: UUID
    name: String
    phone: String
    email: Option<String>
    address: Option<String>
    gstin: Option<String>
    materials_supplied: Vec<UUID>
    payment_terms: Option<String>
    is_active: Boolean
}

PurchaseOrder {
    id: UUID
    vendor_id: UUID
    outlet_id: UUID
    status: Enum (Draft, Sent, PartialReceived, Received, Cancelled)
    items: Vec<PurchaseOrderItem>
    total_amount: Decimal
    notes: Option<String>
    created_at: DateTime
    expected_delivery: Option<DateTime>
}
```

### 6.4 Module: Menu Management

**Priority:** P0

#### Features

| Feature | Description | MVP? |
|---|---|---|
| Categories & Sub-categories | Hierarchical menu organization | ✅ |
| Menu Items | Name, description, price, image | ✅ |
| Variants | Size (S/M/L), type (Veg/Non-veg) with different prices | ✅ |
| Modifiers/Add-ons | Extra cheese, toppings, etc. | ✅ |
| Modifier Groups | Group modifiers (e.g., "Choose Crust" group) | ✅ |
| Pricing | Base price, variant pricing, outlet-specific pricing | ✅ |
| Tax Configuration | Item-level tax rates (GST 5%/12%/18%) | ✅ |
| Availability Toggle | Mark items as available/unavailable in real-time | ✅ |
| Veg/Non-veg/Egg Labels | Indian-specific dietary markers | ✅ |
| Spice Level | Configurable spice levels | ✅ |
| Item Images | Upload and display item images | ✅ |
| Preparation Time | Estimated prep time per item | ✅ |
| Combo Meals | Bundle items at a special price | Phase 2 |
| Seasonal Menu | Time-limited menu items | Phase 2 |
| Multi-language Menu | Menu in regional languages | Phase 2 |
| QR Menu | Customer-facing digital menu | Phase 2 |

#### Data Model (Core)

```
MenuCategory {
    id: UUID
    outlet_id: UUID
    name: String
    description: Option<String>
    parent_id: Option<UUID> (for sub-categories)
    sort_order: i32
    image_url: Option<String>
    is_active: Boolean
}

MenuItem {
    id: UUID
    outlet_id: UUID
    category_id: UUID
    name: String
    description: Option<String>
    base_price: Decimal
    tax_rate: Decimal (5.0, 12.0, 18.0)
    food_type: Enum (Veg, NonVeg, Egg, Vegan)
    spice_level: Option<Enum (Mild, Medium, Hot, ExtraHot)>
    preparation_time_minutes: Option<i32>
    image_url: Option<String>
    sort_order: i32
    is_available: Boolean
    is_active: Boolean
    tags: Vec<String>
}

MenuVariant {
    id: UUID
    menu_item_id: UUID
    name: String (e.g., "Half Plate", "Full Plate", "Regular", "Large")
    price: Decimal
    is_default: Boolean
    is_available: Boolean
    sort_order: i32
}

ModifierGroup {
    id: UUID
    outlet_id: UUID
    name: String (e.g., "Choose Crust", "Add Toppings")
    selection_type: Enum (Single, Multiple)
    min_selection: i32
    max_selection: i32
    is_required: Boolean
}

Modifier {
    id: UUID
    group_id: UUID
    name: String (e.g., "Extra Cheese", "Thin Crust")
    price: Decimal
    is_default: Boolean
    is_available: Boolean
}

MenuItemModifierGroup {
    menu_item_id: UUID
    modifier_group_id: UUID
}
```

### 6.5 Module: Order Management

**Priority:** P0

#### Features

| Feature | Description | MVP? |
|---|---|---|
| Dine-in Orders | Link to table, support multiple KOTs | ✅ |
| Takeaway Orders | Counter orders without table | ✅ |
| Delivery Orders | Address, delivery partner, tracking | ✅ |
| KOT Management | Kitchen order tickets with status tracking | ✅ |
| Order Status Tracking | Placed → Preparing → Ready → Served → Billed | ✅ |
| Order Modification | Add/remove items, change quantity (before billing) | ✅ |
| Order Notes | Special instructions per item or order | ✅ |
| Order History | Searchable order history | ✅ |
| Running Orders Dashboard | Live view of all active orders | ✅ |
| **Waiter → Kitchen (Multi-Device)** | Waiter places order on phone → Kitchen display shows KOT instantly (via local WiFi or cloud) | ✅ |
| **Kitchen Status → Waiter** | Kitchen marks "Ready" → Waiter's phone shows notification | ✅ |
| **Works without internet** | All multi-device communication works over local WiFi when internet is down | ✅ |
| Kitchen Display System (KDS) | Digital KOT display for kitchen (tablet on wall) | ✅ (moved to MVP) |
| Aggregator Integration | Zomato, Swiggy order sync | Phase 2 |
| Online Ordering | Customer-facing ordering system | Phase 2 |

#### Order Lifecycle (Multi-Device, Network-Agnostic)

```
   WAITER'S PHONE (Table 5)                    CASHIER PC                      KITCHEN TABLET
   ──────────────────────                    ──────────                      ──────────────
   Taps: "1x Butter Chicken,                      │                              │
          2x Naan, Table 5"                        │                              │
           │                                       │                              │
           │  ┌─────────────────────────────┐      │                              │
           └─▶│ Smart API Client decides:   │      │                              │
              │ Cloud available? → Cloud API │      │                              │
              │ Only WiFi? → Cashier PC IP   │      │                              │
              └──────────────┬──────────────┘      │                              │
                             │                      │                              │
                             ▼                      ▼                              │
                    ┌──────────────────┐    Saves to SQLite                       │
                    │   Order Created   │    Updates dashboard                     │
                    │   KOT Generated   │──────────────────────────────────────────▶
                    └────────┬─────────┘                              KOT appears:
                             │                                    "TABLE 5:
                             │                                     1x Butter Chicken
           ┌─────────────────┘                                     2x Naan"
           │                                                            │
           ▼                                                            │
    Waiter sees:                                                  Kitchen taps:
    "Order Confirmed ✅"                                          "Butter Chicken READY"
           │                                                            │
           │◄───────────────────────────────────────────────────────────┘
           │                                            (WebSocket push — instant)
    Waiter sees:
    "Table 5: Butter Chicken READY 🟢"
           │
           ▼
    Serves food → Marks "Served"
           │
           ▼                                      ▼
                                           Cashier generates bill
                                           Bill prints on thermal printer
```

**This flow works identically whether:**
- ☁️ All devices are on cloud (internet available)
- 📡 All devices are on local WiFi only (no internet)
- The network switches mid-order (seamless handoff)

#### Order State Machine

```
           ┌──────────┐
           │  CREATED  │ ◄── Waiter places order (phone/tablet)
           └─────┬────┘
                 │
           ┌─────▼────┐
           │ CONFIRMED │ ◄── Auto-confirmed / Staff confirms
           └─────┬────┘
                 │
       ┌─────────┼──────────┐
       │         │          │
  ┌────▼───┐ ┌──▼───┐ ┌───▼────┐
  │ KOT #1 │ │KOT #2│ │ KOT #3 │   ◄── Multiple KOTs per order
  │Sent to │ │Sent  │ │ Sent   │       (via WebSocket to Kitchen)
  │Kitchen │ │      │ │        │
  └────┬───┘ └──┬───┘ └───┬────┘
       │        │         │
  ┌────▼───┐ ┌──▼───┐ ┌──▼─────┐
  │PREPARING│ │PREP  │ │PREPARING│  ◄── Kitchen updates status
  └────┬───┘ └──┬───┘ └───┬────┘
       │        │         │
  ┌────▼───┐ ┌──▼───┐ ┌──▼─────┐
  │ READY  │ │READY │ │ READY  │   ◄── WebSocket push → Waiter notified
  └────┬───┘ └──┬───┘ └───┬────┘
       │        │         │
       └────────┼─────────┘
                │
          ┌─────▼────┐
          │  SERVED   │ ◄── Waiter marks served
          └─────┬────┘
                │
          ┌─────▼────┐
          │  BILLED   │ ◄── Cashier generates bill
          └─────┬────┘
                │
          ┌─────▼────┐
          │ COMPLETED │ ◄── Payment received
          └──────────┘
```

### 6.6 Module: Reporting & Analytics

**Priority:** P0

#### Features

| Report | Description | MVP? |
|---|---|---|
| Daily Sales Report | Total sales, bill count, average bill value | ✅ |
| Item-wise Sales Report | Best sellers, slow movers, quantity sold | ✅ |
| Category-wise Sales | Revenue per category | ✅ |
| Payment Mode Report | Cash vs Card vs UPI breakdown | ✅ |
| GST Report | GSTR-1, GSTR-3B compatible tax summaries | ✅ |
| Hourly Sales | Sales distribution by hour | ✅ |
| Discount Report | Discounts given, by type, by staff | ✅ |
| Inventory Report | Current stock, stock value | ✅ |
| Purchase Report | Purchases by vendor, item, period | ✅ |
| Staff Performance | Bills processed, speed, sales by staff | ✅ |
| Profit & Loss | Revenue - COGS - Expenses | Phase 2 |
| Customer Report | Visit frequency, spending patterns | Phase 2 |
| Trend Analysis | Week-over-week, month-over-month trends | Phase 2 |
| Custom Report Builder | User-defined reports | Phase 2 |
| Scheduled Reports | Auto-email daily/weekly reports | Phase 2 |

#### Report Generation Architecture

```
                     ┌─────────────────────┐
                     │   Report Request     │
                     │   (User or Schedule) │
                     └──────────┬──────────┘
                                │
                     ┌──────────▼──────────┐
                     │   Report Engine      │
                     │                      │
                     │  ┌────────────────┐  │
                     │  │ Query Builder  │  │
                     │  │ (Dynamic SQL)  │  │
                     │  └───────┬────────┘  │
                     │          │           │
                     │  ┌───────▼────────┐  │
                     │  │  Data Source    │  │
                     │  │  Router        │  │
                     │  └───┬───────┬────┘  │
                     │      │       │       │
                     │  ┌───▼──┐ ┌──▼────┐  │
                     │  │SQLite│ │Postgre│  │
                     │  │Local │ │Cloud  │  │
                     │  └───┬──┘ └──┬────┘  │
                     │      │       │       │
                     │  ┌───▼───────▼────┐  │
                     │  │  Aggregation   │  │
                     │  │  Engine        │  │
                     │  └───────┬────────┘  │
                     │          │           │
                     │  ┌───────▼────────┐  │
                     │  │  Formatter     │  │
                     │  │  (Table/Chart/ │  │
                     │  │   PDF/Excel)   │  │
                     │  └───────┬────────┘  │
                     │          │           │
                     └──────────┼──────────┘
                                │
                     ┌──────────▼──────────┐
                     │  Output              │
                     │  • Dashboard Widget  │
                     │  • PDF Download      │
                     │  • Excel Export      │
                     │  • Email Delivery    │
                     │  • Print             │
                     └─────────────────────┘
```

### 6.7 Module: Table Management

**Priority:** P0

#### Features

| Feature | Description | MVP? |
|---|---|---|
| Floor Plan Editor | Visual drag-and-drop table layout | ✅ |
| Multiple Floors/Sections | AC Section, Outdoor, VIP, etc. | ✅ |
| Table Status | Available, Occupied, Reserved, Cleaning | ✅ |
| Table Shapes | Round, Square, Rectangle, Custom | ✅ |
| Capacity | Set seating capacity per table | ✅ |
| Merge Tables | Combine tables for large parties | ✅ |
| Split Table | Move guests to separate tables | ✅ |
| Transfer Table | Move order from one table to another | ✅ |
| Reservation System | Basic date/time/name reservations | ✅ |
| Table Timer | Track time since table was occupied | ✅ |
| Waiter Assignment | Assign tables to specific waiters | ✅ |
| Table-wise Revenue | Track revenue per table per day | Phase 2 |
| Waitlist Management | Queue management during peak hours | Phase 2 |

#### Table State Machine

```
                    ┌────────────┐
              ┌─────│  AVAILABLE  │◄──────────────────────┐
              │     └──────┬─────┘                        │
              │            │                              │
              │     Seat Guest / Reserve                  │
              │            │                              │
              │     ┌──────▼──────┐                       │
              │     │  OCCUPIED    │                       │
              │     └──────┬──────┘                       │
              │            │                              │
              │     Order Complete                   Mark Clean
              │     Bill Paid                             │
              │            │                              │
              │     ┌──────▼──────┐                ┌──────┴──────┐
              │     │  BILLING    │──── Paid ─────▶│  CLEANING   │
              │     └─────────────┘                └─────────────┘
              │
         Reservation
         Created
              │
        ┌─────▼──────┐
        │  RESERVED   │── Guest Arrives ──▶ OCCUPIED
        └─────┬──────┘
              │
         No Show / Cancel
              │
              ▼
          AVAILABLE
```

### 6.8 Cross-Cutting Module: Authentication & Authorization

#### Role-Based Access Control (RBAC)

```
Roles (Default):
├── Super Admin (Restaurant Owner)
│   └── Full access to everything
├── Admin (Manager)
│   └── All operations except critical settings
├── Cashier
│   └── Billing, orders, payments
├── Kitchen Staff
│   └── View orders, update KOT status
├── Waiter
│   └── Place orders, view tables
├── Inventory Manager
│   └── Stock management, purchases
└── Viewer (Accountant)
    └── View reports only

Permissions (Granular):
├── billing:create
├── billing:read
├── billing:update
├── billing:delete
├── billing:discount:apply
├── billing:void
├── inventory:stock:adjust
├── menu:item:create
├── menu:price:update
├── report:sales:view
├── report:export
├── table:manage
├── settings:modify
└── ... (50+ granular permissions)
```

---

## 7. Database Design Philosophy

### 7.1 Multi-Tenancy Strategy

**Approach: Schema-per-Tenant (Hybrid)**

```
PostgreSQL Instance
├── public schema (shared)
│   ├── tenants table
│   ├── plans table
│   └── global_config table
│
├── tenant_abc123 schema
│   ├── outlets
│   ├── users
│   ├── menu_categories
│   ├── menu_items
│   ├── bills
│   ├── orders
│   ├── inventory
│   └── ... (all tenant tables)
│
├── tenant_def456 schema
│   ├── outlets
│   ├── users
│   └── ... (isolated data)
│
└── analytics schema (shared, aggregated)
    ├── usage_metrics
    └── platform_analytics
```

**Why Schema-per-Tenant?**
- Data isolation without separate database overhead
- Easy backup/restore per tenant
- Can migrate high-volume tenants to dedicated databases later
- Simpler compliance (data locality, deletion)

### 7.2 Key Design Principles

1. **UUID Primary Keys** — No auto-increment IDs. UUIDs prevent enumeration attacks and enable conflict-free sync between local and cloud.

2. **Soft Deletes** — Never hard-delete business data. Use `deleted_at` timestamp. Critical for audit trails.

3. **Audit Trail** — Every mutation is logged with `created_by`, `updated_by`, `created_at`, `updated_at`.

4. **Decimal for Money** — Never use floating-point for financial calculations. Use `DECIMAL(12,2)` in PostgreSQL and `rust_decimal` in Rust.

5. **JSON Metadata** — Each core entity has a `metadata: JSONB` column for extensibility without schema changes.

6. **Timestamps in UTC** — All timestamps stored in UTC. Convert to local timezone only in the presentation layer.

7. **Enum Consistency** — Use PostgreSQL enums for fixed types. Store as strings for flexibility.

### 7.3 Core Schema Overview

```sql
-- Tenant (Restaurant Business)
CREATE TABLE tenants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(100) UNIQUE NOT NULL,
    plan_id UUID REFERENCES plans(id),
    status VARCHAR(20) DEFAULT 'active',
    settings JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Outlet (Individual Restaurant Location)
CREATE TABLE outlets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id),
    name VARCHAR(255) NOT NULL,
    address TEXT,
    city VARCHAR(100),
    state VARCHAR(100),
    pincode VARCHAR(10),
    phone VARCHAR(15),
    gstin VARCHAR(15),
    fssai_number VARCHAR(20),
    timezone VARCHAR(50) DEFAULT 'Asia/Kolkata',
    currency VARCHAR(3) DEFAULT 'INR',
    settings JSONB DEFAULT '{}',
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indexes
CREATE INDEX idx_outlets_tenant ON outlets(tenant_id);
CREATE INDEX idx_bills_outlet_date ON bills(outlet_id, created_at);
CREATE INDEX idx_orders_status ON orders(outlet_id, status);
CREATE INDEX idx_menu_items_category ON menu_items(category_id, is_active);
CREATE INDEX idx_stock_movements_material ON stock_movements(raw_material_id, performed_at);
```

### 7.4 SQLite Schema (Local — Desktop)

The local SQLite database mirrors the cloud schema with some differences:

- No multi-tenancy (single restaurant per installation)
- Added `sync_status` column to every table: `Enum(Synced, Pending, Conflict)`
- Added `cloud_id` column for mapping to cloud UUID
- Added `last_synced_at` column
- Optimized indexes for single-user queries

```sql
-- Local SQLite version of bills
CREATE TABLE bills (
    id TEXT PRIMARY KEY,           -- UUID as TEXT in SQLite
    cloud_id TEXT,                 -- Maps to cloud UUID (NULL if never synced)
    sync_status TEXT DEFAULT 'pending', -- 'synced', 'pending', 'conflict'
    last_synced_at TEXT,           -- ISO 8601 timestamp
    
    bill_number TEXT NOT NULL,
    outlet_id TEXT NOT NULL,
    table_id TEXT,
    order_type TEXT NOT NULL,
    
    subtotal REAL NOT NULL,
    discount_amount REAL DEFAULT 0,
    tax_amount REAL NOT NULL,
    service_charge REAL DEFAULT 0,
    round_off REAL DEFAULT 0,
    total_amount REAL NOT NULL,
    
    payment_status TEXT DEFAULT 'pending',
    
    created_by TEXT NOT NULL,
    created_at TEXT NOT NULL,
    finalized_at TEXT,
    updated_at TEXT NOT NULL,
    deleted_at TEXT,
    
    metadata TEXT DEFAULT '{}'
);

CREATE INDEX idx_bills_date ON bills(created_at);
CREATE INDEX idx_bills_sync ON bills(sync_status);
```

---

## 8. Fully Dynamic Networking — Auto-Switching Architecture

> **Core Design Principle:** The user NEVER selects a mode. The app detects the environment and switches automatically. Internet available? Use cloud. Only WiFi? Use local server. Nothing? Work standalone. All transitions are invisible to the user.

### 8.1 The Smart Connection Manager

The **Smart Connection Manager** is a Rust module embedded in every desktop app instance. It continuously monitors connectivity and routes all API calls to the right destination.

```
┌─────────────────────────────────────────────────────────────────────┐
│                   SMART CONNECTION MANAGER                            │
│              (Rust module — runs in background thread)                │
│                                                                      │
│  ┌───────────────────────────────────────────────────────────────┐  │
│  │                   CONNECTIVITY MONITOR                         │  │
│  │                   (runs every 5 seconds)                       │  │
│  │                                                                │  │
│  │  Step 1: Ping cloud server (api.restrosync.com)               │  │
│  │          Timeout: 2 seconds                                    │  │
│  │          ├── Success → cloud_available = true                  │  │
│  │          └── Fail    → cloud_available = false                 │  │
│  │                                                                │  │
│  │  Step 2: Check local network interfaces                       │  │
│  │          ├── WiFi connected? Get local IP (192.168.x.x)       │  │
│  │          └── Broadcast mDNS service for device discovery       │  │
│  │                                                                │  │
│  │  Step 3: Determine active mode                                │  │
│  │          ├── cloud_available = true      → ☁️ CLOUD MODE       │  │
│  │          ├── wifi_connected = true       → 📡 LOCAL MODE       │  │
│  │          └── nothing                     → 💾 STANDALONE       │  │
│  │                                                                │  │
│  └───────────────────────────────────────────────────────────────┘  │
│                                                                      │
│  ┌───────────────────────────────────────────────────────────────┐  │
│  │                   API ROUTER                                   │  │
│  │                                                                │  │
│  │  Every API call goes through this router:                      │  │
│  │                                                                │  │
│  │  fn route_request(request) {                                   │  │
│  │      // ALWAYS write to local SQLite first (instant)           │  │
│  │      local_db.write(request);                                  │  │
│  │                                                                │  │
│  │      match current_mode {                                      │  │
│  │          CloudMode => {                                        │  │
│  │              // Forward to cloud API                           │  │
│  │              cloud_api.send(request);                           │  │
│  │              // Also serve local devices from cloud             │  │
│  │          }                                                     │  │
│  │          LocalMode => {                                        │  │
│  │              // Process locally on embedded server              │  │
│  │              // Serve waiter/kitchen from local server          │  │
│  │              // Queue for cloud sync later                     │  │
│  │              sync_queue.push(request);                          │  │
│  │          }                                                     │  │
│  │          StandaloneMode => {                                   │  │
│  │              // Only local SQLite                              │  │
│  │              // Queue everything for later sync                │  │
│  │              sync_queue.push(request);                          │  │
│  │          }                                                     │  │
│  │      }                                                         │  │
│  │  }                                                             │  │
│  │                                                                │  │
│  └───────────────────────────────────────────────────────────────┘  │
│                                                                      │
│  ┌───────────────────────────────────────────────────────────────┐  │
│  │                   MODE TRANSITION HANDLER                      │  │
│  │                                                                │  │
│  │  On mode change (e.g., Cloud → Local):                        │  │
│  │  1. Current in-flight requests: complete on old mode           │  │
│  │  2. New requests: route to new mode                            │  │
│  │  3. WebSocket connections: reconnect to new target             │  │
│  │     (cloud WS ↔ local WS — same event protocol)               │  │
│  │  4. Notify UI: update status icon (☁️→📡→💾)                   │  │
│  │  5. If going Cloud→Local: start queueing for later sync       │  │
│  │  6. If going Local→Cloud: flush sync queue in background      │  │
│  │                                                                │  │
│  │  ⚡ Transition time: < 2 seconds                               │  │
│  │  ⚡ Zero data loss during transition                           │  │
│  │  ⚡ User sees only a tiny icon change in the corner            │  │
│  │                                                                │  │
│  └───────────────────────────────────────────────────────────────┘  │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### 8.2 The Three Modes — Detailed Behavior

#### ☁️ MODE 1: CLOUD (Internet Available)

```
┌─────────────────────────────────────────────────────────────────────┐
│  ☁️ CLOUD MODE — Best experience, full features                     │
│                                                                      │
│  When: Internet is working (detected by ping to cloud server)       │
│                                                                      │
│  How it works:                                                       │
│  ┌─────────┐   ┌──────────┐   ┌──────────┐   ┌──────────────────┐ │
│  │ Waiter  │   │ Kitchen  │   │ Cashier  │   │    Owner         │ │
│  │ (Phone) │   │ (Tablet) │   │ (PC)     │   │ (Web at home)    │ │
│  └────┬────┘   └────┬─────┘   └────┬─────┘   └────────┬─────────┘ │
│       │             │              │                    │           │
│       └─────────────┴──────────────┴────────────────────┘           │
│                            │                                         │
│                            ▼                                         │
│                   ┌────────────────┐                                 │
│                   │  CLOUD SERVER   │                                 │
│                   │  (Our API)      │                                 │
│                   └────────┬───────┘                                 │
│                            │                                         │
│                   ┌────────▼───────┐                                 │
│                   │  PostgreSQL +   │                                 │
│                   │  Redis (Cloud)  │                                 │
│                   └────────────────┘                                 │
│                                                                      │
│  ✅ Owner sees live dashboard from anywhere                         │
│  ✅ Multi-outlet data centralized                                   │
│  ✅ Automatic cloud backups                                         │
│  ✅ Local SQLite also keeps a copy (for speed + fallback)           │
│                                                                      │
│  Status Bar: ☁️ Connected                                            │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

#### 📡 MODE 2: LOCAL NETWORK (WiFi Only, No Internet)

```
┌─────────────────────────────────────────────────────────────────────┐
│  📡 LOCAL NETWORK MODE — Full multi-device, no internet needed      │
│                                                                      │
│  When: No internet, but WiFi router exists in restaurant            │
│                                                                      │
│  How it works:                                                       │
│  ┌─────────┐    ┌──────────┐                                        │
│  │ Waiter  │    │ Kitchen  │                                        │
│  │ (Phone) │    │ (Tablet) │                                        │
│  └────┬────┘    └────┬─────┘                                        │
│       │              │                                               │
│       │   WiFi       │   WiFi                                        │
│       │  (no internet needed — just the router)                      │
│       │              │                                               │
│       ▼              ▼                                               │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │              CASHIER PC (Embedded Local Server)               │    │
│  │                                                               │    │
│  │  Runs Axum HTTP server on: http://192.168.1.100:8080         │    │
│  │  Runs WebSocket server on: ws://192.168.1.100:8081           │    │
│  │                                                               │    │
│  │  ┌─────────────────────────────────────────────────────┐     │    │
│  │  │  Same REST API endpoints as Cloud Server             │     │    │
│  │  │  POST /v1/orders       ← Waiter places order         │     │    │
│  │  │  GET  /v1/orders/active ← Kitchen gets active orders  │     │    │
│  │  │  PATCH /v1/orders/:id/status ← Kitchen updates KOT   │     │    │
│  │  │  GET  /v1/tables       ← Waiter sees table status     │     │    │
│  │  │  POST /v1/bills        ← Cashier creates bill         │     │    │
│  │  └─────────────────────────────────────────────────────┘     │    │
│  │                                                               │    │
│  │  ┌────────────┐   ┌────────────────────────────────────┐     │    │
│  │  │   SQLite    │   │  Sync Queue                        │     │    │
│  │  │ (all data)  │   │  (stores changes for cloud later)  │     │    │
│  │  └────────────┘   └────────────────────────────────────┘     │    │
│  └───────────────────────────────────────────────────────────────┘    │
│                                                                      │
│  ✅ Waiters take orders on their phones                             │
│  ✅ Kitchen sees KOTs instantly (WebSocket)                         │
│  ✅ Cashier generates bills, prints receipts                        │
│  ✅ EVERYTHING works — zero dependency on internet                  │
│  ⏳ All changes queued → auto-sync when internet returns            │
│                                                                      │
│  Status Bar: 📡 Local Network                                        │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

#### 💾 MODE 3: STANDALONE (Single Device, No Network)

```
┌─────────────────────────────────────────────────────────────────────┐
│  💾 STANDALONE MODE — Single device, no connectivity                 │
│                                                                      │
│  When: No internet AND no WiFi (or single-person operation)         │
│                                                                      │
│  ┌───────────────────────────────────────────────────────────────┐  │
│  │                    CASHIER PC ONLY                              │  │
│  │                                                                │  │
│  │  All operations on one machine:                                │  │
│  │  • Take orders                                                 │  │
│  │  • Generate bills                                              │  │
│  │  • Manage inventory                                            │  │
│  │  • View reports                                                │  │
│  │  • Everything saved in local SQLite                            │  │
│  │                                                                │  │
│  │  No waiter devices, no kitchen display                         │  │
│  │  (because there's no network to connect them)                  │  │
│  │                                                                │  │
│  │  ⏳ All changes queued → sync when any network is available    │  │
│  │                                                                │  │
│  └───────────────────────────────────────────────────────────────┘  │
│                                                                      │
│  Best for: Single-person chai stall, tiny restaurant, backup mode   │
│                                                                      │
│  Status Bar: 💾 Offline                                              │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### 8.3 How Waiter Phones & Kitchen Tablets Discover the Local Server

The waiter and kitchen devices need to find the Cashier PC on the network **without the user manually entering an IP address**:

```
┌─────────────────────────────────────────────────────────────────────┐
│              DEVICE DISCOVERY (Zero Configuration)                    │
│                                                                      │
│  METHOD 1: mDNS Auto-Discovery (Primary)                            │
│  ──────────────────────────────────────                              │
│  Cashier PC broadcasts: _restrosync._tcp.local                       │
│  Waiter phone browser finds it automatically                         │
│  Result: Waiter opens app → sees "Restaurant Server Found" → Done   │
│                                                                      │
│  METHOD 2: QR Code (Fallback — simplest for non-tech users)         │
│  ──────────────────────────────────────────────────────              │
│  Cashier PC displays QR code on screen                               │
│  QR contains: http://192.168.1.100:8080                              │
│  Waiter scans QR with phone camera → opens in browser → Done        │
│                                                                      │
│  METHOD 3: Manual URL (Last resort)                                  │
│  ──────────────────────────────────                                  │
│  Cashier PC shows: "Waiter URL: http://192.168.1.100:8080"          │
│  Waiter types URL in phone browser → Done                            │
│                                                                      │
│  FIRST-TIME SETUP: ~30 seconds                                       │
│  AFTER FIRST TIME: Phone remembers the URL, auto-connects           │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### 8.4 The Golden Rule: Local-First Write

**Every single write operation — regardless of mode — goes to local SQLite FIRST.**

```
┌─────────────────────────────────────────────────────────────────────┐
│                     THE GOLDEN RULE                                   │
│                                                                      │
│     ANY write operation (order, bill, menu change, stock update)    │
│                          │                                           │
│                          ▼                                           │
│                ┌─────────────────┐                                   │
│                │ STEP 1: ALWAYS  │                                   │
│                │ write to LOCAL  │ ◄── This NEVER fails              │
│                │ SQLite FIRST    │     This is INSTANT (<5ms)        │
│                └────────┬────────┘                                   │
│                         │                                            │
│                         ▼                                            │
│                ┌─────────────────┐                                   │
│                │ STEP 2: Check   │                                   │
│                │ current mode    │                                   │
│                └────────┬────────┘                                   │
│                         │                                            │
│           ┌─────────────┼──────────────┐                             │
│           │             │              │                              │
│           ▼             ▼              ▼                              │
│     ☁️ CLOUD       📡 LOCAL       💾 STANDALONE                      │
│     Send to        Process         Done.                             │
│     cloud API      locally         Queued for                        │
│     (async,        Broadcast to    later sync.                       │
│     non-blocking)  connected                                         │
│                    devices via                                        │
│                    WebSocket                                         │
│     Also:          Queue for       Also:                             │
│     Broadcast to   cloud sync     Broadcast                          │
│     devices via    later           nothing                            │
│     cloud WS                       (no devices                        │
│                                    connected)                         │
│                                                                      │
│  RESULT: User action ALWAYS feels instant                            │
│  RESULT: Data is ALWAYS safe (local copy exists)                     │
│  RESULT: Cloud gets data eventually (when available)                 │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### 8.5 Sync Engine Architecture

When the app transitions from offline/local to online, the sync engine kicks in:

```
┌─────────────────────────────────────────────────────────────────────┐
│                        SYNC ENGINE                                    │
│         (activates when Cloud becomes available)                     │
│                                                                      │
│  ┌───────────────────────────────────────────────────────────────┐  │
│  │  CHANGE TRACKER (always running)                               │  │
│  │                                                                │  │
│  │  Every local SQLite write also inserts into:                   │  │
│  │                                                                │  │
│  │  sync_queue table:                                             │  │
│  │  ┌──────────┬───────────┬──────────┬──────────┬────────────┐  │  │
│  │  │ id       │ table     │ operation│ row_data │ created_at │  │  │
│  │  ├──────────┼───────────┼──────────┼──────────┼────────────┤  │  │
│  │  │ uuid-1   │ orders    │ INSERT   │ {json}   │ 2:15 PM    │  │  │
│  │  │ uuid-2   │ bills     │ INSERT   │ {json}   │ 2:16 PM    │  │  │
│  │  │ uuid-3   │ inventory │ UPDATE   │ {json}   │ 2:17 PM    │  │  │
│  │  │ ...      │ ...       │ ...      │ ...      │ ...        │  │  │
│  │  └──────────┴───────────┴──────────┴──────────┴────────────┘  │  │
│  └───────────────────────────────────────────────────────────────┘  │
│                                                                      │
│  ┌───────────────────────────────────────────────────────────────┐  │
│  │  SYNC EXECUTOR (runs when Cloud becomes reachable)             │  │
│  │                                                                │  │
│  │  1. Read all pending entries from sync_queue                   │  │
│  │  2. Batch them (max 100 per request for efficiency)            │  │
│  │  3. POST /v1/sync/push { changes: [...] }                     │  │
│  │  4. Server processes, returns:                                 │  │
│  │     ├── accepted: [uuid-1, uuid-2, uuid-3]                    │  │
│  │     ├── conflicts: [{ id: uuid-4, resolution: {...} }]        │  │
│  │     └── server_changes: [{ table: "menu", ... }]              │  │
│  │  5. Apply server_changes to local SQLite                       │  │
│  │  6. Resolve conflicts per strategy                             │  │
│  │  7. Delete synced entries from sync_queue                      │  │
│  │  8. Update last_sync_timestamp                                 │  │
│  └───────────────────────────────────────────────────────────────┘  │
│                                                                      │
│  ┌───────────────────────────────────────────────────────────────┐  │
│  │  CONFLICT RESOLUTION                                           │  │
│  │                                                                │  │
│  │  Data Type          │ Strategy           │ Rationale           │  │
│  │  ───────────────────┼────────────────────┼──────────────────── │  │
│  │  Bills (finalized)  │ Server wins        │ Financial = truth   │  │
│  │  Bills (draft)      │ Last-write-wins    │ Work in progress    │  │
│  │  Menu items         │ Last-write + merge │ Rarely conflicts    │  │
│  │  Inventory stock    │ Operational Trans. │ Both sides modify   │  │
│  │  Orders (active)    │ Server wins        │ Multi-device coord. │  │
│  │  Settings           │ Last-write-wins    │ Owner's last change │  │
│  │  Reports            │ Regenerate         │ Derived data        │  │
│  └───────────────────────────────────────────────────────────────┘  │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### 8.6 What the User Sees — Status Indicators

The user sees **one small icon** in the top-right corner. That's it.

| Icon | Mode | Meaning | User Action Needed |
|---|---|---|---|
| ☁️ 🟢 | Cloud | Connected to cloud. All features working. | None |
| 📡 🔵 | Local Network | No internet. Working on local WiFi. All devices connected. | None |
| 💾 🟡 | Standalone | No network. Single device mode. | None |
| ☁️ 🔄 | Syncing | Internet just returned. Syncing queued data to cloud. | None |
| ⚠️ 🔴 | Error | Something wrong (rare). | Click icon for details |

**The user never selects a mode. The user never configures anything. It just works.**

### 8.7 Hardware Requirement Summary

```
┌─────────────────────────────────────────────────────────────────────┐
│              WHAT A RESTAURANT NEEDS                                  │
│                                                                      │
│  REQUIRED:                                                           │
│  ├── 1x PC/Laptop (₹15,000+)  → Runs desktop app + local server   │
│  └── 1x Thermal Printer (₹3,000-5,000) → Bill printing             │
│                                                                      │
│  FOR MULTI-DEVICE (Waiter + Kitchen):                               │
│  ├── 1x WiFi Router (₹500-2,000) → They probably already have one  │
│  ├── Waiter phones → They already have personal phones              │
│  └── 1x Kitchen tablet (₹8,000-10,000) → Mounted in kitchen        │
│                                                                      │
│  FOR CLOUD ACCESS (Owner remote monitoring):                         │
│  └── Internet connection → Any ISP, even mobile hotspot works       │
│                                                                      │
│  TOTAL EXTRA COST: ₹0 - ₹10,000 (most things already exist)        │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 9. Multi-Device Communication — Waiter, Kitchen & Cashier Flow

This section details exactly how multiple devices communicate in a restaurant, regardless of which networking mode is active.

### 9.1 Device Roles in a Restaurant

```
┌─────────────────────────────────────────────────────────────────────┐
│                    RESTAURANT DEVICE MAP                              │
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │  CASHIER PC (Desktop App — Tauri)                             │   │
│  │  Role: Central hub + billing + management                     │   │
│  │                                                               │   │
│  │  • Runs the desktop application (full features)               │   │
│  │  • Runs embedded local server (serves other devices)          │   │
│  │  • Connected to thermal printer + cash drawer                 │   │
│  │  • Stores all data in local SQLite                            │   │
│  │  • Manages sync with cloud                                    │   │
│  │  • Always running during business hours                       │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │  WAITER DEVICE (Phone/Tablet — Browser/PWA)                   │   │
│  │  Role: Order taking + table monitoring                        │   │
│  │                                                               │   │
│  │  • Lightweight web app (React PWA)                            │   │
│  │  • Browse menu, take orders, assign to tables                 │   │
│  │  • See table status (available/occupied/reserved)             │   │
│  │  • Receive notifications when food is ready                   │   │
│  │  • View running orders for assigned tables                    │   │
│  │  • No data storage — all data lives on server (cloud/local)  │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │  KITCHEN DISPLAY (Tablet/Monitor — Browser/PWA)               │   │
│  │  Role: View & manage KOTs                                     │   │
│  │                                                               │   │
│  │  • Shows incoming Kitchen Order Tickets (KOTs)                │   │
│  │  • Organized by table / order time                            │   │
│  │  • Kitchen staff taps to mark items: Preparing → Ready        │   │
│  │  • Audio alert on new KOT                                     │   │
│  │  • Color-coded urgency (green → yellow → red by wait time)   │   │
│  │  • No data storage — pure display + status updates            │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │  OWNER DEVICE (Web App — Browser)                             │   │
│  │  Role: Remote monitoring + management                         │   │
│  │                                                               │   │
│  │  • View live sales dashboard                                  │   │
│  │  • Access reports from anywhere                               │   │
│  │  • Modify menu, prices, settings                              │   │
│  │  • Only works when cloud is available (needs internet)        │   │
│  │  • Next.js web application                                    │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### 9.2 Complete Waiter → Kitchen → Cashier Flow

```
┌─────────────────────────────────────────────────────────────────────┐
│     STEP-BY-STEP: Customer walks in → Food served → Bill paid       │
│                                                                      │
│  1. CUSTOMER SEATED AT TABLE 5                                       │
│     Waiter opens phone → Sees floor plan → Taps Table 5             │
│     Table 5 turns "Occupied" on ALL devices instantly               │
│                                                                      │
│  2. WAITER TAKES ORDER                                               │
│     Waiter browses menu on phone:                                    │
│     ├── 1x Butter Chicken (Full, Extra Spicy)  ₹380                │
│     ├── 2x Garlic Naan                          ₹60 each            │
│     └── 1x Mango Lassi                          ₹120               │
│     Adds note: "No onion in naan"                                   │
│     Taps "Send to Kitchen"                                           │
│                                                                      │
│  3. ORDER PROCESSED (< 100ms)                                        │
│     ┌─────────┐     ┌──────────────┐     ┌────────────────┐        │
│     │ Waiter  │────▶│ Server       │────▶│ Kitchen Display │        │
│     │ Phone   │     │(Cloud/Local) │     │ (Tablet)       │        │
│     └─────────┘     └──────┬───────┘     └────────────────┘        │
│                            │                                         │
│                            ▼                                         │
│                     ┌──────────────┐                                 │
│                     │ Cashier PC   │                                 │
│                     │ (Dashboard   │                                 │
│                     │  updated)    │                                 │
│                     └──────────────┘                                 │
│                                                                      │
│  4. KITCHEN SEES KOT                                                 │
│     ┌────────────────────────────────────────┐                      │
│     │  🔔 NEW ORDER — TABLE 5               │                      │
│     │  ─────────────────────────────         │                      │
│     │  1x Butter Chicken (Full, Extra Spicy) │                      │
│     │  2x Garlic Naan ⚠️ No onion            │                      │
│     │  1x Mango Lassi                        │                      │
│     │  ─────────────────────────────         │                      │
│     │  Time: 2:15 PM                         │                      │
│     │  Waiter: Rahul                         │                      │
│     │                                        │                      │
│     │  [ START PREPARING ]                   │                      │
│     └────────────────────────────────────────┘                      │
│                                                                      │
│  5. KITCHEN UPDATES STATUS                                           │
│     Chef taps "Butter Chicken: READY"                                │
│     → WebSocket pushes to Waiter's phone instantly                  │
│     → Waiter sees: "🟢 Butter Chicken READY — Table 5"             │
│     → Cashier dashboard updates                                     │
│                                                                      │
│  6. WAITER SERVES FOOD                                               │
│     Waiter picks up food, marks "Served" on phone                   │
│     → All devices update                                            │
│                                                                      │
│  7. CUSTOMER ASKS FOR BILL                                           │
│     Waiter taps "Request Bill" on phone for Table 5                 │
│     → Cashier PC shows notification: "Table 5 requesting bill"      │
│     → Cashier reviews order, applies discount if any                │
│     → Generates bill → Prints on thermal printer                    │
│     → Waiter delivers bill to table                                 │
│                                                                      │
│  8. PAYMENT & CLOSE                                                  │
│     Cashier records payment (Cash/UPI/Card)                         │
│     → Table 5 status changes to "Cleaning"                          │
│     → Inventory auto-deducted (butter, chicken, naan flour, etc.)   │
│     → Reports updated                                               │
│     → Table 5 turns "Available" after cleaning                      │
│                                                                      │
│  ⚡ This entire flow works IDENTICALLY in:                           │
│     ☁️ Cloud Mode (internet)                                         │
│     📡 Local Network Mode (WiFi only)                                │
│     Even if mode switches MID-FLOW                                   │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### 9.3 WebSocket Event Flow Between Devices

```
┌─────────────────────────────────────────────────────────────────────┐
│              REAL-TIME EVENTS VIA WEBSOCKET                          │
│                                                                      │
│  WebSocket connects to:                                              │
│  ☁️ Cloud mode:  wss://api.restrosync.com/ws                        │
│  📡 Local mode:  ws://192.168.x.x:8081/ws                           │
│  (Same protocol, same events — only the URL changes)                │
│                                                                      │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────────────┐  │
│  │   WAITER      │    │   KITCHEN     │    │   CASHIER             │  │
│  │   subscribes  │    │   subscribes  │    │   subscribes          │  │
│  │   to:         │    │   to:         │    │   to:                 │  │
│  │               │    │               │    │                       │  │
│  │ • kot:status  │    │ • kot:new     │    │ • order:created       │  │
│  │ • table:status│    │ • kot:updated │    │ • order:updated       │  │
│  │ • order:status│    │ • order:cancel│    │ • bill:requested      │  │
│  │               │    │               │    │ • table:status        │  │
│  │   publishes:  │    │   publishes:  │    │ • inventory:alert     │  │
│  │               │    │               │    │                       │  │
│  │ • order:place │    │ • kot:status  │    │   publishes:          │  │
│  │ • table:seat  │    │   (preparing, │    │                       │  │
│  │ • bill:request│    │    ready)     │    │ • bill:created        │  │
│  │ • order:modify│    │               │    │ • table:available     │  │
│  │               │    │               │    │ • menu:availability   │  │
│  └──────────────┘    └──────────────┘    └──────────────────────┘  │
│                                                                      │
│  Event delivery: < 50ms on local WiFi, < 200ms on cloud            │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### 9.4 Unified API Contract — Same API, Any Server

The key to making this seamless: **waiter and kitchen devices use the exact same API whether talking to the cloud or the local Cashier PC.**

```
┌─────────────────────────────────────────────────────────────────────┐
│              UNIFIED API — ONE CONTRACT, TWO SERVERS                  │
│                                                                      │
│  Waiter's phone calls:                                               │
│                                                                      │
│  POST /v1/orders                                                     │
│  { "table_id": "...", "items": [...] }                              │
│                                                                      │
│  The Smart API Client on the phone decides WHERE to send it:        │
│                                                                      │
│  ☁️ Cloud available?                                                 │
│     → POST https://api.restrosync.com/v1/orders                     │
│                                                                      │
│  📡 Only local WiFi?                                                 │
│     → POST http://192.168.1.100:8080/v1/orders                      │
│                                                                      │
│  Same request. Same response format. Same behavior.                  │
│  The waiter doesn't know and doesn't care.                           │
│                                                                      │
│  This works because:                                                 │
│  1. Cloud server (Axum) and embedded local server (Axum) share      │
│     the SAME Rust codebase (packages/core library)                   │
│  2. Same route handlers, same business logic                         │
│  3. Only difference: data goes to PostgreSQL (cloud) vs             │
│     SQLite (local) — abstracted behind a DataSource trait           │
│                                                                      │
│  // Rust pseudo-code                                                 │
│  trait DataSource {                                                   │
│      async fn create_order(&self, order: NewOrder) -> Order;         │
│      async fn get_active_orders(&self) -> Vec<Order>;                │
│      async fn update_kot_status(&self, id: Uuid, s: Status);        │
│  }                                                                   │
│                                                                      │
│  struct PostgresSource { pool: PgPool }    // Cloud                  │
│  struct SqliteSource  { conn: SqlitePool } // Local                  │
│                                                                      │
│  // Both implement DataSource — routes don't care which one          │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 10. Cloud Architecture & Infrastructure

### 9.1 AWS Architecture (Production)

```
┌──────────────────────────────────────────────────────────────────┐
│                        AWS CLOUD                                  │
│                                                                   │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │                    VPC (10.0.0.0/16)                        │  │
│  │                                                             │  │
│  │  ┌─────────────────────────────────────────────────────┐   │  │
│  │  │              Public Subnet                           │   │  │
│  │  │                                                      │   │  │
│  │  │  ┌──────────────┐    ┌──────────────────────────┐   │   │  │
│  │  │  │  CloudFront   │    │    Application Load       │   │   │  │
│  │  │  │  (CDN)        │    │    Balancer (ALB)         │   │   │  │
│  │  │  └──────────────┘    └─────────────┬────────────┘   │   │  │
│  │  │                                     │                │   │  │
│  │  └─────────────────────────────────────┼────────────────┘   │  │
│  │                                        │                     │  │
│  │  ┌─────────────────────────────────────┼────────────────┐   │  │
│  │  │              Private Subnet          │                │   │  │
│  │  │                                      │                │   │  │
│  │  │  ┌──────────────────────────────────▼─────────────┐  │   │  │
│  │  │  │              ECS Fargate Cluster                 │  │   │  │
│  │  │  │                                                  │  │   │  │
│  │  │  │  ┌──────────┐ ┌──────────┐ ┌──────────┐        │  │   │  │
│  │  │  │  │ API      │ │ API      │ │ API      │        │  │   │  │
│  │  │  │  │ Task 1   │ │ Task 2   │ │ Task 3   │        │  │   │  │
│  │  │  │  │ (Rust)   │ │ (Rust)   │ │ (Rust)   │        │  │   │  │
│  │  │  │  └──────────┘ └──────────┘ └──────────┘        │  │   │  │
│  │  │  │                                                  │  │   │  │
│  │  │  │  ┌──────────┐ ┌──────────┐                      │  │   │  │
│  │  │  │  │ Web App  │ │ Worker   │                      │  │   │  │
│  │  │  │  │(Next.js) │ │ (Async)  │                      │  │   │  │
│  │  │  │  └──────────┘ └──────────┘                      │  │   │  │
│  │  │  └─────────────────────────────────────────────────┘  │   │  │
│  │  │                                                        │   │  │
│  │  │  ┌──────────────┐  ┌──────────────┐  ┌────────────┐  │   │  │
│  │  │  │  RDS          │  │  ElastiCache  │  │    S3      │  │   │  │
│  │  │  │  PostgreSQL   │  │  Redis        │  │  Storage   │  │   │  │
│  │  │  │  (Multi-AZ)   │  │  (Cluster)    │  │            │  │   │  │
│  │  │  └──────────────┘  └──────────────┘  └────────────┘  │   │  │
│  │  │                                                        │   │  │
│  │  └────────────────────────────────────────────────────────┘   │  │
│  │                                                               │  │
│  └───────────────────────────────────────────────────────────────┘  │
│                                                                      │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │  Monitoring & Observability                                   │   │
│  │  CloudWatch │ X-Ray │ SNS Alerts                              │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                                                                      │
└──────────────────────────────────────────────────────────────────────┘
```

### 9.2 MVP Infrastructure (Cost-Optimized)

For the MVP, we don't need the full AWS architecture. Start lean:

```
MVP Infrastructure (Month 1-6):

┌─────────────────────────────────────┐
│        Hetzner Cloud (Germany)       │
│        or DigitalOcean               │
│                                      │
│  Server 1: Application Server        │
│  ├── 4 vCPU, 8GB RAM                │
│  ├── Rust API (Axum)                │
│  ├── Next.js Web App                │
│  ├── Redis                          │
│  ├── NATS                           │
│  └── Nginx (Reverse Proxy)          │
│                                      │
│  Server 2: Database Server           │
│  ├── 4 vCPU, 16GB RAM              │
│  ├── PostgreSQL 17                  │
│  └── Automated backups              │
│                                      │
│  Cost: ~₹5,000-8,000/month          │
│                                      │
└─────────────────────────────────────┘

Scale Trigger: When we hit 500+ active restaurants
→ Migrate to AWS/GCP with proper infrastructure
```

### 9.3 Self-Hosting Option

For privacy-conscious customers who want to run everything on their own hardware:

```
Docker Compose Self-Host Package:
├── docker-compose.yml
├── .env.example
├── nginx.conf
├── init-db.sql
└── README.md

Services:
├── restrosync-api (Rust binary in Alpine container)
├── restrosync-web (Next.js in Node Alpine container)
├── postgresql (Official PostgreSQL 17 image)
├── redis (Official Redis 7 image)
└── nginx (Reverse proxy + SSL termination)

One-command deployment:
$ curl -sSL https://install.restrosync.com | bash
```

---

## 10. Security Architecture

### 10.1 Authentication

```
┌────────────────────────────────────────────────────┐
│                AUTHENTICATION FLOW                   │
│                                                      │
│  Login Request (email + password)                   │
│       │                                              │
│       ▼                                              │
│  Rate Limiting (10 attempts / 15 min)               │
│       │                                              │
│       ▼                                              │
│  Verify Password (Argon2id hash)                    │
│       │                                              │
│       ▼                                              │
│  Generate Token Pair:                                │
│  ├── Access Token (JWT, 15 min expiry)              │
│  │   ├── Claims: user_id, tenant_id, outlet_id     │
│  │   ├── Claims: role, permissions[]                │
│  │   └── Signed with RS256 (asymmetric)            │
│  │                                                   │
│  └── Refresh Token (opaque, 7 day expiry)           │
│      ├── Stored in Redis with session metadata      │
│      └── Rotated on each use                         │
│                                                      │
│  Optional: 2FA via TOTP (Google Authenticator)      │
│                                                      │
└────────────────────────────────────────────────────┘
```

### 10.2 Security Measures

| Area | Measure |
|---|---|
| **Passwords** | Argon2id hashing, minimum 8 characters |
| **Transport** | TLS 1.3 everywhere, HSTS headers |
| **API Auth** | JWT with short expiry + refresh token rotation |
| **RBAC** | Role + permission checks on every endpoint |
| **Input Validation** | Server-side validation on all inputs, SQL parameterization |
| **CORS** | Strict origin allowlist |
| **Rate Limiting** | Per-IP and per-user rate limits |
| **Data Encryption** | AES-256 for sensitive data at rest |
| **Local DB** | Optional SQLCipher encryption for local SQLite |
| **Audit Logging** | Every data mutation logged with user, timestamp, IP |
| **Dependency Scanning** | Cargo audit + npm audit in CI pipeline |
| **Secrets Management** | Environment variables, never in code. AWS Secrets Manager for production. |
| **CSP Headers** | Content Security Policy to prevent XSS |
| **CSRF Protection** | Double-submit cookie pattern |

### 10.3 Data Privacy & Compliance

- **Data Residency:** Indian servers for Indian customers (Mumbai AWS region)
- **GDPR-ready:** Data export and deletion capabilities
- **Data Isolation:** Schema-per-tenant ensures no data leakage
- **Backup Encryption:** All backups encrypted with AES-256
- **Right to Delete:** Full data purge capability per tenant

---

## 11. API Design & Communication

### 11.1 REST API Design

**Base URL:** `https://api.restrosync.com/v1`

**Authentication:** `Authorization: Bearer <jwt_token>`

**Standard Response Format:**

```json
{
    "success": true,
    "data": { ... },
    "meta": {
        "page": 1,
        "per_page": 20,
        "total": 150,
        "total_pages": 8
    },
    "errors": null
}
```

**Error Response:**

```json
{
    "success": false,
    "data": null,
    "errors": [
        {
            "code": "VALIDATION_ERROR",
            "field": "email",
            "message": "Invalid email format"
        }
    ]
}
```

### 12.2 Key API Endpoints

**These endpoints are served by BOTH the cloud server AND the embedded local server on the Cashier PC. Same contract, same behavior.**

```
⚡ All endpoints below work on:
   ☁️ https://api.restrosync.com/v1/...  (Cloud)
   📡 http://192.168.x.x:8080/v1/...     (Local Cashier PC)

Authentication:
POST   /v1/auth/login                # Login (local auth works offline too)
POST   /v1/auth/refresh              # Refresh token
POST   /v1/auth/logout               # Logout
POST   /v1/auth/forgot-password      # Forgot password (cloud only)
POST   /v1/auth/reset-password       # Reset password (cloud only)

Billing:
GET    /v1/bills                     # List bills (paginated, filtered)
POST   /v1/bills                     # Create new bill
GET    /v1/bills/:id                 # Get bill details
PUT    /v1/bills/:id                 # Update bill
POST   /v1/bills/:id/finalize        # Finalize bill
POST   /v1/bills/:id/payments        # Add payment to bill
POST   /v1/bills/:id/void            # Void a bill
GET    /v1/bills/:id/print           # Get printable bill (PDF/thermal)

Menu:
GET    /v1/menu/categories            # List categories
POST   /v1/menu/categories            # Create category
GET    /v1/menu/items                 # List items (with search, filters)
POST   /v1/menu/items                 # Create item
PUT    /v1/menu/items/:id             # Update item
PATCH  /v1/menu/items/:id/availability   # Toggle availability
GET    /v1/menu/modifiers             # List modifier groups

Orders (used by Waiter phones, Kitchen tablets, Cashier):
GET    /v1/orders                     # List orders
POST   /v1/orders                     # Create order (Waiter places order)
GET    /v1/orders/:id                 # Get order details
PUT    /v1/orders/:id                 # Update order (Waiter modifies)
PATCH  /v1/orders/:id/status          # Update order status
POST   /v1/orders/:id/kot             # Generate KOT (→ pushed to Kitchen)
GET    /v1/orders/active               # Get all active orders (Kitchen view)
POST   /v1/orders/:id/request-bill    # Waiter requests bill for table

KOT (Kitchen-specific):
GET    /v1/kot/pending                 # Kitchen gets all pending KOTs
PATCH  /v1/kot/:id/status              # Kitchen updates KOT status
                                       # (preparing/ready → pushes to Waiter)

Inventory:
GET    /v1/inventory/materials         # List raw materials
POST   /v1/inventory/materials         # Add raw material
GET    /v1/inventory/stock             # Current stock levels
POST   /v1/inventory/stock/adjust      # Adjust stock
GET    /v1/inventory/recipes           # List recipes
POST   /v1/inventory/recipes           # Create recipe
GET    /v1/inventory/vendors           # List vendors
POST   /v1/inventory/purchases         # Create purchase order
GET    /v1/inventory/alerts            # Low stock alerts

Tables (used by Waiter to see floor plan):
GET    /v1/tables                      # List all tables with status
POST   /v1/tables                      # Create table
PUT    /v1/tables/:id                  # Update table
PATCH  /v1/tables/:id/status           # Update table status
POST   /v1/tables/merge                # Merge tables
POST   /v1/tables/transfer             # Transfer table
GET    /v1/tables/floor-plan           # Get floor plan layout

Reports:
GET    /v1/reports/sales/daily         # Daily sales report
GET    /v1/reports/sales/itemwise      # Item-wise sales
GET    /v1/reports/sales/categorywise  # Category-wise sales
GET    /v1/reports/gst                 # GST report
GET    /v1/reports/inventory           # Inventory report
GET    /v1/reports/payments            # Payment mode report
POST   /v1/reports/export              # Export report (PDF/Excel)

Sync (Cloud server only):
POST   /v1/sync/push                   # Push local changes to cloud
POST   /v1/sync/pull                   # Pull server changes to local
GET    /v1/sync/status                 # Sync status

Device Discovery (Local server only):
GET    /v1/device/info                 # Local server info (name, version)
GET    /v1/device/qr                   # QR code for waiter device setup

Connectivity (Both):
GET    /v1/health                      # Health check (used for mode detection)

Settings:
GET    /v1/settings/outlet             # Outlet settings
PUT    /v1/settings/outlet             # Update outlet settings
GET    /v1/settings/tax                # Tax configuration
PUT    /v1/settings/tax                # Update tax config
GET    /v1/settings/print              # Print settings
```

### 12.3 WebSocket Events (Same on Cloud & Local Server)

```
WebSocket connects to:
  ☁️ Cloud mode:   wss://api.restrosync.com/ws
  📡 Local mode:   ws://192.168.x.x:8081/ws
  (SAME protocol, SAME events — only URL changes)

Server → Client:
├── order:created      # New order placed (→ Kitchen, Cashier)
├── order:updated      # Order modified (→ Kitchen, Cashier)
├── order:status       # Order status changed (→ all devices)
├── kot:new            # New KOT for kitchen (→ Kitchen Display)
├── kot:status         # KOT status updated (→ Waiter, Cashier)
├── table:status       # Table status changed (→ all devices)
├── inventory:alert    # Low stock alert (→ Cashier)
├── bill:created       # New bill generated (→ Cashier)
├── bill:requested     # Waiter requested bill (→ Cashier)
├── sync:update        # Data sync update (→ Desktop)
└── mode:changed       # Connectivity mode changed (→ all devices)

Client → Server:
├── order:place        # Waiter places new order
├── order:modify       # Waiter modifies order
├── kot:acknowledge    # Kitchen acknowledges KOT
├── kot:update_status  # Kitchen marks item preparing/ready
├── table:seat         # Waiter seats customer
├── table:update       # Update table status
├── bill:request       # Waiter requests bill for table
└── heartbeat          # Keep connection alive
```

---

## 12. Development Phases & Timeline

### 12.1 Phase Overview

```
┌────────────────────────────────────────────────────────────────────┐
│                    DEVELOPMENT TIMELINE                              │
│                                                                     │
│  Phase 0     Phase 1        Phase 2         Phase 3      Phase 4   │
│  Setup       MVP Core       MVP Complete    Growth       Scale     │
│                                                                     │
│  ├──────┤├────────────┤├────────────────┤├───────────┤├──────────┤  │
│  2 weeks  8 weeks       6 weeks          8 weeks      Ongoing    │
│                                                                     │
│  Week:  1-2    3-10        11-16           17-24        25+        │
│                                                                     │
│  Total MVP: ~16 weeks (4 months)                                   │
│  Total to Growth: ~24 weeks (6 months)                             │
│                                                                     │
└────────────────────────────────────────────────────────────────────┘
```

### 12.2 Phase 0: Foundation Setup (Weeks 1-2)

**Goal:** Set up the project infrastructure and development environment.

| Task | Duration | Owner |
|---|---|---|
| Initialize monorepo (Turborepo) | 1 day | Lead Dev |
| Set up Rust workspace (cargo workspace) | 1 day | Backend Dev |
| Set up Tauri project with React | 1 day | Frontend Dev |
| Set up Next.js web project | 1 day | Frontend Dev |
| Configure shared UI library (Shadcn) | 2 days | Frontend Dev |
| Design and implement base database schema | 2 days | Backend Dev |
| Set up SeaORM with migrations | 1 day | Backend Dev |
| Configure SQLite for desktop | 1 day | Backend Dev |
| Set up Axum server with basic middleware | 1 day | Backend Dev |
| Set up Docker Compose for local dev | 1 day | DevOps |
| Configure CI/CD pipeline (GitHub Actions) | 1 day | DevOps |
| Set up linting, formatting, pre-commit hooks | 0.5 day | Lead Dev |
| Create design system & Figma components | 3 days | Designer |

**Deliverable:** Working development environment where all team members can run the desktop app, web app, and backend locally.

### 12.3 Phase 1: MVP Core (Weeks 3-10)

#### Sprint 1 (Weeks 3-4): Authentication + Menu Management

| Task | Duration | Owner |
|---|---|---|
| User registration & login API | 2 days | Backend |
| JWT token system + refresh | 1 day | Backend |
| RBAC system with Casbin | 2 days | Backend |
| Login/Register UI (desktop + web) | 2 days | Frontend |
| Menu categories CRUD (API) | 2 days | Backend |
| Menu items CRUD (API) | 2 days | Backend |
| Menu variants & modifiers (API) | 2 days | Backend |
| Menu management UI — category tree | 2 days | Frontend |
| Menu management UI — item editor | 3 days | Frontend |
| Menu management UI — variant/modifier editor | 2 days | Frontend |

#### Sprint 2 (Weeks 5-6): Table Management + Order System

| Task | Duration | Owner |
|---|---|---|
| Table CRUD API | 2 days | Backend |
| Floor plan data model + API | 2 days | Backend |
| Table management UI — floor plan editor | 4 days | Frontend |
| Table status management | 1 day | Backend |
| Order CRUD API | 3 days | Backend |
| KOT generation system | 2 days | Backend |
| Order placement UI (item selection + table link) | 4 days | Frontend |
| Running orders dashboard | 2 days | Frontend |
| KOT view (kitchen screen) | 2 days | Frontend |

#### Sprint 3 (Weeks 7-8): Billing System

| Task | Duration | Owner |
|---|---|---|
| Billing engine (GST calculation, discounts) | 3 days | Backend |
| Bill generation from order | 2 days | Backend |
| Payment recording API | 2 days | Backend |
| Bill printing engine (thermal + A4) | 3 days | Backend |
| Billing UI — review & finalize | 3 days | Frontend |
| Payment UI — multi-mode payment | 2 days | Frontend |
| Bill history & search | 2 days | Frontend |
| Print preview & print flow | 2 days | Frontend |

#### Sprint 4 (Weeks 9-10): Inventory + Core Reports

| Task | Duration | Owner |
|---|---|---|
| Raw material CRUD API | 2 days | Backend |
| Stock movement tracking | 2 days | Backend |
| Recipe management API | 2 days | Backend |
| Auto stock deduction on billing | 2 days | Backend |
| Inventory UI — materials, stock levels | 3 days | Frontend |
| Recipe editor UI | 2 days | Frontend |
| Vendor management API + UI | 2 days | Full-stack |
| Daily sales report API + UI | 2 days | Full-stack |
| Item-wise sales report | 1 day | Full-stack |
| GST report | 1 day | Full-stack |
| Payment mode report | 1 day | Full-stack |

### 12.4 Phase 2: MVP Complete (Weeks 11-16)

#### Sprint 5 (Weeks 11-12): Dynamic Networking, Embedded Server & Sync Engine

| Task | Duration | Owner |
|---|---|---|
| Smart Connection Manager (auto-detect Cloud/Local/Standalone) | 3 days | Backend |
| Embedded local HTTP server in Tauri (Axum lightweight instance) | 3 days | Backend |
| Embedded local WebSocket server for real-time events | 2 days | Backend |
| mDNS service broadcast for device auto-discovery | 1 day | Backend |
| Waiter PWA — order taking UI (works on phone browser) | 3 days | Frontend |
| Kitchen Display PWA — KOT view with status updates | 2 days | Frontend |
| Smart API Client for waiter/kitchen (auto-routes Cloud↔Local) | 2 days | Frontend |
| SQLite schema setup for desktop | 1 day | Backend |
| DataSource trait (PostgreSQL + SQLite abstraction) | 2 days | Backend |
| Sync engine — change tracking + sync queue | 2 days | Backend |
| Sync engine — push/pull protocol + conflict resolution | 2 days | Backend |
| Mode status indicator UI (☁️📡💾 icon) | 1 day | Frontend |
| QR code generation for waiter device setup | 0.5 day | Frontend |
| Desktop-specific features (system tray, auto-start) | 1.5 days | Frontend |

#### Sprint 6 (Weeks 13-14): Polish & Integration

| Task | Duration | Owner |
|---|---|---|
| Thermal printer integration (ESC/POS) | 3 days | Backend |
| Cash drawer support | 1 day | Backend |
| Barcode reader integration | 1 day | Backend |
| Low stock alerts system | 1 day | Backend |
| Purchase order workflow | 2 days | Full-stack |
| Reservation system | 2 days | Full-stack |
| Table merge/split/transfer | 2 days | Full-stack |
| Staff performance reports | 1 day | Full-stack |
| Discount management | 2 days | Full-stack |
| Settings & configuration UI | 2 days | Frontend |

#### Sprint 7 (Weeks 15-16): Testing, Bug Fixing & Launch Prep

| Task | Duration | Owner |
|---|---|---|
| End-to-end testing | 3 days | QA |
| Performance testing & optimization | 2 days | Backend |
| Security audit | 2 days | Backend |
| Bug fixing sprint | 4 days | All |
| Desktop app installer (Windows) | 1 day | DevOps |
| Auto-update system | 1 day | Backend |
| Production deployment | 1 day | DevOps |
| Documentation — user guide | 2 days | All |
| Documentation — API docs | 1 day | Backend |
| Beta launch with 5-10 restaurants | Ongoing | Business |

### 12.5 Phase 3: Growth Features (Weeks 17-24)

| Feature | Duration |
|---|---|
| Kitchen Display System (KDS) | 2 weeks |
| Customer-facing QR menu | 1 week |
| Loyalty & rewards program | 2 weeks |
| Swiggy/Zomato integration | 2 weeks |
| Advanced analytics dashboard | 2 weeks |
| Multi-language support (i18n) | 1 week |
| SMS/WhatsApp bill sharing | 1 week |
| Customer feedback system | 1 week |
| Mobile-responsive waiter pad (PWA) | 2 weeks |

### 12.6 Phase 4: Scale (Week 25+)

- Android/iOS native apps (via React Native or Tauri Mobile)
- Plugin/extension system
- Marketplace for integrations
- AI-powered demand forecasting
- Advanced recipe costing & optimization
- Multi-brand management
- Franchise management system
- White-labeling for enterprise clients

---

## 13. Team Structure & Roles

### 13.1 Minimum Viable Team (MVP)

```
┌─────────────────────────────────────────────────┐
│               MVP TEAM (5-7 people)              │
│                                                  │
│  ┌──────────────────────────────────────────┐   │
│  │  Founder / Tech Lead (1)                  │   │
│  │  • Architecture decisions                 │   │
│  │  • Code review                           │   │
│  │  • Rust backend development              │   │
│  │  • Team coordination                     │   │
│  └──────────────────────────────────────────┘   │
│                                                  │
│  ┌──────────────────┐  ┌──────────────────────┐ │
│  │ Backend Dev (1-2) │  │ Frontend Dev (1-2)   │ │
│  │ • Rust / Axum     │  │ • React / TypeScript │ │
│  │ • PostgreSQL      │  │ • Tauri integration  │ │
│  │ • API development │  │ • Next.js            │ │
│  │ • Sync engine     │  │ • UI/UX impl         │ │
│  └──────────────────┘  └──────────────────────┘ │
│                                                  │
│  ┌──────────────────┐  ┌──────────────────────┐ │
│  │ Full-stack (1)    │  │ UI/UX Designer (1)   │ │
│  │ • Feature dev     │  │ • Figma designs      │ │
│  │ • Bug fixes       │  │ • User research      │ │
│  │ • Testing         │  │ • Design system      │ │
│  └──────────────────┘  └──────────────────────┘ │
│                                                  │
│  ┌──────────────────────────────────────────┐   │
│  │  Business / Product (1) (Founder)         │   │
│  │  • Product roadmap                        │   │
│  │  • Customer interviews                    │   │
│  │  • Beta partnerships with restaurants     │   │
│  │  • Sales & marketing                      │   │
│  └──────────────────────────────────────────┘   │
│                                                  │
└─────────────────────────────────────────────────┘
```

### 13.2 Ideal Skills Matrix

| Role | Must-Have Skills | Nice-to-Have |
|---|---|---|
| **Tech Lead** | Rust (advanced), System design, PostgreSQL, API design | Tauri, Distributed systems |
| **Backend Dev** | Rust (intermediate+), SQL, REST API, Testing | Docker, Redis, Message queues |
| **Frontend Dev** | React (advanced), TypeScript, Tailwind, State management | Tauri, Next.js, WebSocket |
| **Full-stack** | React + Rust/Node.js, SQL, Git | DevOps, Testing |
| **Designer** | Figma, UI/UX principles, Design systems | Frontend development, Prototyping |

### 13.3 Scaling the Team (Post-MVP)

```
Post-MVP Team (8-12 people):
├── Add: DevOps Engineer (1)
├── Add: QA Engineer (1)
├── Add: Mobile Developer (1)
├── Add: Backend Developer (1)
├── Add: Frontend Developer (1)
└── Add: Customer Success (1)

Growth Team (15-25 people):
├── Add: Engineering Manager (1)
├── Add: Product Manager (1)
├── Add: Data Engineer (1)
├── Add: Security Engineer (1)
├── Add: Integration Developer (1-2)
├── Add: Support Engineers (2-3)
└── Add: Sales & Marketing (2-3)
```

---

## 14. Cost Estimation — Detailed Breakdown

### 14.1 Development Cost (MVP — 4 months)

#### Option A: In-House Team (Recommended for Founders)

| Role | Monthly Salary (₹) | Duration | Total (₹) |
|---|---|---|---|
| Tech Lead / Co-founder | ₹0 (Equity) | 4 months | ₹0 |
| Backend Developer (Rust) | ₹80,000 - 1,50,000 | 4 months | ₹3,20,000 - 6,00,000 |
| Frontend Developer (React) | ₹60,000 - 1,20,000 | 4 months | ₹2,40,000 - 4,80,000 |
| Full-stack Developer | ₹60,000 - 1,00,000 | 4 months | ₹2,40,000 - 4,00,000 |
| UI/UX Designer | ₹40,000 - 80,000 | 4 months | ₹1,60,000 - 3,20,000 |
| **Subtotal (Team)** | | | **₹9,60,000 - 18,00,000** |

> **Note:** Salaries based on Indian market for mid-level developers in Tier 1/2 cities. For Tier 3 cities or junior developers, costs can be 30-40% lower.

#### Option B: Outsourced Development (Agency)

| Scope | Estimated Cost (₹) |
|---|---|
| MVP Development (4 months) | ₹20,00,000 - 40,00,000 |
| With Rust expertise | ₹30,00,000 - 50,00,000 |

> **Warning:** Outsourcing is NOT recommended for a product startup. You lose control over architecture decisions, code quality, and iteration speed.

#### Option C: Hybrid (Founders + Freelancers)

| Component | Cost (₹) |
|---|---|
| 2 Founders building core (Equity only) | ₹0 |
| 1 Freelance Rust developer (part-time) | ₹2,00,000 - 4,00,000 |
| 1 Freelance Frontend developer (part-time) | ₹1,50,000 - 3,00,000 |
| 1 Freelance Designer (contract) | ₹80,000 - 1,50,000 |
| **Subtotal** | **₹4,30,000 - 8,50,000** |

### 14.2 Infrastructure Cost (Monthly)

#### MVP Phase (0-500 restaurants)

| Service | Provider | Monthly Cost (₹) |
|---|---|---|
| App Server (4vCPU, 8GB) | Hetzner/DigitalOcean | ₹2,000 - 3,000 |
| Database Server (4vCPU, 16GB) | Hetzner/DigitalOcean | ₹3,000 - 5,000 |
| Domain + DNS | Cloudflare | ₹500 |
| SSL Certificate | Let's Encrypt | ₹0 (Free) |
| Email Service (transactional) | Resend / AWS SES | ₹500 - 1,000 |
| Error Tracking | Sentry (free tier) | ₹0 |
| Monitoring | Grafana Cloud (free tier) | ₹0 |
| S3 Storage (images, backups) | AWS S3 / MinIO | ₹500 - 1,000 |
| GitHub (private repos) | GitHub Team | ₹1,500 |
| Figma (design) | Figma Starter | ₹1,000 |
| **Monthly Total** | | **₹8,000 - 12,000** |
| **Annual Total** | | **₹96,000 - 1,44,000** |

#### Growth Phase (500-5,000 restaurants)

| Service | Provider | Monthly Cost (₹) |
|---|---|---|
| AWS ECS Fargate (auto-scaling) | AWS | ₹15,000 - 30,000 |
| RDS PostgreSQL (Multi-AZ) | AWS | ₹10,000 - 20,000 |
| ElastiCache Redis | AWS | ₹5,000 - 10,000 |
| CloudFront CDN | AWS | ₹2,000 - 5,000 |
| S3 + Backups | AWS | ₹3,000 - 8,000 |
| Monitoring stack | Grafana Cloud | ₹5,000 |
| Email + SMS | SES + Twilio | ₹3,000 - 10,000 |
| **Monthly Total** | | **₹43,000 - 88,000** |
| **Annual Total** | | **₹5,16,000 - 10,56,000** |

### 14.3 Other Costs

| Item | One-time Cost (₹) | Recurring (₹/month) |
|---|---|---|
| Company Registration (LLP/Pvt Ltd) | ₹15,000 - 25,000 | — |
| GST Registration | ₹0 - 5,000 | — |
| Legal (terms, privacy policy) | ₹20,000 - 50,000 | — |
| Trademark Registration | ₹10,000 - 15,000 | — |
| SSL EV Certificate (for trust) | ₹10,000/year | — |
| Code Signing Certificate (Windows) | ₹15,000/year | — |
| POS Hardware for Testing | ₹20,000 - 40,000 | — |
| Marketing (initial launch) | ₹50,000 - 1,00,000 | ₹20,000 - 50,000 |
| Accounting Software | — | ₹1,000 |
| Communication (Slack/Discord) | — | ₹0 (free tier) |

### 14.4 Total Cost Summary

```
┌──────────────────────────────────────────────────────────┐
│              TOTAL MVP COST ESTIMATE                      │
│                                                           │
│  Development (4 months):                                  │
│  ├── Option A (In-house): ₹9,60,000 - ₹18,00,000       │
│  ├── Option B (Agency):   ₹20,00,000 - ₹50,00,000      │
│  └── Option C (Hybrid):   ₹4,30,000 - ₹8,50,000        │
│                                                           │
│  Infrastructure (4 months):  ₹32,000 - ₹48,000          │
│                                                           │
│  Other Costs:               ₹1,40,000 - ₹2,85,000       │
│                                                           │
│  ════════════════════════════════════════════════          │
│                                                           │
│  MINIMUM BUDGET (Option C):     ~₹6,00,000               │
│  RECOMMENDED BUDGET (Option A): ~₹12,00,000 - ₹20,00,000│
│  COMFORTABLE BUDGET:            ~₹25,00,000 - ₹35,00,000 │
│                                                           │
│  Runway needed: 6-8 months of operating costs            │
│  Recommended funding: ₹25,00,000 - ₹50,00,000           │
│                                                           │
└──────────────────────────────────────────────────────────┘
```

### 14.5 Revenue Model & Break-Even

| Plan | Monthly Price | Target Customers |
|---|---|---|
| **Free Tier** | ₹0 | Single outlet, basic features, 100 bills/day |
| **Starter** | ₹499/month | 1 outlet, all core features |
| **Professional** | ₹999/month | Up to 3 outlets, reports, sync |
| **Business** | ₹1,999/month | Up to 10 outlets, all features |
| **Enterprise** | Custom (₹5,000+/month) | Unlimited outlets, SLA, support |

**Break-even Analysis:**

```
Monthly costs (post-MVP): ~₹2,00,000
Average revenue per customer: ~₹800/month

Break-even: 250 paying customers
Target: 250 paying customers within 6 months of launch

With 1,000 customers:
Revenue: ₹8,00,000/month
Costs:   ₹3,50,000/month (including team + infra)
Profit:  ₹4,50,000/month
```

---

## 15. DevOps & CI/CD Pipeline

### 15.1 Monorepo Structure

```
restrosync/
├── .github/
│   └── workflows/
│       ├── ci.yml              # Run on every PR
│       ├── release-desktop.yml  # Build & publish desktop app
│       ├── release-web.yml      # Build & deploy web app
│       └── release-server.yml   # Build & deploy server
│
├── apps/
│   ├── desktop/                 # Tauri desktop application
│   │   ├── src-tauri/          # Rust backend for desktop
│   │   │   ├── src/
│   │   │   │   ├── main.rs
│   │   │   │   ├── commands/   # Tauri commands (IPC)
│   │   │   │   ├── db/        # SQLite operations
│   │   │   │   └── sync/      # Sync engine
│   │   │   ├── Cargo.toml
│   │   │   └── tauri.conf.json
│   │   ├── src/                # React frontend
│   │   │   ├── components/
│   │   │   ├── pages/
│   │   │   ├── hooks/
│   │   │   └── stores/
│   │   ├── package.json
│   │   └── vite.config.ts
│   │
│   ├── web/                    # Next.js web application
│   │   ├── src/
│   │   │   ├── app/           # App Router pages
│   │   │   ├── components/
│   │   │   └── lib/
│   │   ├── package.json
│   │   └── next.config.js
│   │
│   └── server/                 # Axum API server
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
│       │   │   ├── reports.rs
│       │   │   ├── tables.rs
│       │   │   └── sync.rs
│       │   ├── services/       # Business logic
│       │   ├── models/         # Database models
│       │   ├── middleware/
│       │   └── utils/
│       ├── migrations/
│       └── Cargo.toml
│
├── packages/
│   ├── ui/                     # Shared UI components (Shadcn)
│   │   ├── src/
│   │   │   ├── components/
│   │   │   │   ├── ui/        # Base components (button, input, etc.)
│   │   │   │   ├── billing/   # Billing components
│   │   │   │   ├── menu/      # Menu components
│   │   │   │   ├── orders/    # Order components
│   │   │   │   ├── tables/    # Table components
│   │   │   │   └── reports/   # Report components
│   │   │   └── lib/
│   │   └── package.json
│   │
│   ├── shared/                 # Shared types, utils, constants
│   │   ├── src/
│   │   │   ├── types/         # TypeScript types
│   │   │   ├── constants/
│   │   │   ├── utils/
│   │   │   └── validation/    # Zod schemas
│   │   └── package.json
│   │
│   └── core/                   # Shared Rust core library
│       ├── src/
│       │   ├── lib.rs
│       │   ├── billing/       # Billing logic (shared desktop + server)
│       │   ├── inventory/
│       │   ├── menu/
│       │   ├── orders/
│       │   ├── gst/           # GST calculation engine
│       │   └── models/        # Shared Rust types
│       └── Cargo.toml
│
├── tools/
│   ├── db-seed/               # Database seeding scripts
│   └── scripts/               # Utility scripts
│
├── Cargo.toml                  # Rust workspace root
├── turbo.json                  # Turborepo config
├── package.json                # Root package.json
├── pnpm-workspace.yaml
├── docker-compose.yml          # Local development services
├── docker-compose.prod.yml     # Production services
├── Dockerfile.server           # Server container
├── Dockerfile.web              # Web app container
├── .env.example
└── README.md
```

### 15.2 CI/CD Pipeline

```
On Pull Request:
┌──────────────────────────────────────────────────┐
│  1. Code Quality                                  │
│     ├── ESLint (TypeScript/JavaScript)            │
│     ├── Clippy (Rust)                            │
│     ├── Prettier format check                     │
│     └── rustfmt check                            │
│                                                   │
│  2. Type Checking                                 │
│     ├── TypeScript (tsc --noEmit)                │
│     └── Rust (cargo check)                       │
│                                                   │
│  3. Tests                                         │
│     ├── Rust unit tests (cargo test)             │
│     ├── Rust integration tests                    │
│     ├── React component tests (Vitest)           │
│     └── API endpoint tests                       │
│                                                   │
│  4. Security                                      │
│     ├── cargo audit                              │
│     └── npm audit                                │
│                                                   │
│  5. Build Check                                   │
│     ├── cargo build --release                    │
│     └── pnpm build (all packages)                │
└──────────────────────────────────────────────────┘

On Merge to Main:
┌──────────────────────────────────────────────────┐
│  1. All PR checks (above)                         │
│  2. Build Docker images                           │
│  3. Push to container registry                    │
│  4. Deploy to staging                             │
│  5. Run E2E tests on staging                     │
│  6. Manual approval gate                          │
│  7. Deploy to production (blue-green)             │
│  8. Smoke tests                                   │
│  9. Notify team                                   │
└──────────────────────────────────────────────────┘

On Release Tag (v*):
┌──────────────────────────────────────────────────┐
│  Desktop App:                                     │
│  1. Build Tauri for Windows (x64 + ARM)          │
│  2. Code sign the executable                      │
│  3. Create MSI/NSIS installer                     │
│  4. Upload to release server                      │
│  5. Publish auto-update manifest                  │
│  6. Notify existing installations                 │
│                                                   │
│  Web App:                                         │
│  1. Build Next.js production bundle              │
│  2. Deploy to production                          │
│  3. Invalidate CDN cache                          │
│  4. Run smoke tests                               │
│                                                   │
│  Server:                                          │
│  1. Build Rust release binary                     │
│  2. Build Docker image                            │
│  3. Deploy with zero-downtime rollout             │
│  4. Run database migrations                       │
│  5. Health check verification                     │
└──────────────────────────────────────────────────┘
```

---

## 16. Testing Strategy

### 16.1 Testing Pyramid

```
                    ╱╲
                   ╱  ╲
                  ╱ E2E╲          5% — Critical user flows
                 ╱──────╲         (Playwright / Cypress)
                ╱        ╲
               ╱Integration╲      20% — API, DB, module interaction
              ╱────────────╲      (Rust integration tests, Supertest)
             ╱              ╲
            ╱   Unit Tests    ╲    75% — Pure logic, components
           ╱────────────────────╲  (Vitest, cargo test)
          ╱                      ╲
         ╱──────────────────────────╲
```

### 16.2 Testing Breakdown

| Layer | Tool | What to Test | Coverage Target |
|---|---|---|---|
| **Rust Unit Tests** | cargo test | Business logic, calculations, GST engine | 90%+ |
| **Rust Integration** | cargo test + testcontainers | Database operations, API endpoints | 80%+ |
| **React Components** | Vitest + Testing Library | Component rendering, interactions | 80%+ |
| **API E2E** | cargo test / Hurl | Full request/response cycles | Critical paths |
| **UI E2E** | Playwright | Key user workflows (billing, ordering) | Top 10 flows |
| **Performance** | k6 / Criterion (Rust) | Response times, throughput | P95 < 100ms |

### 16.3 Critical Test Scenarios

```
Billing:
├── GST calculation accuracy (5%, 12%, 18% rates)
├── Discount application (flat, percentage, item-level)
├── Split bill calculations
├── Round-off accuracy
├── Multi-payment mode settlement
├── Bill void and refund
└── Concurrent billing (multiple terminals)

Inventory:
├── Auto-deduction accuracy from recipes
├── Stock level calculations after purchase + consumption
├── Low stock alert triggers
├── Unit conversion accuracy
└── Purchase order workflow

Sync:
├── Offline → Online sync (100+ pending changes)
├── Conflict resolution (same bill edited on 2 devices)
├── Data integrity after sync
├── Large dataset sync performance
└── Network interruption during sync
```

---

## 17. Performance & Scalability Targets

### 17.1 Performance Targets

| Metric | Target | Measurement |
|---|---|---|
| **Bill generation time** | < 50ms | From "Pay" click to bill number generated |
| **Item search** | < 30ms | Fuzzy search across 500+ menu items |
| **Report generation (daily)** | < 200ms | Daily sales report for single outlet |
| **Report generation (monthly)** | < 2s | Monthly report with 10,000+ bills |
| **Desktop app startup** | < 2s | Cold start to login screen |
| **Desktop app RAM usage** | < 100MB | Idle state with app open |
| **Desktop app bundle size** | < 15MB | Installer size |
| **API response time (P50)** | < 50ms | Simple CRUD operations |
| **API response time (P95)** | < 200ms | Complex queries |
| **API response time (P99)** | < 500ms | Heavy operations |
| **WebSocket latency** | < 100ms | Event propagation |
| **Sync time (100 records)** | < 3s | Full sync cycle |
| **Concurrent users per server** | 1,000+ | Active WebSocket connections |

### 17.2 Scalability Targets

| Scale Point | Restaurants | Daily Bills | Infrastructure |
|---|---|---|---|
| MVP Launch | 10-50 | 5,000 | Single server |
| 6 months | 100-500 | 50,000 | 2 servers + managed DB |
| 1 year | 500-2,000 | 200,000 | Auto-scaling cluster |
| 2 years | 2,000-10,000 | 1,000,000 | Multi-region deployment |
| 3 years | 10,000-50,000 | 5,000,000 | Full microservices + K8s |

### 17.3 Performance Optimization Strategies

1. **Database:**
   - Connection pooling (deadpool-postgres)
   - Prepared statements
   - Strategic indexing
   - Read replicas for reports
   - Materialized views for dashboards

2. **Caching:**
   - Redis for session data and hot lookups
   - Menu data cached (invalidate on change)
   - Report results cached with TTL
   - Client-side caching with TanStack Query

3. **Application:**
   - Rust's zero-cost abstractions
   - Async I/O throughout (Tokio)
   - Batch operations for bulk inserts
   - Lazy loading for UI components
   - Virtual scrolling for large lists

4. **Network:**
   - gzip/brotli compression
   - CDN for static assets
   - WebSocket for real-time (avoid polling)
   - API pagination with cursor-based approach

---

## 18. Future Expansion Roadmap

### 18.1 Feature Roadmap (12-Month View)

```
Month 1-4:    MVP Launch
              ├── 6 Core Modules
              ├── Windows Desktop App
              ├── Basic Web Dashboard
              └── Local + Cloud modes

Month 5-6:    Kitchen & Operations
              ├── Kitchen Display System (KDS)
              ├── Customer-facing QR Menu
              ├── Digital Receipts (WhatsApp)
              └── Multi-language Support

Month 7-8:    Integrations
              ├── Swiggy Integration
              ├── Zomato Integration
              ├── UPI Payment Gateway
              ├── Razorpay / PayU
              └── GST Filing Integration

Month 9-10:   Advanced Analytics
              ├── AI-powered Sales Forecasting
              ├── Food Cost Optimization
              ├── Custom Report Builder
              ├── Automated Daily Email Reports
              └── Dashboard Customization

Month 11-12:  Platform Expansion
              ├── Mobile App (Android — Primary)
              ├── Mobile App (iOS)
              ├── Tablet-optimized Waiter Pad
              ├── Plugin/Extension System
              └── API Marketplace
```

### 18.2 Long-Term Vision (Year 2-3)

| Feature | Description |
|---|---|
| **AI Recipe Optimization** | Suggest recipe changes to reduce food cost |
| **Demand Forecasting** | Predict daily demand based on history, weather, events |
| **Dynamic Pricing** | Auto-adjust prices based on demand, time, competition |
| **Multi-brand Management** | Manage multiple brands from one dashboard |
| **Franchise Management** | Centralized control with location-level customization |
| **CRM & Marketing** | Customer database, campaigns, loyalty programs |
| **Employee Management** | Scheduling, attendance, payroll integration |
| **Accounting Integration** | Tally, Zoho Books, QuickBooks sync |
| **White-label Solution** | Reseller/partner program |
| **Marketplace** | Third-party plugins, themes, integrations |
| **API Platform** | Public API for developers to build on |

---

## 19. Risk Assessment & Mitigation

### 19.1 Technical Risks

| Risk | Probability | Impact | Mitigation |
|---|---|---|---|
| **Rust hiring difficulty** | High | High | Invest in Rust training for team; strong documentation; consider Rust-curious developers who can learn |
| **Sync engine complexity** | High | High | Start with simple last-write-wins; iterate. Use proven libraries (CRDTs). Test extensively. |
| **Tauri immaturity** | Medium | Medium | Tauri 2.x is stable. Have fallback plan (Electron) but unlikely needed. Active community support. |
| **Performance on old hardware** | Medium | High | Test on low-end devices from day 1. Set up a test lab with ₹15,000 PCs. |
| **Thermal printer compatibility** | Medium | High | Test with 10+ popular Indian thermal printer models. ESC/POS is standard. |
| **Multi-tenancy data leakage** | Low | Critical | Schema-per-tenant isolation. Automated security testing. Code review for all DB queries. |

### 19.2 Business Risks

| Risk | Probability | Impact | Mitigation |
|---|---|---|---|
| **Petpooja price war** | High | High | Compete on features and privacy, not just price. Free tier as moat. |
| **Slow restaurant adoption** | High | Medium | Partner with 10 restaurants early. Free setup + support. Build trust. |
| **Cash burn before revenue** | High | Critical | Keep team small. Founders take equity, not salary. Target 250 paying customers in 6 months. |
| **Regulatory changes (GST)** | Medium | Medium | Modular tax engine that can be updated independently. Stay updated with CA advisors. |
| **Hardware dependency** | Medium | Medium | Support existing hardware (printers, cash drawers). Don't sell hardware initially. |
| **Team member departure** | Medium | High | Good documentation. Knowledge sharing sessions. Equity vesting (4 years, 1 year cliff). |

### 19.3 Mitigation Priority Matrix

```
                    HIGH IMPACT
                        │
    ┌───────────────────┼───────────────────┐
    │                   │                   │
    │  Sync Engine      │  Rust Hiring      │
    │  Complexity       │  Cash Burn        │
    │                   │  Slow Adoption    │
    │  (Mitigate Early) │  (Plan for These) │
    │                   │                   │
────┼───────────────────┼───────────────────┼────
    │                   │                   │
LOW │  Regulatory       │  Multi-tenancy    │ HIGH
PROB│  Changes          │  Leakage          │ PROB
    │  Hardware Deps    │                   │
    │                   │                   │
    │  (Monitor)        │  (Prevent)        │
    │                   │                   │
    └───────────────────┼───────────────────┘
                        │
                   LOW IMPACT
```

---

## 20. Key Architectural Decisions Record (ADR)

### ADR-001: Rust as Primary Backend Language

- **Status:** Accepted
- **Context:** Need a language that delivers high performance, memory safety, and can be shared between desktop (Tauri) and server.
- **Decision:** Use Rust for all backend code (desktop core + API server + embedded local server).
- **Consequences:** Steeper learning curve, smaller talent pool, but significantly better performance, safety, and resource efficiency. Single language for entire backend stack. Critically, allows us to embed a lightweight Axum server inside the desktop app for local WiFi serving.

### ADR-002: Tauri over Electron for Desktop

- **Status:** Accepted
- **Context:** Need a cross-platform desktop framework. Primary target is Windows. App will run all day on POS terminals. Must also host an embedded HTTP server for local WiFi device serving.
- **Decision:** Use Tauri 2.x instead of Electron.
- **Consequences:** 10x smaller bundle, 5x less RAM, native performance. Rust backend allows us to embed an Axum web server directly into the desktop app binary. Less mature ecosystem than Electron, but actively developed and Rust-native.

### ADR-003: Modular Monolith (MVP) → Microservices (Scale)

- **Status:** Accepted
- **Context:** Starting as a small team building an MVP. Don't need distributed systems overhead yet.
- **Decision:** Build as a modular monolith with clear module boundaries. Extract to microservices when needed.
- **Consequences:** Faster development, simpler deployment, easier debugging. Need discipline to maintain module boundaries.

### ADR-004: Schema-per-Tenant Multi-tenancy

- **Status:** Accepted
- **Context:** Need data isolation between restaurants while keeping infrastructure simple.
- **Decision:** Use PostgreSQL schema-per-tenant approach.
- **Consequences:** Good isolation, easy backup/restore per tenant. Slightly more complex migration management. Can migrate high-traffic tenants to dedicated databases later.

### ADR-005: SQLite for Local Desktop Database

- **Status:** Accepted
- **Context:** Desktop app needs a local database for offline-first operation. Embedded local server also needs a data store that doesn't require installing separate software.
- **Decision:** Use SQLite as the embedded database in the Tauri desktop app. The embedded local server also reads/writes to this same SQLite database.
- **Consequences:** Zero configuration, battle-tested, handles millions of records. Single-writer limitation managed via WAL mode and short transactions. Waiter/kitchen requests go through embedded server → SQLite.

### ADR-006: Last-Write-Wins + Server Authority for Sync

- **Status:** Accepted
- **Context:** Need a conflict resolution strategy for local ↔ cloud sync.
- **Decision:** Last-write-wins for most data. Server is authoritative for finalized financial data (bills, payments).
- **Consequences:** Simple to implement, occasional data loss in rare conflict scenarios. For financial data, local changes queue until server confirms. Can evolve to CRDTs later if needed.

### ADR-007: React + TypeScript for Frontend

- **Status:** Accepted
- **Context:** Need a frontend framework for desktop (Tauri webview), web (Next.js), waiter PWA, and kitchen display.
- **Decision:** React 19 with TypeScript, shared component library via Turborepo. Waiter and Kitchen UIs are lightweight React PWAs served by the embedded local server or cloud.
- **Consequences:** Maximum code reuse between desktop, web, waiter, and kitchen UIs. Huge ecosystem, easy hiring.

### ADR-008: NATS over RabbitMQ/Kafka for Messaging

- **Status:** Accepted
- **Context:** Need a message queue for async operations and future microservice communication.
- **Decision:** Use NATS for pub/sub and task queuing.
- **Consequences:** Extremely lightweight (~15MB binary), Rust-native client, simple to operate. Less feature-rich than Kafka but sufficient for our scale. Can add Kafka later for analytics/streaming.

### ADR-009: Fully Dynamic Auto-Switching Networking (NEW)

- **Status:** Accepted
- **Context:** Restaurants in India have unreliable internet. Some have no internet at all. But they still need multi-device operation (waiter phones, kitchen display). We cannot force them to choose a mode — it must be automatic.
- **Decision:** Implement a Smart Connection Manager that auto-detects Cloud/Local WiFi/Standalone connectivity and seamlessly switches between modes. Embed a lightweight Axum HTTP+WebSocket server inside the Tauri desktop app so the Cashier PC can serve waiter and kitchen devices on local WiFi when internet is down.
- **Consequences:** More complex architecture to build (embedded server + mode switching + device discovery). But this is our **single biggest competitive advantage** over Petpooja and every other competitor. No other POS system in India offers true auto-switching multi-device operation without internet. Worth the engineering investment.

### ADR-010: Unified API Contract Across Cloud & Local Server (NEW)

- **Status:** Accepted
- **Context:** Waiter phones and kitchen tablets should not need to know whether they're talking to the cloud server or the local Cashier PC server.
- **Decision:** Both the cloud Axum server and the embedded local Axum server implement the **exact same REST + WebSocket API contract**. They share the same route handlers from `packages/core`. The only difference is the data source (PostgreSQL vs SQLite), abstracted behind a `DataSource` trait.
- **Consequences:** One codebase for business logic. Waiter/kitchen Smart API Client just changes the base URL. Zero code duplication. API tests run against both backends.

### ADR-011: mDNS for Zero-Config Device Discovery (NEW)

- **Status:** Accepted
- **Context:** When a waiter opens the app on their phone, they need to find the Cashier PC server on the local WiFi without manually entering an IP address.
- **Decision:** Use mDNS (Multicast DNS) for service discovery. Cashier PC broadcasts `_restrosync._tcp.local`. Waiter/kitchen devices discover it automatically. Fallback: QR code scan or manual IP entry.
- **Consequences:** Zero-config setup for non-technical restaurant staff. mDNS works on all modern devices and operating systems. QR code fallback handles edge cases where mDNS is blocked.

---

## 21. Conclusion

### What We're Building

RestroSync is not just another POS system — it's a **platform** designed to evolve with the Indian restaurant industry. By starting with a solid foundation (Rust, fully dynamic networking, modular design), we're setting ourselves up for:

1. **Technical superiority** — Faster, lighter, more reliable than any competitor
2. **Unique market position** — The ONLY solution offering fully dynamic auto-switching networking (Cloud ↔ Local WiFi ↔ Standalone) with multi-device support without internet
3. **Zero-friction multi-device** — Waiters use their own phones, kitchen uses a cheap tablet, all working on local WiFi — no internet bill needed
4. **Scalable business** — From a single chai stall to a 500-outlet chain, same platform
5. **Developer ecosystem** — Plugin system creates a moat and community

### Immediate Next Steps

| # | Action | Owner | Deadline |
|---|---|---|---|
| 1 | Finalize team (hire/commit co-founders) | Founders | Week 1 |
| 2 | Set up development environment (monorepo, CI) | Tech Lead | Week 1-2 |
| 3 | Design database schema (detailed) | Tech Lead + Backend | Week 1-2 |
| 4 | Create Figma designs for billing flow | Designer | Week 1-3 |
| 5 | Build Tauri + React hello world with SQLite | Frontend + Backend | Week 2 |
| 6 | Build Axum server with auth + first CRUD | Backend | Week 2-3 |
| 7 | Partner with 3-5 restaurants for beta testing | Business Founder | Week 1-4 |
| 8 | Register company (LLP/Pvt Ltd) | Business Founder | Week 1-2 |

### The Bottom Line

```
┌─────────────────────────────────────────────────────────────┐
│                                                              │
│   MVP Budget:           ₹12,00,000 - ₹20,00,000            │
│   MVP Timeline:         4 months (16 weeks)                  │
│   Team Size:            5-7 people                           │
│   Time to First Revenue: 5-6 months                         │
│   Break-even:           250 paying customers                 │
│   Target Market:        75,000+ restaurants (India alone)    │
│                                                              │
│   KILLER FEATURE: Fully dynamic networking.                  │
│   Internet? Cloud mode. WiFi only? Local server mode.        │
│   Nothing? Standalone. Auto-switches. User never configures. │
│   No competitor in India can do this.                        │
│                                                              │
│   The opportunity is massive.                                │
│   The technology is ready.                                   │
│   The competition has gaps.                                  │
│                                                              │
│   Let's build. 🚀                                            │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

---

*This document is a living blueprint. It will be updated as we make progress, learn from users, and adapt to market needs. Every architectural decision recorded here should be revisited quarterly.*

**Last Updated:** June 2025 — v1.1.0 (Added Fully Dynamic Networking Architecture)
**Next Review:** After Phase 0 completion
