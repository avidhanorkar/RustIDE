use uuid::Uuid;
use tokio_postgres::Row;
use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct Project {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub name: String,
    pub created_at: NaiveDateTime
}

impl Project {
    pub fn from_row(row: &Row) -> Self {
        Project {
            id: row.get("id"),
            owner_id: row.get("owner_id"),
            name: row.get("name"),
            created_at: row.get("created_at"),
        }
    }
}