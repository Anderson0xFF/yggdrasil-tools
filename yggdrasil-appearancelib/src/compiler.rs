use crate::error::Result;
use crate::sprite::load_sprite_with_size;
use crate::types::{AppearancesFile, SpriteData};
use byteorder::{LittleEndian, WriteBytesExt};
use image::GenericImageView;
use std::collections::HashMap;
use std::fs;
use std::io::{Cursor, Write};
use std::path::{Path, PathBuf};

/// Resultado da compilação
pub struct CompilationResult {
    pub appearances_count: usize,
    pub sprites_count:     usize,
    pub dat_size:          usize,
    pub total_spr_size:    usize,
}

/// Compila o appearances.json em arquivos binários
pub fn compile_appearances<P: AsRef<Path>>(
    appearances_file: &AppearancesFile, base_path: P, output_path: P,
) -> Result<CompilationResult> {
    let base_path = base_path.as_ref();
    let output_path = output_path.as_ref();

    // Cria a pasta de output se não existir
    fs::create_dir_all(output_path)?;

    // Mapa para evitar duplicar sprites (mesmo path = mesmo arquivo .spr)
    let mut sprite_cache: HashMap<String, u32> = HashMap::new();
    let mut next_sprite_id = 1u32;

    // Buffer para o arquivo .dat
    let mut dat_buffer = Cursor::new(Vec::new());

    // Escreve header do .dat
    dat_buffer.write_u32::<LittleEndian>(appearances_file.version)?;
    dat_buffer.write_u32::<LittleEndian>(appearances_file.appearances.len() as u32)?;

    let mut total_sprites = 0;
    let mut total_spr_size = 0;

    // Processa cada appearance
    for appearance in &appearances_file.appearances {
        // Escreve dados da appearance
        dat_buffer.write_u32::<LittleEndian>(appearance.id)?;

        // Nome
        write_string(&mut dat_buffer, &appearance.name)?;

        // Offset
        dat_buffer.write_i32::<LittleEndian>(appearance.offset.x)?;
        dat_buffer.write_i32::<LittleEndian>(appearance.offset.y)?;

        // Size
        dat_buffer.write_u32::<LittleEndian>(appearance.size)?;

        // Número de animações
        dat_buffer.write_u32::<LittleEndian>(appearance.animations.len() as u32)?;

        // Processa cada animação
        for (anim_name, animation) in &appearance.animations {
            // Nome da animação
            write_string(&mut dat_buffer, anim_name)?;

            // Resolve o path completo da sprite
            let sprite_path = if animation.path.starts_with("assets/") {
                base_path.join(&animation.path)
            } else {
                PathBuf::from(&animation.path)
            };

            let sprite_path_str = sprite_path.display().to_string();

            // Verifica se já processamos essa sprite
            let sprite_id = if let Some(&cached_id) = sprite_cache.get(&sprite_path_str) {
                cached_id
            } else {
                // Carrega e processa a sprite
                let sprite_data =
                    load_sprite_with_size(&sprite_path, &appearance.name, anim_name, animation, appearance.size)?;

                // Salva o arquivo .spr
                let sprite_id = next_sprite_id;
                save_sprite_file(output_path, sprite_id, &sprite_data)?;

                total_spr_size += sprite_data.compressed_pixels.len();
                sprite_cache.insert(sprite_path_str, sprite_id);
                next_sprite_id += 1;
                total_sprites += 1;

                sprite_id
            };

            // Escreve metadados da animação
            dat_buffer.write_u32::<LittleEndian>(sprite_id)?;

            // Dimensões da sprite
            let sprite_data = sprite_cache
                .iter()
                .find(|(_, id)| **id == sprite_id)
                .map(|(path, _)| {
                    // Recarrega para obter dimensões (poderia ser otimizado cacheando)
                    let img = image::open(path).ok()?;
                    Some(img.dimensions())
                })
                .flatten();

            if let Some((width, height)) = sprite_data {
                dat_buffer.write_u32::<LittleEndian>(width)?;
                dat_buffer.write_u32::<LittleEndian>(height)?;
            } else {
                // Calcula dimensões esperadas
                let width = if animation.directions > 0 {
                    appearance.size * animation.frames
                } else {
                    appearance.size * animation.frames
                };
                let height = if animation.directions > 0 {
                    appearance.size * animation.directions
                } else {
                    appearance.size
                };
                dat_buffer.write_u32::<LittleEndian>(width)?;
                dat_buffer.write_u32::<LittleEndian>(height)?;
            }

            dat_buffer.write_u32::<LittleEndian>(animation.frames)?;
            dat_buffer.write_u32::<LittleEndian>(animation.directions)?;
            dat_buffer.write_u32::<LittleEndian>(animation.duration.unwrap_or(0))?;

            // Orientation (0 = Vertical, 1 = Horizontal)
            let orientation_byte = match animation.orientation {
                crate::types::Orientation::Vertical => 0u8,
                crate::types::Orientation::Horizontal => 1u8,
            };
            dat_buffer.write_u8(orientation_byte)?;
        }
    }

    // Salva o arquivo .dat
    let dat_path = output_path.join("appearances.dat");
    let dat_bytes = dat_buffer.into_inner();
    fs::write(&dat_path, &dat_bytes)?;

    Ok(CompilationResult {
        appearances_count: appearances_file.appearances.len(),
        sprites_count: total_sprites,
        dat_size: dat_bytes.len(),
        total_spr_size,
    })
}

/// Salva um arquivo .spr
fn save_sprite_file<P: AsRef<Path>>(output_path: P, sprite_id: u32, sprite_data: &SpriteData) -> Result<()> {
    let filename = format!("{:05}.spr", sprite_id);
    let filepath = output_path.as_ref().join(filename);

    let mut buffer = Cursor::new(Vec::new());

    // Header do .spr
    buffer.write_u32::<LittleEndian>(sprite_data.width)?;
    buffer.write_u32::<LittleEndian>(sprite_data.height)?;
    buffer.write_u32::<LittleEndian>(sprite_data.compressed_pixels.len() as u32)?;

    // Pixels compactados
    buffer.write_all(&sprite_data.compressed_pixels)?;

    // Salva o arquivo
    fs::write(filepath, buffer.into_inner())?;

    Ok(())
}

/// Escreve uma string no formato: length (u32) + bytes (UTF-8)
fn write_string<W: Write>(writer: &mut W, s: &str) -> Result<()> {
    writer.write_u32::<LittleEndian>(s.len() as u32)?;
    writer.write_all(s.as_bytes())?;
    Ok(())
}
