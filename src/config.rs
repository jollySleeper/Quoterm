use std::path::PathBuf;
use std::fs;
use serde::{Deserialize, Serialize};
use dirs;
use termion::color;
use crate::error::QuotermError;
use crate::colors::Color;

/// Configuration for Quoterm
/// 
/// The configuration file is stored in JSON format at:
/// - Linux/macOS: `~/.config/quoterm/config.json`
/// - Windows: `%APPDATA%\quoterm\config.json`
/// 
/// Example configuration:
/// ```json
/// {
///   "colors": {
///     "divider": "yellow",
///     "quote": "blue",
///     "author": "red"
///   }
/// }
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Color configuration for different elements
    pub colors: Colors,
}

/// Color configuration for different elements of the quote display
#[derive(Debug, Serialize, Deserialize)]
pub struct Colors {
    /// Color for the divider line
    pub divider: Color,
    /// Color for the quote text
    pub quote: Color,
    /// Color for the author attribution
    pub author: Color,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            colors: Colors {
                divider: Color::Yellow,
                quote: Color::Blue,
                author: Color::Red,
            },
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, QuotermError> {
        let config_path = get_config_path()?;
        
        if !config_path.exists() {
            let config = Config::default();
            let content = serde_json::to_string_pretty(&config)
                .map_err(|e| QuotermError::JsonError(e))?;
            fs::write(&config_path, content)
                .map_err(|e| QuotermError::IoError(e))?;
            return Ok(config);
        }

        let content = fs::read_to_string(config_path)
            .map_err(|e| QuotermError::IoError(e))?;
        serde_json::from_str(&content)
            .map_err(|e| QuotermError::JsonError(e))
    }

    pub fn get_divider_color(&self) -> color::Rgb {
        self.colors.divider.to_termion_color()
    }

    pub fn get_quote_color(&self) -> color::Rgb {
        self.colors.quote.to_termion_color()
    }

    pub fn get_author_color(&self) -> color::Rgb {
        self.colors.author.to_termion_color()
    }
}

fn get_config_path() -> Result<PathBuf, QuotermError> {
    // Allow overriding config path through environment variable (useful for testing)
    if let Ok(path) = std::env::var("QUOTERM_CONFIG_PATH") {
        return Ok(PathBuf::from(path));
    }

    let mut path = dirs::config_dir()
        .ok_or_else(|| QuotermError::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Could not find config directory",
        )))?;
    path.push("quoterm");
    fs::create_dir_all(&path)
        .map_err(|e| QuotermError::IoError(e))?;
    path.push("config.json");
    Ok(path)
}
