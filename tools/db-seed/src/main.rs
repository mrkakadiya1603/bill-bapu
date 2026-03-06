use anyhow::Result;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use uuid::Uuid;

mod entities;

use entities::*;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt().with_env_filter("info").init();

    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://restrosync:restrosync_dev@localhost:5432/restrosync".into()
    });

    tracing::info!("Seeding database: {}", database_url);

    let db = sea_orm::Database::connect(&database_url).await?;

    seed_data(&db).await?;

    tracing::info!("Seeding completed successfully");
    Ok(())
}

async fn seed_data(db: &DatabaseConnection) -> Result<()> {
    // 1. Create plan
    let plan_id = Uuid::now_v7();
    plans::ActiveModel {
        id: Set(plan_id),
        name: Set("Professional".to_string()),
        max_outlets: Set(5),
        max_users: Set(25),
        features: Set(serde_json::json!({"billing": true, "inventory": true, "reports": true, "multi_outlet": true})),
        price_monthly: Set(rust_decimal::Decimal::new(299_900, 2)),
        price_yearly: Set(rust_decimal::Decimal::new(2_999_000, 2)),
        is_active: Set(true),
        ..Default::default()
    }
    .insert(db)
    .await?;
    tracing::info!("Created plan: Professional");

    // 2. Create tenant
    let tenant_id = Uuid::now_v7();
    tenants::ActiveModel {
        id: Set(tenant_id),
        name: Set("Spice Garden Restaurant".to_string()),
        slug: Set("spice-garden".to_string()),
        plan_id: Set(plan_id),
        gst_number: Set(Some("27AABCT1332L1ZT".to_string())),
        fssai_number: Set(Some("10020022000123".to_string())),
        contact_email: Set("info@spicegarden.in".to_string()),
        contact_phone: Set("+919876543210".to_string()),
        address: Set(Some("123 MG Road".to_string())),
        city: Set(Some("Mumbai".to_string())),
        state: Set(Some("Maharashtra".to_string())),
        pincode: Set(Some("400001".to_string())),
        is_active: Set(true),
        ..Default::default()
    }
    .insert(db)
    .await?;
    tracing::info!("Created tenant: Spice Garden Restaurant");

    // 3. Create outlet
    let outlet_id = Uuid::now_v7();
    outlets::ActiveModel {
        id: Set(outlet_id),
        tenant_id: Set(tenant_id),
        name: Set("Spice Garden - Andheri".to_string()),
        code: Set("SG-AND".to_string()),
        address: Set(Some("45 Link Road, Andheri West".to_string())),
        city: Set(Some("Mumbai".to_string())),
        state: Set(Some("Maharashtra".to_string())),
        pincode: Set(Some("400053".to_string())),
        phone: Set(Some("+912226543210".to_string())),
        email: Set(Some("andheri@spicegarden.in".to_string())),
        gst_number: Set(Some("27AABCT1332L1ZT".to_string())),
        is_active: Set(true),
        ..Default::default()
    }
    .insert(db)
    .await?;
    tracing::info!("Created outlet: Spice Garden - Andheri");

    // 4. Create roles
    let admin_role_id = Uuid::now_v7();
    let cashier_role_id = Uuid::now_v7();
    let waiter_role_id = Uuid::now_v7();
    let kitchen_role_id = Uuid::now_v7();

    for (id, name, desc) in [
        (admin_role_id, "Admin", "Full system access"),
        (cashier_role_id, "Cashier", "Billing and payment access"),
        (
            waiter_role_id,
            "Waiter",
            "Order taking and table management",
        ),
        (
            kitchen_role_id,
            "Kitchen",
            "Kitchen display and KOT management",
        ),
    ] {
        roles::ActiveModel {
            id: Set(id),
            tenant_id: Set(tenant_id),
            name: Set(name.to_string()),
            description: Set(Some(desc.to_string())),
            is_system: Set(true),
            ..Default::default()
        }
        .insert(db)
        .await?;
    }
    tracing::info!("Created roles: Admin, Cashier, Waiter, Kitchen");

    // 5. Create users
    for (name, phone, pin, role_id) in [
        ("Rahul Sharma", "+919876543211", "1234", admin_role_id),
        ("Priya Patel", "+919876543212", "2345", cashier_role_id),
        ("Amit Kumar", "+919876543213", "3456", waiter_role_id),
        ("Suresh Reddy", "+919876543214", "4567", kitchen_role_id),
    ] {
        let user_id = Uuid::now_v7();
        users::ActiveModel {
            id: Set(user_id),
            tenant_id: Set(tenant_id),
            outlet_id: Set(Some(outlet_id)),
            name: Set(name.to_string()),
            phone: Set(phone.to_string()),
            pin: Set(pin.to_string()),
            is_active: Set(true),
            ..Default::default()
        }
        .insert(db)
        .await?;

        user_roles::ActiveModel {
            user_id: Set(user_id),
            role_id: Set(role_id),
            ..Default::default()
        }
        .insert(db)
        .await?;
    }
    tracing::info!(
        "Created users: Rahul (Admin), Priya (Cashier), Amit (Waiter), Suresh (Kitchen)"
    );

    // 6. Create menu categories and items
    #[allow(clippy::type_complexity)]
    let categories: &[(&str, &[(&str, f64, bool, f64)])] = &[
        (
            "Starters",
            &[
                ("Paneer Tikka", 280.0, true, 5.0),
                ("Chicken Tikka", 320.0, false, 5.0),
                ("Veg Spring Roll", 180.0, true, 5.0),
                ("Fish Amritsari", 350.0, false, 5.0),
            ],
        ),
        (
            "Main Course",
            &[
                ("Dal Makhani", 250.0, true, 5.0),
                ("Butter Chicken", 350.0, false, 5.0),
                ("Palak Paneer", 260.0, true, 5.0),
                ("Mutton Rogan Josh", 420.0, false, 5.0),
            ],
        ),
        (
            "Breads",
            &[
                ("Butter Naan", 60.0, true, 5.0),
                ("Garlic Naan", 80.0, true, 5.0),
                ("Tandoori Roti", 40.0, true, 5.0),
                ("Lachha Paratha", 70.0, true, 5.0),
            ],
        ),
        (
            "Rice & Biryani",
            &[
                ("Steamed Rice", 120.0, true, 5.0),
                ("Veg Biryani", 220.0, true, 5.0),
                ("Chicken Biryani", 280.0, false, 5.0),
                ("Jeera Rice", 150.0, true, 5.0),
            ],
        ),
        (
            "Beverages",
            &[
                ("Masala Chai", 40.0, true, 12.0),
                ("Fresh Lime Soda", 80.0, true, 12.0),
                ("Mango Lassi", 100.0, true, 12.0),
                ("Cold Coffee", 120.0, true, 12.0),
            ],
        ),
    ];

    for (idx, (cat_name, items)) in categories.iter().enumerate() {
        let cat_id = Uuid::now_v7();
        menu_categories::ActiveModel {
            id: Set(cat_id),
            outlet_id: Set(outlet_id),
            name: Set(cat_name.to_string()),
            sort_order: Set(idx as i32),
            is_active: Set(true),
            ..Default::default()
        }
        .insert(db)
        .await?;

        for (item_idx, (name, price, is_veg, gst)) in items.iter().enumerate() {
            let item_id = Uuid::now_v7();
            menu_items::ActiveModel {
                id: Set(item_id),
                category_id: Set(cat_id),
                outlet_id: Set(outlet_id),
                name: Set(name.to_string()),
                base_price: Set(rust_decimal::Decimal::from_f64_retain(*price).unwrap()),
                gst_percent: Set(rust_decimal::Decimal::from_f64_retain(*gst).unwrap()),
                is_veg: Set(*is_veg),
                is_available: Set(true),
                is_active: Set(true),
                sort_order: Set(item_idx as i32),
                ..Default::default()
            }
            .insert(db)
            .await?;

            // Add variants for select items (half/full portions, sizes)
            let variant_defs: &[(&str, f64)] = match *name {
                "Paneer Tikka" => &[("Half", 160.0), ("Full", 280.0)],
                "Chicken Tikka" => &[("Half", 180.0), ("Full", 320.0)],
                "Dal Makhani" => &[("Half", 150.0), ("Full", 250.0)],
                "Butter Chicken" => &[("Half", 200.0), ("Full", 350.0)],
                "Veg Biryani" => &[("Half", 140.0), ("Full", 220.0)],
                "Chicken Biryani" => &[("Half", 180.0), ("Full", 280.0)],
                "Mango Lassi" => &[("Regular", 100.0), ("Large", 150.0)],
                "Cold Coffee" => &[("Regular", 120.0), ("Large", 170.0)],
                _ => &[],
            };

            for (v_idx, (v_name, v_price)) in variant_defs.iter().enumerate() {
                menu_variants::ActiveModel {
                    id: Set(Uuid::now_v7()),
                    menu_item_id: Set(item_id),
                    name: Set(v_name.to_string()),
                    price: Set(rust_decimal::Decimal::from_f64_retain(*v_price).unwrap()),
                    is_active: Set(true),
                    sort_order: Set(v_idx as i32),
                }
                .insert(db)
                .await?;
            }
        }
    }
    tracing::info!("Created 5 menu categories with 20 items and variants");

    // 7. Create floors and tables
    let floor1_id = Uuid::now_v7();
    let floor2_id = Uuid::now_v7();

    floors::ActiveModel {
        id: Set(floor1_id),
        outlet_id: Set(outlet_id),
        name: Set("Ground Floor".to_string()),
        sort_order: Set(0),
        is_active: Set(true),
        ..Default::default()
    }
    .insert(db)
    .await?;

    floors::ActiveModel {
        id: Set(floor2_id),
        outlet_id: Set(outlet_id),
        name: Set("First Floor".to_string()),
        sort_order: Set(1),
        is_active: Set(true),
        ..Default::default()
    }
    .insert(db)
    .await?;

    // 6 tables on ground floor, 4 on first floor
    for i in 1..=6 {
        restaurant_tables::ActiveModel {
            id: Set(Uuid::now_v7()),
            floor_id: Set(floor1_id),
            outlet_id: Set(outlet_id),
            number: Set(format!("G{}", i)),
            capacity: Set(if i <= 4 { 4 } else { 6 }),
            status: Set("available".to_string()),
            is_active: Set(true),
            ..Default::default()
        }
        .insert(db)
        .await?;
    }

    for i in 1..=4 {
        restaurant_tables::ActiveModel {
            id: Set(Uuid::now_v7()),
            floor_id: Set(floor2_id),
            outlet_id: Set(outlet_id),
            number: Set(format!("F{}", i)),
            capacity: Set(if i <= 2 { 4 } else { 8 }),
            status: Set("available".to_string()),
            is_active: Set(true),
            ..Default::default()
        }
        .insert(db)
        .await?;
    }
    tracing::info!("Created 2 floors with 10 tables");

    Ok(())
}
