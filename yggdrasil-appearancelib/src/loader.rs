use crate::error::{AppearanceError, Result};
use crate::loaded_types::{
    AppearanceDatabase, FrameOrientation, LoadedAnimation, LoadedAppearance, LoadedSprite,
};
use crate::types::Offset;
use byteorder::{LittleEndian, ReadBytesExt};
use flate2::read::GzDecoder;
use std::collections::HashMap;
use std::fs;
use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};

/// Loader para arquivos compilados (.dat + .spr)
#[derive(Default, Debug, Clone)]
pub struct AppearanceLoader {
    base_path:    PathBuf,
    sprite_cache: HashMap<u32, LoadedSprite>,
}

impl AppearanceLoader {
    /// Cria um novo loader
    pub fn new<P: AsRef<Path>>(base_path: P) -> Self {
        Self {
            base_path:    base_path.as_ref().to_path_buf(),
            sprite_cache: HashMap::new(),
        }
    }

    /// Carrega o arquivo appearances.dat completo
    pub fn load_database(&mut self) -> Result<AppearanceDatabase> {
        let dat_path = self.base_path.join("appearances.dat");
        let data = fs::read(&dat_path)?;
        let mut cursor = Cursor::new(data);

        // Lê header
        let version = cursor.read_u32::<LittleEndian>()?;
        let appearance_count = cursor.read_u32::<LittleEndian>()?;

        let mut database = AppearanceDatabase::new(version);

        // Lê todas as appearances
        for _ in 0..appearance_count {
            let appearance = self.read_appearance(&mut cursor)?;
            database.add_appearance(appearance);
        }

        Ok(database)
    }

    /// Lê uma appearance do cursor
    fn read_appearance<R: Read>(&self, cursor: &mut R) -> Result<LoadedAppearance> {
        // ID
        let id = cursor.read_u32::<LittleEndian>()?;

        // Nome
        let name = read_string(cursor)?;

        // Offset
        let offset_x = cursor.read_i32::<LittleEndian>()?;
        let offset_y = cursor.read_i32::<LittleEndian>()?;
        let offset = Offset {
            x: offset_x,
            y: offset_y,
        };

        // Size
        let size = cursor.read_u32::<LittleEndian>()?;

        // Animações
        let animation_count = cursor.read_u32::<LittleEndian>()?;
        let mut animations = HashMap::new();

        for _ in 0..animation_count {
            let anim_name = read_string(cursor)?;
            let sprite_id = cursor.read_u32::<LittleEndian>()?;
            let width = cursor.read_u32::<LittleEndian>()?;
            let height = cursor.read_u32::<LittleEndian>()?;
            let frames = cursor.read_u32::<LittleEndian>()?;
            let directions = cursor.read_u32::<LittleEndian>()?;
            let duration = cursor.read_u32::<LittleEndian>()?;

            // Lê a orientação (0 = Vertical, 1 = Horizontal)
            let orientation_byte = cursor.read_u8()?;
            let orientation = match orientation_byte {
                0 => FrameOrientation::Vertical,
                1 => FrameOrientation::Horizontal,
                _ => FrameOrientation::Vertical, // Default para valores inválidos
            };

            let animation = LoadedAnimation {
                name: anim_name.clone(),
                sprite_id,
                width,
                height,
                frames,
                directions,
                duration,
                orientation,
            };

            animations.insert(anim_name, animation);
        }

        Ok(LoadedAppearance {
            id,
            name,
            offset,
            size,
            animations,
        })
    }

    /// Carrega um arquivo .spr específico
    pub fn load_sprite(&mut self, sprite_id: u32) -> Result<&LoadedSprite> {
        // Verifica se já está no cache
        if self.sprite_cache.contains_key(&sprite_id) {
            return Ok(&self.sprite_cache[&sprite_id]);
        }

        // Carrega do arquivo
        let sprite = self.load_sprite_from_file(sprite_id)?;
        self.sprite_cache.insert(sprite_id, sprite);

        Ok(&self.sprite_cache[&sprite_id])
    }

    /// Carrega um sprite do arquivo .spr
    fn load_sprite_from_file(&self, sprite_id: u32) -> Result<LoadedSprite> {
        let filename = format!("{:05}.spr", sprite_id);
        let sprite_path = self.base_path.join(&filename);

        if !sprite_path.exists() {
            return Err(AppearanceError::SpriteNotFound {
                path: sprite_path.display().to_string(),
            });
        }

        let data = fs::read(&sprite_path)?;
        let mut cursor = Cursor::new(data);

        // Lê header
        let width = cursor.read_u32::<LittleEndian>()?;
        let height = cursor.read_u32::<LittleEndian>()?;
        let compressed_size = cursor.read_u32::<LittleEndian>()?;

        // Lê pixels compactados
        let mut compressed_pixels = vec![0u8; compressed_size as usize];
        cursor.read_exact(&mut compressed_pixels)?;

        // Descompacta
        let mut decoder = GzDecoder::new(&compressed_pixels[..]);
        let mut pixels = Vec::new();
        decoder.read_to_end(&mut pixels)?;

        Ok(LoadedSprite {
            sprite_id,
            width,
            height,
            pixels,
        })
    }

    /// Pré-carrega múltiplos sprites de uma vez
    pub fn preload_sprites(&mut self, sprite_ids: &[u32]) -> Result<()> {
        for &sprite_id in sprite_ids {
            if !self.sprite_cache.contains_key(&sprite_id) {
                let sprite = self.load_sprite_from_file(sprite_id)?;
                self.sprite_cache.insert(sprite_id, sprite);
            }
        }
        Ok(())
    }

    /// Pré-carrega todos os sprites de uma appearance
    pub fn preload_appearance_sprites(&mut self, appearance: &LoadedAppearance) -> Result<()> {
        let sprite_ids: Vec<u32> = appearance.animations.values().map(|anim| anim.sprite_id).collect();

        self.preload_sprites(&sprite_ids)
    }

    /// Retorna um sprite do cache (se estiver carregado)
    pub fn get_cached_sprite(&self, sprite_id: u32) -> Option<&LoadedSprite> {
        self.sprite_cache.get(&sprite_id)
    }

    /// Limpa o cache de sprites
    pub fn clear_sprite_cache(&mut self) {
        self.sprite_cache.clear();
    }

    /// Retorna o número de sprites no cache
    pub fn cached_sprite_count(&self) -> usize {
        self.sprite_cache.len()
    }

    /// Retorna o tamanho total do cache em bytes
    pub fn cache_size_bytes(&self) -> usize {
        self.sprite_cache.values().map(|sprite| sprite.pixels.len()).sum()
    }
}

/// Lê uma string do formato: length (u32) + bytes (UTF-8)
fn read_string<R: Read>(reader: &mut R) -> Result<String> {
    let length = reader.read_u32::<LittleEndian>()?;
    let mut bytes = vec![0u8; length as usize];
    reader.read_exact(&mut bytes)?;
    let s =
        String::from_utf8(bytes).map_err(|e| AppearanceError::InvalidData(format!("Invalid UTF-8 string: {}", e)))?;
    Ok(s)
}

/// Função helper para carregar database + todos os sprites de uma vez
pub fn load_all<P: AsRef<Path>>(base_path: P) -> Result<(AppearanceDatabase, AppearanceLoader)> {
    let mut loader = AppearanceLoader::new(base_path);
    let database = loader.load_database()?;

    // Pré-carrega todos os sprites
    let all_sprite_ids: Vec<u32> = database
        .all_appearances()
        .flat_map(|app| app.animations.values().map(|anim| anim.sprite_id))
        .collect();

    loader.preload_sprites(&all_sprite_ids)?;

    Ok((database, loader))
}

/// Função helper para carregar apenas o database (lazy loading de sprites)
pub fn load_database_only<P: AsRef<Path>>(base_path: P) -> Result<(AppearanceDatabase, AppearanceLoader)> {
    let mut loader = AppearanceLoader::new(base_path);
    let database = loader.load_database()?;
    Ok((database, loader))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_string() {
        let data = vec![
            5, 0, 0, 0, // length = 5
            b'h', b'e', b'l', b'l', b'o',
        ];
        let mut cursor = Cursor::new(data);
        let result = read_string(&mut cursor).unwrap();
        assert_eq!(result, "hello");
    }
}
