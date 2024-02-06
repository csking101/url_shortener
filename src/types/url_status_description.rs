use serde::{Serialize,Deserialize};

// This struct is used for storing in the DB (this is the value of the global hashmap's key)
#[derive(Debug,Serialize, Deserialize)]
pub struct URLStatusDescription {
    pub long_url: String,
    pub rate_limit: Option<u64>,
    pub permission_rule: Option<String>,
}
