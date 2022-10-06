use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct PreferenceEntity {
    pub preference_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub status: bool
}


pub struct PreferenceLocaleEntity {
    pub locale_id: Uuid,
    pub description: String,
    pub locale: String,
    pub video_url: Option<String>,
    pub preference_id: Uuid,
}

pub struct NewPreferenceLocaleEntity {
    pub description: String,
    pub locale: String,
    pub video_url: Option<String>,
}