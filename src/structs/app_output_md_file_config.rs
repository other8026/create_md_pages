use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct AppOutputMdFileConfig {
    pub title: String,
    pub description: String,
    pub alias: String,
    pub before_text: Option<String>,
    pub after_text: Option<String>,
}
