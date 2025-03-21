use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Settings {
    pub database_url: String,
    pub allowed_origin: String,
}
