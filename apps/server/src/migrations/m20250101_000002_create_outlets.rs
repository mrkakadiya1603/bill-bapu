use sea_orm_migration::prelude::*;

use super::m20250101_000001_create_tenants_and_plans::Tenants;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Outlets::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Outlets::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Outlets::TenantId).uuid().not_null())
                    .col(ColumnDef::new(Outlets::Name).string_len(255).not_null())
                    .col(ColumnDef::new(Outlets::Code).string_len(20).not_null())
                    .col(ColumnDef::new(Outlets::Address).text().null())
                    .col(ColumnDef::new(Outlets::City).string_len(100).null())
                    .col(ColumnDef::new(Outlets::State).string_len(100).null())
                    .col(ColumnDef::new(Outlets::Pincode).string_len(6).null())
                    .col(ColumnDef::new(Outlets::Phone).string_len(15).null())
                    .col(ColumnDef::new(Outlets::Email).string_len(255).null())
                    .col(ColumnDef::new(Outlets::GstNumber).string_len(15).null())
                    .col(ColumnDef::new(Outlets::FssaiNumber).string_len(14).null())
                    .col(
                        ColumnDef::new(Outlets::TimezoneOffset)
                            .string_len(10)
                            .not_null()
                            .default("+05:30"),
                    )
                    .col(
                        ColumnDef::new(Outlets::CurrencyCode)
                            .string_len(3)
                            .not_null()
                            .default("INR"),
                    )
                    .col(
                        ColumnDef::new(Outlets::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(Outlets::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Outlets::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Outlets::Table, Outlets::TenantId)
                            .to(Tenants::Table, Tenants::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Unique constraint on tenant_id + code
        manager
            .create_index(
                Index::create()
                    .name("idx_outlets_tenant_code")
                    .table(Outlets::Table)
                    .col(Outlets::TenantId)
                    .col(Outlets::Code)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Outlets::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Outlets {
    Table,
    Id,
    TenantId,
    Name,
    Code,
    Address,
    City,
    State,
    Pincode,
    Phone,
    Email,
    GstNumber,
    FssaiNumber,
    TimezoneOffset,
    CurrencyCode,
    IsActive,
    CreatedAt,
    UpdatedAt,
}
