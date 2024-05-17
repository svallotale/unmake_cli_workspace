use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Github {
    pub remote_path: String,
}
