use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Status {
    pub status: String,
}

impl Status {
    pub fn new(given_status: &str) -> Self {
        Self {
            status: given_status.to_string(),
        }
    }
}
