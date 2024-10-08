use ratatui::style::Color;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct ColorsConfig {
    #[serde(deserialize_with = "crate::config::hex_to_rgb")]
    pub title: Color,
    #[serde(deserialize_with = "crate::config::hex_to_rgb")]
    pub connected: Color,
    #[serde(deserialize_with = "crate::config::hex_to_rgb")]
    pub disconnected: Color,
    #[serde(deserialize_with = "crate::config::hex_to_rgb")]
    pub items: Color,
    #[serde(deserialize_with = "crate::config::hex_to_rgb")]
    pub items_selected: Color,
    #[serde(deserialize_with = "crate::config::hex_to_rgb")]
    pub normal_mode: Color,
    #[serde(deserialize_with = "crate::config::hex_to_rgb")]
    pub search_mode: Color,
    #[serde(deserialize_with = "crate::config::hex_to_rgb")]
    pub connection_output: Color,
    #[serde(deserialize_with = "crate::config::hex_to_rgb")]
    pub background: Color,
}

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    pub colors: ColorsConfig,
}

impl Config {
    pub fn load(path: Option<&str>) -> Result<Self, std::io::Error> {
        let config = if let Some(p) = path {
            std::fs::read_to_string(p)?
        } else {
           let mut p = std::env::var("HOME").unwrap_or("./".to_string());
           p.push_str("/.config/nordvpn-tui/config.toml");
           std::fs::read_to_string(p)?
        };
        toml::from_str(&config).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }
}

fn hex_to_rgb<'de, D>(deserializer: D) -> Result<Color, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let hex_str: String = String::deserialize(deserializer)?;
    let hex = hex_str.trim_start_matches('#');

    if hex.len() != 6 {
        return Err(serde::de::Error::custom(format!("Invalid hex color: {}", hex_str)));
    }

    let r = u8::from_str_radix(&hex[0..2], 16).map_err(serde::de::Error::custom)?;
    let g = u8::from_str_radix(&hex[2..4], 16).map_err(serde::de::Error::custom)?;
    let b = u8::from_str_radix(&hex[4..6], 16).map_err(serde::de::Error::custom)?;

    Ok(Color::Rgb(r, g, b))
}
