-- RestroSync SQLite Schema (mirrors PostgreSQL with sync columns)
-- Each table includes: sync_status, cloud_id, last_synced_at

-- Plans
CREATE TABLE IF NOT EXISTS plans (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    max_outlets INTEGER NOT NULL DEFAULT 1,
    max_users INTEGER NOT NULL DEFAULT 5,
    features TEXT NOT NULL DEFAULT '{}',
    price_monthly REAL NOT NULL,
    price_yearly REAL NOT NULL,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    -- Sync columns
    sync_status TEXT NOT NULL DEFAULT 'pending',
    cloud_id TEXT,
    last_synced_at TEXT
);

-- Tenants
CREATE TABLE IF NOT EXISTS tenants (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    slug TEXT NOT NULL UNIQUE,
    plan_id TEXT NOT NULL REFERENCES plans(id),
    gst_number TEXT,
    fssai_number TEXT,
    contact_email TEXT NOT NULL,
    contact_phone TEXT NOT NULL,
    address TEXT,
    city TEXT,
    state TEXT,
    pincode TEXT,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    sync_status TEXT NOT NULL DEFAULT 'pending',
    cloud_id TEXT,
    last_synced_at TEXT
);

-- Outlets
CREATE TABLE IF NOT EXISTS outlets (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    code TEXT NOT NULL,
    address TEXT,
    city TEXT,
    state TEXT,
    pincode TEXT,
    phone TEXT,
    email TEXT,
    gst_number TEXT,
    fssai_number TEXT,
    timezone_offset TEXT NOT NULL DEFAULT '+05:30',
    currency_code TEXT NOT NULL DEFAULT 'INR',
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    sync_status TEXT NOT NULL DEFAULT 'pending',
    cloud_id TEXT,
    last_synced_at TEXT,
    UNIQUE(tenant_id, code)
);

-- Roles
CREATE TABLE IF NOT EXISTS roles (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    description TEXT,
    is_system INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    sync_status TEXT NOT NULL DEFAULT 'pending',
    cloud_id TEXT,
    last_synced_at TEXT
);

-- Permissions
CREATE TABLE IF NOT EXISTS permissions (
    id TEXT PRIMARY KEY,
    module TEXT NOT NULL,
    action TEXT NOT NULL,
    description TEXT,
    UNIQUE(module, action)
);

-- Role permissions
CREATE TABLE IF NOT EXISTS role_permissions (
    role_id TEXT NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    permission_id TEXT NOT NULL REFERENCES permissions(id) ON DELETE CASCADE,
    PRIMARY KEY (role_id, permission_id)
);

-- Users
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    outlet_id TEXT REFERENCES outlets(id) ON DELETE SET NULL,
    name TEXT NOT NULL,
    email TEXT,
    phone TEXT NOT NULL,
    pin TEXT NOT NULL,
    password_hash TEXT,
    is_active INTEGER NOT NULL DEFAULT 1,
    last_login_at TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    sync_status TEXT NOT NULL DEFAULT 'pending',
    cloud_id TEXT,
    last_synced_at TEXT,
    UNIQUE(tenant_id, phone)
);

-- User roles
CREATE TABLE IF NOT EXISTS user_roles (
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role_id TEXT NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    assigned_at TEXT NOT NULL DEFAULT (datetime('now')),
    PRIMARY KEY (user_id, role_id)
);

-- Menu categories (hierarchical with parent_id for subcategories)
CREATE TABLE IF NOT EXISTS menu_categories (
    id TEXT PRIMARY KEY,
    outlet_id TEXT NOT NULL REFERENCES outlets(id) ON DELETE CASCADE,
    parent_id TEXT REFERENCES menu_categories(id) ON DELETE SET NULL,
    name TEXT NOT NULL,
    description TEXT,
    sort_order INTEGER NOT NULL DEFAULT 0,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    sync_status TEXT NOT NULL DEFAULT 'pending',
    cloud_id TEXT,
    last_synced_at TEXT
);

-- Menu items
CREATE TABLE IF NOT EXISTS menu_items (
    id TEXT PRIMARY KEY,
    category_id TEXT NOT NULL REFERENCES menu_categories(id) ON DELETE CASCADE,
    outlet_id TEXT NOT NULL REFERENCES outlets(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    description TEXT,
    short_code TEXT,
    base_price REAL NOT NULL,
    gst_percent REAL NOT NULL DEFAULT 5.0,
    is_veg INTEGER NOT NULL DEFAULT 1,
    is_available INTEGER NOT NULL DEFAULT 1,
    is_active INTEGER NOT NULL DEFAULT 1,
    sort_order INTEGER NOT NULL DEFAULT 0,
    image_url TEXT,
    preparation_time INTEGER,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    sync_status TEXT NOT NULL DEFAULT 'pending',
    cloud_id TEXT,
    last_synced_at TEXT
);

-- Menu variants
CREATE TABLE IF NOT EXISTS menu_variants (
    id TEXT PRIMARY KEY,
    menu_item_id TEXT NOT NULL REFERENCES menu_items(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    price REAL NOT NULL,
    is_active INTEGER NOT NULL DEFAULT 1,
    sort_order INTEGER NOT NULL DEFAULT 0,
    sync_status TEXT NOT NULL DEFAULT 'pending',
    cloud_id TEXT,
    last_synced_at TEXT
);

-- Modifier groups
CREATE TABLE IF NOT EXISTS modifier_groups (
    id TEXT PRIMARY KEY,
    outlet_id TEXT NOT NULL REFERENCES outlets(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    min_selection INTEGER NOT NULL DEFAULT 0,
    max_selection INTEGER NOT NULL DEFAULT 1,
    is_required INTEGER NOT NULL DEFAULT 0,
    is_active INTEGER NOT NULL DEFAULT 1,
    sync_status TEXT NOT NULL DEFAULT 'pending',
    cloud_id TEXT,
    last_synced_at TEXT
);

-- Modifiers
CREATE TABLE IF NOT EXISTS modifiers (
    id TEXT PRIMARY KEY,
    group_id TEXT NOT NULL REFERENCES modifier_groups(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    price REAL NOT NULL DEFAULT 0,
    is_active INTEGER NOT NULL DEFAULT 1,
    sort_order INTEGER NOT NULL DEFAULT 0,
    sync_status TEXT NOT NULL DEFAULT 'pending',
    cloud_id TEXT,
    last_synced_at TEXT
);

-- Menu item modifier groups
CREATE TABLE IF NOT EXISTS menu_item_modifier_groups (
    menu_item_id TEXT NOT NULL REFERENCES menu_items(id) ON DELETE CASCADE,
    modifier_group_id TEXT NOT NULL REFERENCES modifier_groups(id) ON DELETE CASCADE,
    PRIMARY KEY (menu_item_id, modifier_group_id)
);

-- Floors
CREATE TABLE IF NOT EXISTS floors (
    id TEXT PRIMARY KEY,
    outlet_id TEXT NOT NULL REFERENCES outlets(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    sort_order INTEGER NOT NULL DEFAULT 0,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    sync_status TEXT NOT NULL DEFAULT 'pending',
    cloud_id TEXT,
    last_synced_at TEXT
);

-- Restaurant tables
CREATE TABLE IF NOT EXISTS restaurant_tables (
    id TEXT PRIMARY KEY,
    floor_id TEXT NOT NULL REFERENCES floors(id) ON DELETE CASCADE,
    outlet_id TEXT NOT NULL REFERENCES outlets(id) ON DELETE CASCADE,
    number TEXT NOT NULL,
    capacity INTEGER NOT NULL DEFAULT 4,
    status TEXT NOT NULL DEFAULT 'available',
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    sync_status TEXT NOT NULL DEFAULT 'pending',
    cloud_id TEXT,
    last_synced_at TEXT,
    UNIQUE(outlet_id, number)
);

-- Reservations
CREATE TABLE IF NOT EXISTS reservations (
    id TEXT PRIMARY KEY,
    table_id TEXT NOT NULL REFERENCES restaurant_tables(id) ON DELETE CASCADE,
    outlet_id TEXT NOT NULL REFERENCES outlets(id) ON DELETE CASCADE,
    customer_name TEXT NOT NULL,
    customer_phone TEXT NOT NULL,
    guest_count INTEGER NOT NULL,
    reserved_for TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'confirmed',
    notes TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    sync_status TEXT NOT NULL DEFAULT 'pending',
    cloud_id TEXT,
    last_synced_at TEXT
);

-- Orders
CREATE TABLE IF NOT EXISTS orders (
    id TEXT PRIMARY KEY,
    outlet_id TEXT NOT NULL REFERENCES outlets(id) ON DELETE CASCADE,
    order_number TEXT NOT NULL,
    order_type TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    table_id TEXT,
    customer_id TEXT,
    customer_name TEXT,
    customer_phone TEXT,
    guest_count INTEGER,
    subtotal REAL NOT NULL DEFAULT 0,
    tax_total REAL NOT NULL DEFAULT 0,
    discount_total REAL NOT NULL DEFAULT 0,
    grand_total REAL NOT NULL DEFAULT 0,
    notes TEXT,
    created_by TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    sync_status TEXT NOT NULL DEFAULT 'pending',
    cloud_id TEXT,
    last_synced_at TEXT,
    UNIQUE(outlet_id, order_number)
);

-- Order items
CREATE TABLE IF NOT EXISTS order_items (
    id TEXT PRIMARY KEY,
    order_id TEXT NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    menu_item_id TEXT NOT NULL,
    variant_id TEXT,
    name TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    unit_price REAL NOT NULL,
    tax_amount REAL NOT NULL DEFAULT 0,
    total_price REAL NOT NULL,
    modifiers_json TEXT,
    notes TEXT,
    status TEXT NOT NULL DEFAULT 'pending',
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    sync_status TEXT NOT NULL DEFAULT 'pending',
    cloud_id TEXT,
    last_synced_at TEXT
);

-- KOTs
CREATE TABLE IF NOT EXISTS kots (
    id TEXT PRIMARY KEY,
    order_id TEXT NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    kot_number TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    created_by TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    sync_status TEXT NOT NULL DEFAULT 'pending',
    cloud_id TEXT,
    last_synced_at TEXT
);

-- KOT items
CREATE TABLE IF NOT EXISTS kot_items (
    id TEXT PRIMARY KEY,
    kot_id TEXT NOT NULL REFERENCES kots(id) ON DELETE CASCADE,
    order_item_id TEXT NOT NULL REFERENCES order_items(id) ON DELETE CASCADE,
    quantity INTEGER NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    notes TEXT,
    sync_status TEXT NOT NULL DEFAULT 'pending',
    cloud_id TEXT,
    last_synced_at TEXT
);

-- Bills
CREATE TABLE IF NOT EXISTS bills (
    id TEXT PRIMARY KEY,
    outlet_id TEXT NOT NULL REFERENCES outlets(id) ON DELETE CASCADE,
    order_id TEXT NOT NULL,
    bill_number TEXT NOT NULL,
    subtotal REAL NOT NULL,
    cgst_amount REAL NOT NULL DEFAULT 0,
    sgst_amount REAL NOT NULL DEFAULT 0,
    discount_amount REAL NOT NULL DEFAULT 0,
    round_off REAL NOT NULL DEFAULT 0,
    grand_total REAL NOT NULL,
    payment_status TEXT NOT NULL DEFAULT 'unpaid',
    is_cancelled INTEGER NOT NULL DEFAULT 0,
    cancel_reason TEXT,
    created_by TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    sync_status TEXT NOT NULL DEFAULT 'pending',
    cloud_id TEXT,
    last_synced_at TEXT,
    UNIQUE(outlet_id, bill_number)
);

-- Bill items
CREATE TABLE IF NOT EXISTS bill_items (
    id TEXT PRIMARY KEY,
    bill_id TEXT NOT NULL REFERENCES bills(id) ON DELETE CASCADE,
    menu_item_id TEXT NOT NULL,
    name TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    unit_price REAL NOT NULL,
    gst_percent REAL NOT NULL,
    tax_amount REAL NOT NULL,
    total_price REAL NOT NULL,
    sync_status TEXT NOT NULL DEFAULT 'pending',
    cloud_id TEXT,
    last_synced_at TEXT
);

-- Payments
CREATE TABLE IF NOT EXISTS payments (
    id TEXT PRIMARY KEY,
    bill_id TEXT NOT NULL REFERENCES bills(id) ON DELETE CASCADE,
    amount REAL NOT NULL,
    mode TEXT NOT NULL,
    reference_number TEXT,
    status TEXT NOT NULL DEFAULT 'completed',
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    sync_status TEXT NOT NULL DEFAULT 'pending',
    cloud_id TEXT,
    last_synced_at TEXT
);

-- Discounts
CREATE TABLE IF NOT EXISTS discounts (
    id TEXT PRIMARY KEY,
    outlet_id TEXT NOT NULL REFERENCES outlets(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    discount_type TEXT NOT NULL,
    value REAL NOT NULL,
    max_amount REAL,
    min_order_value REAL,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    sync_status TEXT NOT NULL DEFAULT 'pending',
    cloud_id TEXT,
    last_synced_at TEXT
);

-- Raw materials
CREATE TABLE IF NOT EXISTS raw_materials (
    id TEXT PRIMARY KEY,
    outlet_id TEXT NOT NULL REFERENCES outlets(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    sku TEXT,
    unit TEXT NOT NULL,
    current_stock REAL NOT NULL DEFAULT 0,
    min_stock REAL NOT NULL DEFAULT 0,
    cost_per_unit REAL NOT NULL DEFAULT 0,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    sync_status TEXT NOT NULL DEFAULT 'pending',
    cloud_id TEXT,
    last_synced_at TEXT
);

-- Recipes
CREATE TABLE IF NOT EXISTS recipes (
    id TEXT PRIMARY KEY,
    menu_item_id TEXT NOT NULL REFERENCES menu_items(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    yield_amount REAL NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    sync_status TEXT NOT NULL DEFAULT 'pending',
    cloud_id TEXT,
    last_synced_at TEXT
);

-- Recipe ingredients
CREATE TABLE IF NOT EXISTS recipe_ingredients (
    id TEXT PRIMARY KEY,
    recipe_id TEXT NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
    raw_material_id TEXT NOT NULL REFERENCES raw_materials(id) ON DELETE CASCADE,
    quantity REAL NOT NULL,
    unit TEXT NOT NULL,
    sync_status TEXT NOT NULL DEFAULT 'pending',
    cloud_id TEXT,
    last_synced_at TEXT
);

-- Stock movements
CREATE TABLE IF NOT EXISTS stock_movements (
    id TEXT PRIMARY KEY,
    raw_material_id TEXT NOT NULL REFERENCES raw_materials(id) ON DELETE CASCADE,
    movement_type TEXT NOT NULL,
    quantity REAL NOT NULL,
    unit_cost REAL,
    reference TEXT,
    notes TEXT,
    created_by TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    sync_status TEXT NOT NULL DEFAULT 'pending',
    cloud_id TEXT,
    last_synced_at TEXT
);

-- Vendors
CREATE TABLE IF NOT EXISTS vendors (
    id TEXT PRIMARY KEY,
    outlet_id TEXT NOT NULL REFERENCES outlets(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    contact_name TEXT,
    phone TEXT,
    email TEXT,
    address TEXT,
    gst_number TEXT,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    sync_status TEXT NOT NULL DEFAULT 'pending',
    cloud_id TEXT,
    last_synced_at TEXT
);

-- Purchase orders
CREATE TABLE IF NOT EXISTS purchase_orders (
    id TEXT PRIMARY KEY,
    outlet_id TEXT NOT NULL REFERENCES outlets(id) ON DELETE CASCADE,
    vendor_id TEXT NOT NULL REFERENCES vendors(id) ON DELETE CASCADE,
    po_number TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'draft',
    total_amount REAL NOT NULL DEFAULT 0,
    notes TEXT,
    expected_date TEXT,
    received_date TEXT,
    created_by TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    sync_status TEXT NOT NULL DEFAULT 'pending',
    cloud_id TEXT,
    last_synced_at TEXT
);

-- Purchase order items
CREATE TABLE IF NOT EXISTS purchase_order_items (
    id TEXT PRIMARY KEY,
    purchase_order_id TEXT NOT NULL REFERENCES purchase_orders(id) ON DELETE CASCADE,
    raw_material_id TEXT NOT NULL REFERENCES raw_materials(id) ON DELETE CASCADE,
    quantity REAL NOT NULL,
    unit_cost REAL NOT NULL,
    total_cost REAL NOT NULL,
    received_quantity REAL NOT NULL DEFAULT 0,
    sync_status TEXT NOT NULL DEFAULT 'pending',
    cloud_id TEXT,
    last_synced_at TEXT
);

-- Create indexes for common queries
CREATE INDEX IF NOT EXISTS idx_orders_outlet_status ON orders(outlet_id, status);
CREATE INDEX IF NOT EXISTS idx_orders_created_at ON orders(created_at);
CREATE INDEX IF NOT EXISTS idx_bills_outlet_created ON bills(outlet_id, created_at);
CREATE INDEX IF NOT EXISTS idx_menu_items_category ON menu_items(category_id);
CREATE INDEX IF NOT EXISTS idx_menu_items_outlet ON menu_items(outlet_id);
CREATE INDEX IF NOT EXISTS idx_sync_status_pending ON orders(sync_status) WHERE sync_status = 'pending';

-- Report/analytics performance indexes (task 0.8.6)
CREATE INDEX IF NOT EXISTS idx_bills_created_at ON bills(created_at);
CREATE INDEX IF NOT EXISTS idx_bill_items_menu_item_id ON bill_items(menu_item_id);
CREATE INDEX IF NOT EXISTS idx_bills_outlet_created_at ON bills(outlet_id, created_at);
CREATE INDEX IF NOT EXISTS idx_menu_categories_parent ON menu_categories(parent_id);
