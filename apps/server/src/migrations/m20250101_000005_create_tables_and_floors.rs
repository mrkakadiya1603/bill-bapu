use sea_orm_migration::prelude::*;

use super::m20250101_000002_create_outlets::Outlets;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Floors
        manager
            .create_table(
                Table::create()
                    .table(Floors::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Floors::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Floors::OutletId).uuid().not_null())
                    .col(ColumnDef::new(Floors::Name).string_len(100).not_null())
                    .col(
                        ColumnDef::new(Floors::SortOrder)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Floors::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(Floors::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Floors::Table, Floors::OutletId)
                            .to(Outlets::Table, Outlets::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Restaurant tables
        manager
            .create_table(
                Table::create()
                    .table(RestaurantTables::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RestaurantTables::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(RestaurantTables::FloorId).uuid().not_null())
                    .col(ColumnDef::new(RestaurantTables::OutletId).uuid().not_null())
                    .col(
                        ColumnDef::new(RestaurantTables::Number)
                            .string_len(20)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RestaurantTables::Capacity)
                            .integer()
                            .not_null()
                            .default(4),
                    )
                    .col(
                        ColumnDef::new(RestaurantTables::Status)
                            .string_len(20)
                            .not_null()
                            .default("available"),
                    )
                    .col(
                        ColumnDef::new(RestaurantTables::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(RestaurantTables::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(RestaurantTables::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(RestaurantTables::Table, RestaurantTables::FloorId)
                            .to(Floors::Table, Floors::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(RestaurantTables::Table, RestaurantTables::OutletId)
                            .to(Outlets::Table, Outlets::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_tables_outlet_number")
                    .table(RestaurantTables::Table)
                    .col(RestaurantTables::OutletId)
                    .col(RestaurantTables::Number)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // Reservations
        manager
            .create_table(
                Table::create()
                    .table(Reservations::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Reservations::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Reservations::TableId).uuid().not_null())
                    .col(ColumnDef::new(Reservations::OutletId).uuid().not_null())
                    .col(
                        ColumnDef::new(Reservations::CustomerName)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Reservations::CustomerPhone)
                            .string_len(15)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Reservations::GuestCount)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Reservations::ReservedFor)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Reservations::Status)
                            .string_len(20)
                            .not_null()
                            .default("confirmed"),
                    )
                    .col(ColumnDef::new(Reservations::Notes).text().null())
                    .col(
                        ColumnDef::new(Reservations::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Reservations::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Reservations::Table, Reservations::TableId)
                            .to(RestaurantTables::Table, RestaurantTables::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Reservations::Table, Reservations::OutletId)
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
            .drop_table(Table::drop().table(Reservations::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(RestaurantTables::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Floors::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Floors {
    Table,
    Id,
    OutletId,
    Name,
    SortOrder,
    IsActive,
    CreatedAt,
}

#[derive(DeriveIden)]
enum RestaurantTables {
    Table,
    Id,
    FloorId,
    OutletId,
    Number,
    Capacity,
    Status,
    IsActive,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Reservations {
    Table,
    Id,
    TableId,
    OutletId,
    CustomerName,
    CustomerPhone,
    GuestCount,
    ReservedFor,
    Status,
    Notes,
    CreatedAt,
    UpdatedAt,
}
