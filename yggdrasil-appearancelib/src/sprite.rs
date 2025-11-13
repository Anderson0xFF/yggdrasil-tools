use crate::error::{AppearanceError, Result};
use crate::types::{Animation, SpriteData};
use flate2::write::GzEncoder;
use flate2::Compression;
use image::GenericImageView;
use std::io::Write;
use std::path::Path;

const COMPRESSION_LEVEL: u32 = 6;

/// Carrega uma sprite e extrai seus pixels compactados
pub fn load_sprite<P: AsRef<Path>>(
    path: P, appearance_name: &str, animation_name: &str, animation: &Animation,
) -> Result<SpriteData> {
    let path_ref = path.as_ref();

    // Verifica se o arquivo existe
    if !path_ref.exists() {
        return Err(AppearanceError::SpriteNotFound {
            path: path_ref.display().to_string(),
        });
    }

    // Carrega a imagem
    let image = image::open(path_ref)?;
    let (actual_width, actual_height) = image.dimensions();

    // Calcula dimensões esperadas
    let expected_width = if animation.directions > 0 {
        animation.size() * animation.frames
    } else {
        animation.size() * animation.frames
    };

    let expected_height = if animation.directions > 0 {
        animation.size() * animation.directions
    } else {
        animation.size()
    };

    // Valida dimensões
    if actual_width != expected_width || actual_height != expected_height {
        return Err(AppearanceError::InvalidDimensions {
            name: appearance_name.to_string(),
            animation: animation_name.to_string(),
            expected_width,
            expected_height,
            actual_width,
            actual_height,
        });
    }

    // Extrai pixels RGBA
    let rgba = image.to_rgba8();
    let pixels = rgba.into_raw();

    // Compacta os pixels
    let compressed_pixels = compress_pixels(&pixels)?;

    Ok(SpriteData {
        width: actual_width,
        height: actual_height,
        compressed_pixels,
    })
}

/// Compacta pixels usando Gzip
fn compress_pixels(pixels: &[u8]) -> Result<Vec<u8>> {
    let mut compressed = Vec::new();
    let mut encoder = GzEncoder::new(&mut compressed, Compression::new(COMPRESSION_LEVEL));
    encoder.write_all(pixels)?;
    encoder.finish()?;
    Ok(compressed)
}

impl Animation {
    /// Retorna o tamanho base de cada frame da sprite
    /// Nota: Assumimos que 'size' está definido no nível da Appearance,
    /// mas precisamos passá-lo aqui. Vamos adicionar ao Animation.
    fn size(&self) -> u32 {
        // Este método será usado internamente
        // O tamanho vem da Appearance, não da Animation
        // Vamos refatorar para passar o size como parâmetro
        0
    }
}

/// Carrega uma sprite com o tamanho especificado
pub fn load_sprite_with_size<P: AsRef<Path>>(
    path: P, appearance_name: &str, animation_name: &str, animation: &Animation, size: u32,
) -> Result<SpriteData> {
    let path_ref = path.as_ref();

    // Verifica se o arquivo existe
    if !path_ref.exists() {
        return Err(AppearanceError::SpriteNotFound {
            path: path_ref.display().to_string(),
        });
    }

    // Carrega a imagem
    let image = image::open(path_ref)?;
    let (actual_width, actual_height) = image.dimensions();

    // Calcula dimensões esperadas
    let expected_width = if animation.directions > 0 {
        size * animation.frames
    } else {
        size * animation.frames
    };

    let expected_height = if animation.directions > 0 {
        size * animation.directions
    } else {
        size
    };

    // Valida dimensões
    if actual_width != expected_width || actual_height != expected_height {
        return Err(AppearanceError::InvalidDimensions {
            name: appearance_name.to_string(),
            animation: animation_name.to_string(),
            expected_width,
            expected_height,
            actual_width,
            actual_height,
        });
    }

    // Extrai pixels RGBA
    let rgba = image.to_rgba8();
    let pixels = rgba.into_raw();

    // Compacta os pixels
    let compressed_pixels = compress_pixels(&pixels)?;

    Ok(SpriteData {
        width: actual_width,
        height: actual_height,
        compressed_pixels,
    })
}
