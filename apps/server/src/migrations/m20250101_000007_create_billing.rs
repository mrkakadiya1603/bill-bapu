use sea_orm_migration::prelude::*;

use super::m20250101_000002_create_outlets::Outlets;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Bills
        manager
            .create_table(
                Table::create()
                    .table(Bills::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Bills::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Bills::OutletId).uuid().not_null())
                    .col(ColumnDef::new(Bills::OrderId).uuid().not_null())
                    .col(ColumnDef::new(Bills::BillNumber).string_len(30).not_null())
                    .col(
                        ColumnDef::new(Bills::Subtotal)
                            .decimal_len(12, 2)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Bills::CgstAmount)
                            .decimal_len(12, 2)
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Bills::SgstAmount)
                            .decimal_len(12, 2)
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Bills::DiscountAmount)
                            .decimal_len(12, 2)
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Bills::RoundOff)
                            .decimal_len(5, 2)
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Bills::GrandTotal)
                            .decimal_len(12, 2)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Bills::PaymentStatus)
                            .string_len(20)
                            .not_null()
                            .default("unpaid"),
                    )
                    .col(
                        ColumnDef::new(Bills::IsCancelled)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Bills::CancelReason).text().null())
                    .col(ColumnDef::new(Bills::CreatedBy).uuid().null())
                    .col(
                        ColumnDef::new(Bills::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Bills::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Bills::Table, Bills::OutletId)
                            .to(Outlets::Table, Outlets::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_bills_outlet_number")
                    .table(Bills::Table)
                    .col(Bills::OutletId)
                    .col(Bills::BillNumber)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // Bill items
        manager
            .create_table(
                Table::create()
                    .table(BillItems::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(BillItems::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(BillItems::BillId).uuid().not_null())
                    .col(ColumnDef::new(BillItems::MenuItemId).uuid().not_null())
                    .col(ColumnDef::new(BillItems::Name).string_len(255).not_null())
                    .col(ColumnDef::new(BillItems::Quantity).integer().not_null())
                    .col(
                        ColumnDef::new(BillItems::UnitPrice)
                            .decimal_len(10, 2)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BillItems::GstPercent)
                            .decimal_len(5, 2)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BillItems::TaxAmount)
                            .decimal_len(10, 2)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BillItems::TotalPrice)
                            .decimal_len(10, 2)
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(BillItems::Table, BillItems::BillId)
                            .to(Bills::Table, Bills::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Payments
        manager
            .create_table(
                Table::create()
                    .table(Payments::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Payments::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Payments::BillId).uuid().not_null())
                    .col(
                        ColumnDef::new(Payments::Amount)
                            .decimal_len(12, 2)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Payments::Mode).string_len(20).not_null())
                    .col(
                        ColumnDef::new(Payments::ReferenceNumber)
                            .string_len(100)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Payments::Status)
                            .string_len(20)
                            .not_null()
                            .default("completed"),
                    )
                    .col(
                        ColumnDef::new(Payments::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Payments::Table, Payments::BillId)
                            .to(Bills::Table, Bills::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Discounts
        manager
            .create_table(
                Table::create()
                    .table(Discounts::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Discounts::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Discounts::OutletId).uuid().not_null())
                    .col(ColumnDef::new(Discounts::Name).string_len(100).not_null())
                    .col(
                        ColumnDef::new(Discounts::DiscountType)
                            .string_len(20)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Discounts::Value)
                            .decimal_len(10, 2)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Discounts::MaxAmount)
                            .decimal_len(10, 2)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Discounts::MinOrderValue)
                            .decimal_len(10, 2)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Discounts::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(Discounts::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Discounts::Table, Discounts::OutletId)
                            .to(Outlets::Table, Outlets::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Discounts::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Payments::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(BillItems::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Bills::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Bills {
    Table,
    Id,
    OutletId,
    OrderId,
    BillNumber,
    Subtotal,
    CgstAmount,
    SgstAmount,
    DiscountAmount,
    RoundOff,
    GrandTotal,
    PaymentStatus,
    IsCancelled,
    CancelReason,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum BillItems {
    Table,
    Id,
    BillId,
    MenuItemId,
    Name,
    Quantity,
    UnitPrice,
    GstPercent,
    TaxAmount,
    TotalPrice,
}

#[derive(DeriveIden)]
enum Payments {
    Table,
    Id,
    BillId,
    Amount,
    Mode,
    ReferenceNumber,
    Status,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Discounts {
    Table,
    Id,
    OutletId,
    Name,
    DiscountType,
    Value,
    MaxAmount,
    MinOrderValue,
    IsActive,
    CreatedAt,
}
