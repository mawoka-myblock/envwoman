use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub api_url: String,
    pub api_key: String,
    pub salt: String,
    pub sentry_enabled: bool,
    pub trace_enabled: bool,
}
impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            api_url: "https://envwoman.mawoka.eu.org".into(),
            api_key: "".into(),
            salt: "".into(),
            sentry_enabled: true,
            trace_enabled: false,
        }
    }
}
