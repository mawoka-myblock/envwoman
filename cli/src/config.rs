use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub api_url: String,
    pub api_key: String,
    pub salt: String,
}
impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            api_url: "http://localhost:8000".into(),
            api_key: "".into(),
            salt: "".into(),
        }
    }
}
