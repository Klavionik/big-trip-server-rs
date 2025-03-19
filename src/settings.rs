use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Settings {
    pub database_url: String,
}
