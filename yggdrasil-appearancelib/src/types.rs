use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Orientação dos frames na spritesheet
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Orientation {
    Vertical,
    Horizontal,
}

impl Default for Orientation {
    fn default() -> Self {
        Orientation::Vertical
    }
}

/// Offset para renderização
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Offset {
    pub x: i32,
    pub y: i32,
}

/// Arquivo principal de appearances (appearances.json)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppearancesFile {
    pub version:     u32,
    pub appearances: Vec<Appearance>,
}

/// Uma appearance (criatura, item, efeito, projétil, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Appearance {
    pub id:         u32,
    pub name:       String,
    #[serde(default)]
    pub offset:     Offset,
    pub size:       u32,
    pub animations: HashMap<String, Animation>,
}

/// Uma animação dentro de uma appearance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Animation {
    pub path:        String,
    #[serde(default = "default_frames")]
    pub frames:      u32,
    #[serde(default = "default_directions")]
    pub directions:  u32,
    #[serde(default)]
    pub duration:    Option<u32>,
    #[serde(default)]
    pub orientation: Orientation,
}

fn default_frames() -> u32 {
    1
}

fn default_directions() -> u32 {
    0
}

/// Metadados de uma sprite compilada
#[derive(Debug, Clone)]
pub struct SpriteMetadata {
    pub sprite_id:  u32,
    pub width:      u32,
    pub height:     u32,
    pub frames:     u32,
    pub directions: u32,
    pub duration:   u32,
}

/// Dados de uma sprite (pixels compactados)
#[derive(Debug, Clone)]
pub struct SpriteData {
    pub width:             u32,
    pub height:            u32,
    pub compressed_pixels: Vec<u8>,
}
