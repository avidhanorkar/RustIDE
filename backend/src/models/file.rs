use tokio_postgres::Row;
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct File {
    pub id: Uuid,
    pub project_id: Uuid,
    pub path: String,
    pub content: String,
    pub created_at: NaiveDateTime
}

impl File {
    pub fn from_row(row: &Row) -> Self {
        File {
            id: row.get("id"),
            project_id: row.get("project_id"),
            path: row.get("path"),
            content: row.get("content"),
            created_at: row.get("created_at"),
        }
    }
}