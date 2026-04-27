use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VaultEntry {
    pub id: Uuid,
    pub title: String,
    pub username: String,
    pub password: String,
    pub website: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl VaultEntry {
    pub fn new(
        title: String,
        username: String,
        password: String,
        website: Option<String>,
        notes: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            username,
            password,
            website,
            notes,
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Vault {
    pub entries: Vec<VaultEntry>,
}