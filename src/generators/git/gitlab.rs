use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Gitlab {
    pub remote_path: String,
}
