use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DataDocument {
    pub data: String,
    pub code: String,
    pub created_at: DateTime<Utc>,
}
