use sea_orm_migration::prelude::*;

use super::m20250101_000002_create_outlets::Outlets;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Orders
        manager
            .create_table(
                Table::create()
                    .table(Orders::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Orders::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Orders::OutletId).uuid().not_null())
                    .col(
                        ColumnDef::new(Orders::OrderNumber)
                            .string_len(20)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Orders::OrderType).string_len(20).not_null())
                    .col(
                        ColumnDef::new(Orders::Status)
                            .string_len(20)
                            .not_null()
                            .default("pending"),
                    )
                    .col(ColumnDef::new(Orders::TableId).uuid().null())
                    .col(ColumnDef::new(Orders::CustomerId).uuid().null())
                    .col(ColumnDef::new(Orders::CustomerName).string_len(255).null())
                    .col(ColumnDef::new(Orders::CustomerPhone).string_len(15).null())
                    .col(ColumnDef::new(Orders::GuestCount).integer().null())
                    .col(
                        ColumnDef::new(Orders::Subtotal)
                            .decimal_len(12, 2)
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Orders::TaxTotal)
                            .decimal_len(12, 2)
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Orders::DiscountTotal)
                            .decimal_len(12, 2)
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Orders::GrandTotal)
                            .decimal_len(12, 2)
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(Orders::Notes).text().null())
                    .col(ColumnDef::new(Orders::CreatedBy).uuid().null())
                    .col(
                        ColumnDef::new(Orders::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Orders::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Orders::Table, Orders::OutletId)
                            .to(Outlets::Table, Outlets::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_orders_outlet_number")
                    .table(Orders::Table)
                    .col(Orders::OutletId)
                    .col(Orders::OrderNumber)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // Order items
        manager
            .create_table(
                Table::create()
                    .table(OrderItems::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OrderItems::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(OrderItems::OrderId).uuid().not_null())
                    .col(ColumnDef::new(OrderItems::MenuItemId).uuid().not_null())
                    .col(ColumnDef::new(OrderItems::VariantId).uuid().null())
                    .col(ColumnDef::new(OrderItems::Name).string_len(255).not_null())
                    .col(ColumnDef::new(OrderItems::Quantity).integer().not_null())
                    .col(
                        ColumnDef::new(OrderItems::UnitPrice)
                            .decimal_len(10, 2)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OrderItems::TaxAmount)
                            .decimal_len(10, 2)
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(OrderItems::TotalPrice)
                            .decimal_len(10, 2)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OrderItems::ModifiersJson)
                            .json_binary()
                            .null(),
                    )
                    .col(ColumnDef::new(OrderItems::Notes).text().null())
                    .col(
                        ColumnDef::new(OrderItems::Status)
                            .string_len(20)
                            .not_null()
                            .default("pending"),
                    )
                    .col(
                        ColumnDef::new(OrderItems::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(OrderItems::Table, OrderItems::OrderId)
                            .to(Orders::Table, Orders::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // KOTs (Kitchen Order Tickets)
        manager
            .create_table(
                Table::create()
                    .table(Kots::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Kots::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Kots::OrderId).uuid().not_null())
                    .col(ColumnDef::new(Kots::KotNumber).string_len(20).not_null())
                    .col(
                        ColumnDef::new(Kots::Status)
                            .string_len(20)
                            .not_null()
                            .default("pending"),
                    )
                    .col(ColumnDef::new(Kots::CreatedBy).uuid().null())
                    .col(
                        ColumnDef::new(Kots::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Kots::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Kots::Table, Kots::OrderId)
                            .to(Orders::Table, Orders::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // KOT items
        manager
            .create_table(
                Table::create()
                    .table(KotItems::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(KotItems::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(KotItems::KotId).uuid().not_null())
                    .col(ColumnDef::new(KotItems::OrderItemId).uuid().not_null())
                    .col(ColumnDef::new(KotItems::Quantity).integer().not_null())
                    .col(
                        ColumnDef::new(KotItems::Status)
                            .string_len(20)
                            .not_null()
                            .default("pending"),
                    )
                    .col(ColumnDef::new(KotItems::Notes).text().null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(KotItems::Table, KotItems::KotId)
                            .to(Kots::Table, Kots::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(KotItems::Table, KotItems::OrderItemId)
                            .to(OrderItems::Table, OrderItems::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(KotItems::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Kots::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(OrderItems::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Orders::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Orders {
    Table,
    Id,
    OutletId,
    OrderNumber,
    OrderType,
    Status,
    TableId,
    CustomerId,
    CustomerName,
    CustomerPhone,
    GuestCount,
    Subtotal,
    TaxTotal,
    DiscountTotal,
    GrandTotal,
    Notes,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum OrderItems {
    Table,
    Id,
    OrderId,
    MenuItemId,
    VariantId,
    Name,
    Quantity,
    UnitPrice,
    TaxAmount,
    TotalPrice,
    ModifiersJson,
    Notes,
    Status,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Kots {
    Table,
    Id,
    OrderId,
    KotNumber,
    Status,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum KotItems {
    Table,
    Id,
    KotId,
    OrderItemId,
    Quantity,
    Status,
    Notes,
}
