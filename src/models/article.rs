use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub id: String,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Article {
    pub fn new(title: String, content: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            content,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update(&mut self, title: Option<String>, content: Option<String>) {
        if let Some(title) = title {
            self.title = title;
        }
        if let Some(content) = content {
            self.content = content;
        }
        self.updated_at = Utc::now();
    }
}
