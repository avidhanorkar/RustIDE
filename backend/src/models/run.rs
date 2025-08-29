use tokio_postgres::Row;
use uuid::Uuid;
use chrono::{NaiveDateTime};

#[derive(Debug, Clone)]
pub struct Run {
    pub id: Uuid,
    pub project_id: Uuid,
    pub language: String,
    pub status: String,
    pub output: String,
    pub created_at: NaiveDateTime
}

impl Run {
    pub fn from_row(row: &Row) -> Self {
        Run {
            id: row.get("id"),
            project_id: row.get("project_id"),
            language: row.get("language"),
            status: row.get("status"),
            output: row.get("output"),
            created_at: row.get("created_at"),
        }
    }
}