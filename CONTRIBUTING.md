# Contributing to RestroSync

## Development Setup

1. **Prerequisites**: Node.js 20+, pnpm 10+, Rust 1.77+, Docker
2. **Install dependencies**: `pnpm install`
3. **Start services**: `docker compose up -d`
4. **Run migrations**: `cargo run --bin migrate`
5. **Seed data**: `cargo run --bin seed`
6. **Start dev servers**: `pnpm dev`

## Branch Naming

- `feat/<description>` — New features
- `fix/<description>` — Bug fixes
- `refactor/<description>` — Code refactoring
- `docs/<description>` — Documentation changes
- `chore/<description>` — Build, CI, tooling changes

## Commit Conventions

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat(billing): add split payment support
fix(orders): correct GST calculation for takeaway
refactor(core): extract validation into shared module
docs: update API endpoint documentation
chore(ci): add Rust clippy check to pipeline
```

**Scopes**: `billing`, `orders`, `menu`, `inventory`, `tables`, `auth`, `core`, `ui`, `desktop`, `web`, `server`, `ci`

## Pull Request Process

1. Create a feature branch from `main`
2. Make changes following the code style guidelines
3. Ensure all checks pass: `just lint && just test`
4. Open a PR with a clear title and description
5. Request review from at least one team member
6. Squash merge into `main` after approval

## Code Style

- **Rust**: Follow `rustfmt` defaults, pass `clippy` with no warnings
- **TypeScript/React**: Follow ESLint + Prettier config
- **Commits**: Pre-commit hooks run `lint-staged` automatically

## Project Structure

```
apps/server/       — Axum REST API (Rust)
apps/desktop/      — Tauri desktop app (Rust + React)
apps/web/          — Next.js web dashboard
packages/core/     — Shared Rust library (types, business logic)
packages/ui/       — Shared React component library
packages/shared/   — Shared TypeScript types & utils
tools/db-seed/     — Database seeding tool
```
