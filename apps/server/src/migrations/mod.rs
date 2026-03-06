use sea_orm_migration::prelude::*;

mod m20250101_000001_create_tenants_and_plans;
mod m20250101_000002_create_outlets;
mod m20250101_000003_create_users_and_roles;
mod m20250101_000004_create_menu;
mod m20250101_000005_create_tables_and_floors;
mod m20250101_000006_create_orders;
mod m20250101_000007_create_billing;
mod m20250101_000008_create_inventory;
mod m20250101_000009_create_report_indexes;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250101_000001_create_tenants_and_plans::Migration),
            Box::new(m20250101_000002_create_outlets::Migration),
            Box::new(m20250101_000003_create_users_and_roles::Migration),
            Box::new(m20250101_000004_create_menu::Migration),
            Box::new(m20250101_000005_create_tables_and_floors::Migration),
            Box::new(m20250101_000006_create_orders::Migration),
            Box::new(m20250101_000007_create_billing::Migration),
            Box::new(m20250101_000008_create_inventory::Migration),
            Box::new(m20250101_000009_create_report_indexes::Migration),
        ]
    }
}
