use chrono::NaiveDateTime;
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}

impl User {
    pub fn from_row(row: &Row) -> Self {
        User {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email"),
            password: row.get("password"),
            created_at: row.get("created_at"),
        }
    }
}
