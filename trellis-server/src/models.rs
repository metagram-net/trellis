use super::schema::settings;
use chrono::NaiveDateTime;
use serde_json::Value as Jsonb;
use uuid::Uuid;

#[derive(Insertable)]
#[table_name = "settings"]
pub struct NewSettings {
    pub data: Jsonb,
    pub user_id: String,
}

#[derive(Queryable)]
pub struct Settings {
    pub id: Uuid,
    pub data: Jsonb,
    pub user_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
