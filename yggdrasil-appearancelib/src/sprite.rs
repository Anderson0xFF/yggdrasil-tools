use crate::error::{AppearanceError, Result};
use crate::types::{Orientation, SpriteData};
use flate2::Compression;
use flate2::write::GzEncoder;
use image::{DynamicImage, GenericImageView};
use std::io::Write;
use std::path::Path;

const COMPRESSION_LEVEL: u32 = 6;

/// Recorta um spritesheet em sprites individuais
///
/// # Parâmetros
/// - `spritesheet_path`: Caminho para o spritesheet
/// - `sprite_size`: Tamanho de cada sprite (largura e altura)
/// - `num_frames`: Número de frames
/// - `num_directions`: Número de direções, 0 se não houver direções
/// - `orientation`: Orientação do spritesheet (Vertical ou Horizontal)
///
/// # Retorna
/// Um vetor de `SpriteData`, onde cada elemento é uma sprite individual recortada
pub fn slice_spritesheet<P: AsRef<Path>>(
    spritesheet_path: P,
    sprite_size: u32,
    num_frames: u32,
    num_directions: u32,
    orientation: Orientation,
) -> Result<Vec<SpriteData>> {
    let path_ref = spritesheet_path.as_ref();

    // Verifica se o arquivo existe
    if !path_ref.exists() {
        return Err(AppearanceError::SpriteNotFound {
            path: path_ref.display().to_string(),
        });
    }

    // Carrega a imagem
    let spritesheet = image::open(path_ref)?;
    let (sheet_width, sheet_height) = spritesheet.dimensions();

    // Calcula dimensões esperadas baseado na orientação
    let (expected_width, expected_height) = match orientation {
        Orientation::Horizontal => {
            // Horizontal: direções em colunas (lado a lado), frames em linhas (empilhados)
            // Exemplo: [N][E][S][W] ← Frame 1
            //          [N][E][S][W] ← Frame 2
            let width = if num_directions > 0 {
                sprite_size * num_directions
            } else {
                sprite_size
            };
            let height = sprite_size * num_frames;
            (width, height)
        }
        Orientation::Vertical => {
            // Vertical: frames em colunas (lado a lado), direções em linhas (empilhadas)
            // Exemplo: [N1][N2][N3]... ← Norte
            //          [E1][E2][E3]... ← Leste
            let width = sprite_size * num_frames;
            let height = if num_directions > 0 {
                sprite_size * num_directions
            } else {
                sprite_size
            };
            (width, height)
        }
    };

    // Valida dimensões do spritesheet
    if sheet_width != expected_width || sheet_height != expected_height {
        return Err(AppearanceError::InvalidDimensions {
            name: path_ref.display().to_string(),
            animation: "spritesheet".to_string(),
            expected_width,
            expected_height,
            actual_width: sheet_width,
            actual_height: sheet_height,
        });
    }

    let mut sprites = Vec::new();

    // Recorta baseado na orientação
    match orientation {
        Orientation::Horizontal => {
            // Horizontal: itera linhas (frames) e depois colunas (direções)
            // Para cada frame, percorre todas as direções
            for row in 0..num_frames {
                let cols = if num_directions > 0 { num_directions } else { 1 };
                for col in 0..cols {
                    let x = col * sprite_size;
                    let y = row * sprite_size;

                    let sprite_img = spritesheet.crop_imm(x, y, sprite_size, sprite_size);
                    let sprite_data = image_to_sprite_data(&sprite_img)?;
                    sprites.push(sprite_data);
                }
            }
        }
        Orientation::Vertical => {
            // Vertical: itera linhas (direções) e depois colunas (frames)
            // Para cada direção, percorre todos os frames
            let rows = if num_directions > 0 { num_directions } else { 1 };
            for row in 0..rows {
                for col in 0..num_frames {
                    let x = col * sprite_size;
                    let y = row * sprite_size;

                    let sprite_img = spritesheet.crop_imm(x, y, sprite_size, sprite_size);
                    let sprite_data = image_to_sprite_data(&sprite_img)?;
                    sprites.push(sprite_data);
                }
            }
        }
    }

    Ok(sprites)
}

/// Converte uma DynamicImage em SpriteData compactado
fn image_to_sprite_data(image: &DynamicImage) -> Result<SpriteData> {
    let (width, height) = image.dimensions();

    // Extrai pixels RGBA
    let rgba = image.to_rgba8();
    let pixels = rgba.into_raw();

    // Compacta os pixels
    let compressed_pixels = compress_pixels(&pixels)?;

    Ok(SpriteData {
        width,
        height,
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
