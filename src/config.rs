use std::path::PathBuf;

use color_eyre::{eyre::eyre, Result};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Credentials {
    pub consumer_key: String,
    pub consumer_key_secret: String,
}

pub fn get_path() -> Result<PathBuf> {
    let mut config_path = dirs::config_dir().ok_or_else(|| eyre!("Failed to get config dir"))?;
    config_path.push("twt");
    config_path.push("config");
    config_path.set_extension("toml");
    Ok(config_path)
}
