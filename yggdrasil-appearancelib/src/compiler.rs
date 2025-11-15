use crate::error::Result;
use crate::sprite::slice_spritesheet;
use crate::types::{AppearancesFile, Direction, SpriteData};
use byteorder::{LittleEndian, WriteBytesExt};
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

        // Número de framegroups
        dat_buffer.write_u32::<LittleEndian>(appearance.framegroups.len() as u32)?;

        // Processa cada framegroup
        for framegroup in &appearance.framegroups {
            // Nome do framegroup
            write_string(&mut dat_buffer, &framegroup.name)?;

            // Resolve o path completo do spritesheet
            let spritesheet_path = if framegroup.spritesheet.starts_with("assets/") {
                base_path.join(&framegroup.spritesheet)
            } else {
                PathBuf::from(&framegroup.spritesheet)
            };

            // Número de animações (direções)
            dat_buffer.write_u32::<LittleEndian>(framegroup.animations.len() as u32)?;

            // Processa cada animação/direção
            for (direction, animation) in &framegroup.animations {
                // Escreve a direção (ou None se não houver)
                if let Some(dir) = direction {
                    dat_buffer.write_u8(1)?; // Tem direção
                    dat_buffer.write_u8(direction_to_u8(*dir))?;
                } else {
                    dat_buffer.write_u8(0)?; // Sem direção
                }

                // Determina o número de direções para recorte
                let num_directions = if direction.is_some() {
                    // Se há uma direção específica, assumimos que o spritesheet
                    // contém todas as direções em linhas
                    framegroup.animations.len() as u32
                } else {
                    0
                };

                // Recorta o spritesheet em sprites individuais
                let sprites = slice_spritesheet(
                    &spritesheet_path,
                    appearance.size,
                    animation.frame_count,
                    num_directions,
                    framegroup.orientation,
                )?;

                // Determina quais sprites pertencem a esta animação específica
                let sprite_ids: Vec<u32> = if let Some(dir) = direction {
                    // Calcula os índices baseado na direção e orientação
                    let direction_index = calculate_direction_row(*dir, &framegroup.animations);

                    // Para orientação Horizontal: sprites são organizadas por frame
                    // Frame 0: [N][E][S][W], Frame 1: [N][E][S][W], etc.
                    // Para orientação Vertical: sprites são organizadas por direção
                    // North: [N1][N2][N3]..., East: [E1][E2][E3]..., etc.
                    let sprite_indices: Vec<usize> = match framegroup.orientation {
                        crate::types::Orientation::Horizontal => {
                            // Para cada frame, pega a sprite na coluna da direção
                            (0..animation.frame_count)
                                .map(|frame| (frame as usize * num_directions as usize) + direction_index)
                                .collect()
                        }
                        crate::types::Orientation::Vertical => {
                            // Pega todas as sprites da linha da direção
                            let start_idx = direction_index * animation.frame_count as usize;
                            (start_idx..start_idx + animation.frame_count as usize).collect()
                        }
                    };

                    // Salva as sprites
                    sprite_indices
                        .iter()
                        .map(|&idx| {
                            let sprite_data = &sprites[idx];
                            let sprite_id = next_sprite_id;
                            save_sprite_file(output_path, sprite_id, sprite_data).ok();
                            total_spr_size += sprite_data.compressed_pixels.len();
                            total_sprites += 1;
                            next_sprite_id += 1;
                            sprite_id
                        })
                        .collect()
                } else {
                    // Sem direção, todas as sprites são dessa animação
                    sprites
                        .iter()
                        .map(|sprite_data| {
                            let sprite_id = next_sprite_id;
                            save_sprite_file(output_path, sprite_id, sprite_data).ok();
                            total_spr_size += sprite_data.compressed_pixels.len();
                            total_sprites += 1;
                            next_sprite_id += 1;
                            sprite_id
                        })
                        .collect()
                };

                // Escreve o número de sprite IDs
                dat_buffer.write_u32::<LittleEndian>(sprite_ids.len() as u32)?;

                // Escreve cada sprite ID
                for sprite_id in sprite_ids {
                    dat_buffer.write_u32::<LittleEndian>(sprite_id)?;
                }

                // Escreve a duração
                dat_buffer.write_u32::<LittleEndian>(animation.duration.unwrap_or(0))?;

                // Escreve o flag looped (1 = true, 0 = false)
                dat_buffer.write_u8(if animation.looped.unwrap_or(true) { 1 } else { 0 })?;
            }
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

/// Converte Direction para u8
fn direction_to_u8(dir: Direction) -> u8 {
    match dir {
        Direction::North => 0,
        Direction::East => 1,
        Direction::South => 2,
        Direction::West => 3,
        Direction::NorthEast => 4,
        Direction::SouthEast => 5,
        Direction::SouthWest => 6,
        Direction::NorthWest => 7,
    }
}

/// Calcula o índice da linha para uma direção específica
fn calculate_direction_row(dir: Direction, animations: &HashMap<Option<Direction>, crate::types::Animation>) -> usize {
    // Ordena as direções para garantir consistência
    let mut directions: Vec<Direction> = animations.keys().filter_map(|k| *k).collect();

    directions.sort_by_key(|d| direction_to_u8(*d));

    directions.iter().position(|d| *d == dir).unwrap_or(0)
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
