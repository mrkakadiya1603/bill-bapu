use sea_orm_migration::prelude::*;

use super::m20250101_000002_create_outlets::Outlets;
use super::m20250101_000004_create_menu::MenuItems;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Raw materials
        manager
            .create_table(
                Table::create()
                    .table(RawMaterials::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RawMaterials::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(RawMaterials::OutletId).uuid().not_null())
                    .col(
                        ColumnDef::new(RawMaterials::Name)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(ColumnDef::new(RawMaterials::Sku).string_len(50).null())
                    .col(ColumnDef::new(RawMaterials::Unit).string_len(20).not_null())
                    .col(
                        ColumnDef::new(RawMaterials::CurrentStock)
                            .decimal_len(12, 3)
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(RawMaterials::MinStock)
                            .decimal_len(12, 3)
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(RawMaterials::CostPerUnit)
                            .decimal_len(10, 2)
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(RawMaterials::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(RawMaterials::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(RawMaterials::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(RawMaterials::Table, RawMaterials::OutletId)
                            .to(Outlets::Table, Outlets::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Recipes (links menu items to raw materials)
        manager
            .create_table(
                Table::create()
                    .table(Recipes::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Recipes::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Recipes::MenuItemId).uuid().not_null())
                    .col(ColumnDef::new(Recipes::Name).string_len(255).not_null())
                    .col(
                        ColumnDef::new(Recipes::Yield)
                            .decimal_len(10, 3)
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(Recipes::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Recipes::Table, Recipes::MenuItemId)
                            .to(MenuItems::Table, MenuItems::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Recipe ingredients
        manager
            .create_table(
                Table::create()
                    .table(RecipeIngredients::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RecipeIngredients::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(RecipeIngredients::RecipeId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RecipeIngredients::RawMaterialId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RecipeIngredients::Quantity)
                            .decimal_len(10, 3)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RecipeIngredients::Unit)
                            .string_len(20)
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(RecipeIngredients::Table, RecipeIngredients::RecipeId)
                            .to(Recipes::Table, Recipes::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(RecipeIngredients::Table, RecipeIngredients::RawMaterialId)
                            .to(RawMaterials::Table, RawMaterials::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Stock movements
        manager
            .create_table(
                Table::create()
                    .table(StockMovements::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(StockMovements::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(StockMovements::RawMaterialId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(StockMovements::MovementType)
                            .string_len(20)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(StockMovements::Quantity)
                            .decimal_len(12, 3)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(StockMovements::UnitCost)
                            .decimal_len(10, 2)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(StockMovements::Reference)
                            .string_len(100)
                            .null(),
                    )
                    .col(ColumnDef::new(StockMovements::Notes).text().null())
                    .col(ColumnDef::new(StockMovements::CreatedBy).uuid().null())
                    .col(
                        ColumnDef::new(StockMovements::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(StockMovements::Table, StockMovements::RawMaterialId)
                            .to(RawMaterials::Table, RawMaterials::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Vendors
        manager
            .create_table(
                Table::create()
                    .table(Vendors::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Vendors::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Vendors::OutletId).uuid().not_null())
                    .col(ColumnDef::new(Vendors::Name).string_len(255).not_null())
                    .col(ColumnDef::new(Vendors::ContactName).string_len(255).null())
                    .col(ColumnDef::new(Vendors::Phone).string_len(15).null())
                    .col(ColumnDef::new(Vendors::Email).string_len(255).null())
                    .col(ColumnDef::new(Vendors::Address).text().null())
                    .col(ColumnDef::new(Vendors::GstNumber).string_len(15).null())
                    .col(
                        ColumnDef::new(Vendors::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(Vendors::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Vendors::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Vendors::Table, Vendors::OutletId)
                            .to(Outlets::Table, Outlets::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Purchase orders
        manager
            .create_table(
                Table::create()
                    .table(PurchaseOrders::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PurchaseOrders::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PurchaseOrders::OutletId).uuid().not_null())
                    .col(ColumnDef::new(PurchaseOrders::VendorId).uuid().not_null())
                    .col(
                        ColumnDef::new(PurchaseOrders::PoNumber)
                            .string_len(30)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PurchaseOrders::Status)
                            .string_len(20)
                            .not_null()
                            .default("draft"),
                    )
                    .col(
                        ColumnDef::new(PurchaseOrders::TotalAmount)
                            .decimal_len(12, 2)
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(PurchaseOrders::Notes).text().null())
                    .col(ColumnDef::new(PurchaseOrders::ExpectedDate).date().null())
                    .col(ColumnDef::new(PurchaseOrders::ReceivedDate).date().null())
                    .col(ColumnDef::new(PurchaseOrders::CreatedBy).uuid().null())
                    .col(
                        ColumnDef::new(PurchaseOrders::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(PurchaseOrders::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(PurchaseOrders::Table, PurchaseOrders::OutletId)
                            .to(Outlets::Table, Outlets::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(PurchaseOrders::Table, PurchaseOrders::VendorId)
                            .to(Vendors::Table, Vendors::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Purchase order items
        manager
            .create_table(
                Table::create()
                    .table(PurchaseOrderItems::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PurchaseOrderItems::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PurchaseOrderItems::PurchaseOrderId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PurchaseOrderItems::RawMaterialId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PurchaseOrderItems::Quantity)
                            .decimal_len(12, 3)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PurchaseOrderItems::UnitCost)
                            .decimal_len(10, 2)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PurchaseOrderItems::TotalCost)
                            .decimal_len(12, 2)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PurchaseOrderItems::ReceivedQuantity)
                            .decimal_len(12, 3)
                            .not_null()
                            .default(0),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                PurchaseOrderItems::Table,
                                PurchaseOrderItems::PurchaseOrderId,
                            )
                            .to(PurchaseOrders::Table, PurchaseOrders::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(PurchaseOrderItems::Table, PurchaseOrderItems::RawMaterialId)
                            .to(RawMaterials::Table, RawMaterials::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PurchaseOrderItems::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(PurchaseOrders::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Vendors::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(StockMovements::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(RecipeIngredients::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Recipes::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(RawMaterials::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum RawMaterials {
    Table,
    Id,
    OutletId,
    Name,
    Sku,
    Unit,
    CurrentStock,
    MinStock,
    CostPerUnit,
    IsActive,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Recipes {
    Table,
    Id,
    MenuItemId,
    Name,
    Yield,
    CreatedAt,
}

#[derive(DeriveIden)]
enum RecipeIngredients {
    Table,
    Id,
    RecipeId,
    RawMaterialId,
    Quantity,
    Unit,
}

#[derive(DeriveIden)]
enum StockMovements {
    Table,
    Id,
    RawMaterialId,
    MovementType,
    Quantity,
    UnitCost,
    Reference,
    Notes,
    CreatedBy,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Vendors {
    Table,
    Id,
    OutletId,
    Name,
    ContactName,
    Phone,
    Email,
    Address,
    GstNumber,
    IsActive,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum PurchaseOrders {
    Table,
    Id,
    OutletId,
    VendorId,
    PoNumber,
    Status,
    TotalAmount,
    Notes,
    ExpectedDate,
    ReceivedDate,
    CreatedBy,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum PurchaseOrderItems {
    Table,
    Id,
    PurchaseOrderId,
    RawMaterialId,
    Quantity,
    UnitCost,
    TotalCost,
    ReceivedQuantity,
}
