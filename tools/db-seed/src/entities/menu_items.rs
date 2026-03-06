use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "menu_items")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub category_id: Uuid,
    pub outlet_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub short_code: Option<String>,
    pub base_price: Decimal,
    pub gst_percent: Decimal,
    pub is_veg: bool,
    pub is_available: bool,
    pub is_active: bool,
    pub sort_order: i32,
    pub image_url: Option<String>,
    pub preparation_time: Option<i32>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
