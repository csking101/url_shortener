use serde::{Deserialize, Serialize};

// This struct is used for making the request
#[derive(Debug, Serialize, Deserialize)]
pub struct URLCreationDescription {
    pub long_url: String,
    pub rate_limit: Option<u64>,
    pub permission_rule: Option<String>,
}
