use sea_orm_migration::prelude::*;

use super::m20250101_000002_create_outlets::Outlets;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Menu categories
        manager
            .create_table(
                Table::create()
                    .table(MenuCategories::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MenuCategories::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(MenuCategories::OutletId).uuid().not_null())
                    .col(ColumnDef::new(MenuCategories::ParentId).uuid().null())
                    .col(
                        ColumnDef::new(MenuCategories::Name)
                            .string_len(100)
                            .not_null(),
                    )
                    .col(ColumnDef::new(MenuCategories::Description).text().null())
                    .col(
                        ColumnDef::new(MenuCategories::SortOrder)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(MenuCategories::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(MenuCategories::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(MenuCategories::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(MenuCategories::Table, MenuCategories::OutletId)
                            .to(Outlets::Table, Outlets::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(MenuCategories::Table, MenuCategories::ParentId)
                            .to(MenuCategories::Table, MenuCategories::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        // Menu items
        manager
            .create_table(
                Table::create()
                    .table(MenuItems::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MenuItems::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(MenuItems::CategoryId).uuid().not_null())
                    .col(ColumnDef::new(MenuItems::OutletId).uuid().not_null())
                    .col(ColumnDef::new(MenuItems::Name).string_len(255).not_null())
                    .col(ColumnDef::new(MenuItems::Description).text().null())
                    .col(ColumnDef::new(MenuItems::ShortCode).string_len(20).null())
                    .col(
                        ColumnDef::new(MenuItems::BasePrice)
                            .decimal_len(10, 2)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(MenuItems::GstPercent)
                            .decimal_len(5, 2)
                            .not_null()
                            .default(5.0),
                    )
                    .col(
                        ColumnDef::new(MenuItems::IsVeg)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(MenuItems::IsAvailable)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(MenuItems::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(MenuItems::SortOrder)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(MenuItems::ImageUrl).text().null())
                    .col(ColumnDef::new(MenuItems::PreparationTime).integer().null())
                    .col(
                        ColumnDef::new(MenuItems::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(MenuItems::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(MenuItems::Table, MenuItems::CategoryId)
                            .to(MenuCategories::Table, MenuCategories::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(MenuItems::Table, MenuItems::OutletId)
                            .to(Outlets::Table, Outlets::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Menu variants
        manager
            .create_table(
                Table::create()
                    .table(MenuVariants::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MenuVariants::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(MenuVariants::MenuItemId).uuid().not_null())
                    .col(
                        ColumnDef::new(MenuVariants::Name)
                            .string_len(100)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(MenuVariants::Price)
                            .decimal_len(10, 2)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(MenuVariants::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(MenuVariants::SortOrder)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(MenuVariants::Table, MenuVariants::MenuItemId)
                            .to(MenuItems::Table, MenuItems::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Modifier groups
        manager
            .create_table(
                Table::create()
                    .table(ModifierGroups::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ModifierGroups::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ModifierGroups::OutletId).uuid().not_null())
                    .col(
                        ColumnDef::new(ModifierGroups::Name)
                            .string_len(100)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ModifierGroups::MinSelection)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(ModifierGroups::MaxSelection)
                            .integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(ModifierGroups::IsRequired)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(ModifierGroups::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(ModifierGroups::Table, ModifierGroups::OutletId)
                            .to(Outlets::Table, Outlets::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Modifiers
        manager
            .create_table(
                Table::create()
                    .table(Modifiers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Modifiers::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Modifiers::GroupId).uuid().not_null())
                    .col(ColumnDef::new(Modifiers::Name).string_len(100).not_null())
                    .col(
                        ColumnDef::new(Modifiers::Price)
                            .decimal_len(10, 2)
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Modifiers::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(Modifiers::SortOrder)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Modifiers::Table, Modifiers::GroupId)
                            .to(ModifierGroups::Table, ModifierGroups::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Menu item ↔ modifier group junction
        manager
            .create_table(
                Table::create()
                    .table(MenuItemModifierGroups::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MenuItemModifierGroups::MenuItemId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(MenuItemModifierGroups::ModifierGroupId)
                            .uuid()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(MenuItemModifierGroups::MenuItemId)
                            .col(MenuItemModifierGroups::ModifierGroupId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                MenuItemModifierGroups::Table,
                                MenuItemModifierGroups::MenuItemId,
                            )
                            .to(MenuItems::Table, MenuItems::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                MenuItemModifierGroups::Table,
                                MenuItemModifierGroups::ModifierGroupId,
                            )
                            .to(ModifierGroups::Table, ModifierGroups::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(MenuItemModifierGroups::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(Modifiers::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(ModifierGroups::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(MenuVariants::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(MenuItems::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(MenuCategories::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum MenuCategories {
    Table,
    Id,
    OutletId,
    ParentId,
    Name,
    Description,
    SortOrder,
    IsActive,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
pub enum MenuItems {
    Table,
    Id,
    CategoryId,
    OutletId,
    Name,
    Description,
    ShortCode,
    BasePrice,
    GstPercent,
    IsVeg,
    IsAvailable,
    IsActive,
    SortOrder,
    ImageUrl,
    PreparationTime,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum MenuVariants {
    Table,
    Id,
    MenuItemId,
    Name,
    Price,
    IsActive,
    SortOrder,
}

#[derive(DeriveIden)]
enum ModifierGroups {
    Table,
    Id,
    OutletId,
    Name,
    MinSelection,
    MaxSelection,
    IsRequired,
    IsActive,
}

#[derive(DeriveIden)]
enum Modifiers {
    Table,
    Id,
    GroupId,
    Name,
    Price,
    IsActive,
    SortOrder,
}

#[derive(DeriveIden)]
enum MenuItemModifierGroups {
    Table,
    MenuItemId,
    ModifierGroupId,
}
