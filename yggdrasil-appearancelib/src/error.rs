use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppearanceError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Image error: {0}")]
    Image(#[from] image::ImageError),

    #[error("Invalid sprite dimensions for appearance '{name}' animation '{animation}': expected {expected_width}x{expected_height}, got {actual_width}x{actual_height}")]
    InvalidDimensions {
        name:            String,
        animation:       String,
        expected_width:  u32,
        expected_height: u32,
        actual_width:    u32,
        actual_height:   u32,
    },

    #[error("Sprite file not found: {path}")]
    SpriteNotFound { path: String },

    #[error("Invalid appearance data: {0}")]
    InvalidData(String),
}

pub type Result<T> = std::result::Result<T, AppearanceError>;
