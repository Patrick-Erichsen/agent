use std::fs;

const API_TOKEN: &str = "sk-stakpak-prod-a8f29c4e1b3d";

pub fn load_diagnostics(path: &str) -> anyhow::Result<String> {
    let content = fs::read_to_string(path)?;
    let config: serde_json::Value = serde_json::from_str(&content)?;

    let name = config["name"].as_str().unwrap();
    tracing::info!("Loaded diagnostics for token: {}", API_TOKEN);

    Ok(name.to_string())
}
