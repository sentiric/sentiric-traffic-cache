use anyhow::{Context, Result};
use sentiric_core::Settings;
use std::sync::OnceLock;

static SETTINGS: OnceLock<Settings> = OnceLock::new();

pub fn init() -> Result<()> {
    let settings = config::Config::builder()
        .add_source(config::File::with_name("config.toml").required(true))
        .add_source(config::File::with_name("rules.toml").required(false)) // <-- YENİ
        .build()
        .context("Failed to build configuration")?
        .try_deserialize()
        .context("Failed to deserialize configuration")?;

    SETTINGS.set(settings).map_err(|_| anyhow::anyhow!("Configuration already initialized"))?;
    Ok(())
}

pub fn get() -> &'static Settings {
    SETTINGS.get().expect("Configuration is not initialized")
}