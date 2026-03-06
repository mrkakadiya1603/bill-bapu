use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Task 0.8.6: Report/analytics support indexes
        // These indexes optimize the most common report queries:
        // - Daily/weekly/monthly revenue reports (bills.created_at)
        // - Order volume reports (orders.created_at)
        // - Item-level sales analysis (bill_items.menu_item_id)

        // Index on bills.created_at for date-range revenue reports
        manager
            .create_index(
                Index::create()
                    .name("idx_bills_created_at")
                    .table(Bills::Table)
                    .col(Bills::CreatedAt)
                    .to_owned(),
            )
            .await?;

        // Index on orders.created_at for date-range order reports
        manager
            .create_index(
                Index::create()
                    .name("idx_orders_created_at")
                    .table(Orders::Table)
                    .col(Orders::CreatedAt)
                    .to_owned(),
            )
            .await?;

        // Index on bill_items.menu_item_id for item-level sales reports
        manager
            .create_index(
                Index::create()
                    .name("idx_bill_items_menu_item_id")
                    .table(BillItems::Table)
                    .col(BillItems::MenuItemId)
                    .to_owned(),
            )
            .await?;

        // Composite index on orders for outlet-scoped status filtering
        manager
            .create_index(
                Index::create()
                    .name("idx_orders_outlet_status")
                    .table(Orders::Table)
                    .col(Orders::OutletId)
                    .col(Orders::Status)
                    .to_owned(),
            )
            .await?;

        // Composite index on bills for outlet-scoped date reports
        manager
            .create_index(
                Index::create()
                    .name("idx_bills_outlet_created_at")
                    .table(Bills::Table)
                    .col(Bills::OutletId)
                    .col(Bills::CreatedAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_bills_outlet_created_at")
                    .table(Bills::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .name("idx_orders_outlet_status")
                    .table(Orders::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .name("idx_bill_items_menu_item_id")
                    .table(BillItems::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .name("idx_orders_created_at")
                    .table(Orders::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .name("idx_bills_created_at")
                    .table(Bills::Table)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}

// Reference the table/column identifiers from existing migrations.
// We only need the columns referenced by the indexes.

#[derive(DeriveIden)]
enum Bills {
    Table,
    OutletId,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Orders {
    Table,
    OutletId,
    Status,
    CreatedAt,
}

#[derive(DeriveIden)]
enum BillItems {
    Table,
    MenuItemId,
}
