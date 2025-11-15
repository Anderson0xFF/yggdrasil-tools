use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

// Re-export Direction from common
pub use yggdrasil_common::types::Direction;

/// Orientação do spritesheet
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Orientation {
    /// Frames em colunas, direções em linhas (padrão)
    /// Largura = size × frames, Altura = size × directions
    Vertical,
    /// Direções em colunas, frames em linhas
    /// Largura = size × directions, Altura = size × frames
    Horizontal,
}

impl Default for Orientation {
    fn default() -> Self {
        Orientation::Vertical
    }
}

/// Wrapper para deserializar Option<Direction> com suporte a "null" como string
fn deserialize_direction_map<'de, D>(
    deserializer: D,
) -> Result<HashMap<Option<Direction>, Animation>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;

    let map: HashMap<String, Animation> = HashMap::deserialize(deserializer)?;
    let mut result = HashMap::new();

    for (key, value) in map {
        let direction = if key == "null" {
            None
        } else {
            Some(serde_json::from_str(&format!("\"{}\"", key)).map_err(D::Error::custom)?)
        };
        result.insert(direction, value);
    }

    Ok(result)
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
    pub id:          u32,
    pub name:        String,
    #[serde(default)]
    pub offset:      Offset,
    pub size:        u32,
    pub framegroups: Vec<FrameGroup>,
}

/// Um grupo de frames com diferentes direções e animações
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameGroup {
    pub name:        String,
    /// Caminho para o spritesheet que será recortado
    pub spritesheet: String,
    /// Orientação do spritesheet (vertical ou horizontal)
    #[serde(default)]
    pub orientation: Orientation,
    /// Mapa de direções para animações
    /// Se não houver direções, usar uma única entrada sem direção
    #[serde(deserialize_with = "deserialize_direction_map")]
    pub animations:  HashMap<Option<Direction>, Animation>,
}

/// Uma animação com lista de sprite IDs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Animation {
    /// Lista de sprite IDs que compõem esta animação
    /// Estes IDs serão gerados pelo compilador ao recortar o spritesheet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sprite_ids:  Option<Vec<u32>>,
    /// Número de frames (usado durante a compilação para recortar)
    #[serde(default = "default_frames")]
    pub frame_count: u32,
    /// Duração total da animação em milissegundos
    #[serde(default)]
    pub duration:    Option<u32>,
    /// Se a animação deve fazer loop (padrão: true)
    #[serde(default = "default_looped")]
    pub looped:      Option<bool>,
}

fn default_frames() -> u32 {
    1
}

fn default_looped() -> Option<bool> {
    Some(true)
}

/// Metadados de uma sprite individual compilada
#[derive(Debug, Clone)]
pub struct SpriteMetadata {
    pub sprite_id: u32,
    pub width:     u32,
    pub height:    u32,
}

/// Dados de uma sprite individual (pixels compactados)
#[derive(Debug, Clone)]
pub struct SpriteData {
    pub width:             u32,
    pub height:            u32,
    pub compressed_pixels: Vec<u8>,
}
