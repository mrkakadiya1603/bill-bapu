# RestroSync development commands

# Start all development servers
dev:
    pnpm dev

# Start only the backend server
dev-server:
    cargo run --bin server

# Start the web app
dev-web:
    pnpm --filter @restrosync/web dev

# Start the desktop app (requires Tauri)
dev-desktop:
    pnpm --filter @restrosync/desktop tauri dev

# Build all
build:
    pnpm build && cargo build --release

# Lint everything
lint:
    cargo clippy --workspace -- -D warnings && cargo fmt --check && pnpm lint

# Run all tests
test:
    cargo test --workspace && pnpm test

# Format code
format:
    cargo fmt --all && pnpm format

# Start Docker services
services-up:
    docker compose up -d

# Stop Docker services
services-down:
    docker compose down

# Run database migrations
db-migrate:
    cargo run --bin migrate

# Seed database
db-seed:
    cargo run --bin seed

# Full reset: stop services, remove volumes, restart, migrate, seed
db-reset:
    docker compose down -v && docker compose up -d && sleep 3 && cargo run --bin migrate && cargo run --bin seed

# Generate TypeScript types from Rust structs
generate-types:
    TS_RS_EXPORT_DIR={{justfile_directory()}}/packages/shared/src/generated cargo test -p restrosync-core --lib

# Clean all build artifacts
clean:
    cargo clean && pnpm clean
