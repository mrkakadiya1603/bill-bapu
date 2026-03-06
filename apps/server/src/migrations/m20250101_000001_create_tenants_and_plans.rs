use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Plans table
        manager
            .create_table(
                Table::create()
                    .table(Plans::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Plans::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Plans::Name).string_len(100).not_null())
                    .col(
                        ColumnDef::new(Plans::MaxOutlets)
                            .integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(Plans::MaxUsers)
                            .integer()
                            .not_null()
                            .default(5),
                    )
                    .col(
                        ColumnDef::new(Plans::Features)
                            .json_binary()
                            .not_null()
                            .default("{}"),
                    )
                    .col(
                        ColumnDef::new(Plans::PriceMonthly)
                            .decimal_len(10, 2)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Plans::PriceYearly)
                            .decimal_len(10, 2)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Plans::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(Plans::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Plans::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // Tenants table
        manager
            .create_table(
                Table::create()
                    .table(Tenants::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Tenants::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Tenants::Name).string_len(255).not_null())
                    .col(
                        ColumnDef::new(Tenants::Slug)
                            .string_len(100)
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Tenants::PlanId).uuid().not_null())
                    .col(ColumnDef::new(Tenants::GstNumber).string_len(15).null())
                    .col(ColumnDef::new(Tenants::FssaiNumber).string_len(14).null())
                    .col(
                        ColumnDef::new(Tenants::ContactEmail)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Tenants::ContactPhone)
                            .string_len(15)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Tenants::Address).text().null())
                    .col(ColumnDef::new(Tenants::City).string_len(100).null())
                    .col(ColumnDef::new(Tenants::State).string_len(100).null())
                    .col(ColumnDef::new(Tenants::Pincode).string_len(6).null())
                    .col(
                        ColumnDef::new(Tenants::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(Tenants::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Tenants::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Tenants::Table, Tenants::PlanId)
                            .to(Plans::Table, Plans::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Tenants::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Plans::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Plans {
    Table,
    Id,
    Name,
    MaxOutlets,
    MaxUsers,
    Features,
    PriceMonthly,
    PriceYearly,
    IsActive,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum Tenants {
    Table,
    Id,
    Name,
    Slug,
    PlanId,
    GstNumber,
    FssaiNumber,
    ContactEmail,
    ContactPhone,
    Address,
    City,
    State,
    Pincode,
    IsActive,
    CreatedAt,
    UpdatedAt,
}
