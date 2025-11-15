use crate::types::{Direction, Offset};
use std::collections::HashMap;

/// Appearance carregada do arquivo .dat
#[derive(Default, Debug, Clone)]
pub struct LoadedAppearance {
    pub id:          u32,
    pub name:        String,
    pub offset:      Offset,
    pub size:        u32,
    pub framegroups: Vec<LoadedFrameGroup>,
}

/// FrameGroup carregado
#[derive(Debug, Clone)]
pub struct LoadedFrameGroup {
    pub name:       String,
    pub animations: HashMap<Option<Direction>, LoadedAnimation>,
}

/// Animação carregada com lista de sprite IDs
#[derive(Debug, Clone)]
pub struct LoadedAnimation {
    pub sprite_ids: Vec<u32>,
    pub duration:   u32,
    pub looped:     bool,
}

/// Sprite carregada (pixels descompactados)
#[derive(Default, Debug, Clone)]
pub struct LoadedSprite {
    pub sprite_id: u32,
    pub width:     u32,
    pub height:    u32,
    pub pixels:    Vec<u8>, // RGBA descompactado
}

/// Database completa de appearances
#[derive(Default, Debug, Clone)]
pub struct AppearanceDatabase {
    pub version:     u32,
    pub appearances: HashMap<u32, LoadedAppearance>,
}

impl AppearanceDatabase {
    /// Cria um database vazio
    pub fn new(version: u32) -> Self {
        Self {
            version,
            appearances: HashMap::new(),
        }
    }

    /// Adiciona uma appearance ao database
    pub fn add_appearance(&mut self, appearance: LoadedAppearance) {
        self.appearances.insert(appearance.id, appearance);
    }

    /// Busca uma appearance por ID
    pub fn get_appearance(&self, id: u32) -> Option<&LoadedAppearance> {
        self.appearances.get(&id)
    }

    /// Retorna todas as appearances
    pub fn all_appearances(&self) -> impl Iterator<Item = &LoadedAppearance> {
        self.appearances.values()
    }

    /// Retorna o número de appearances carregadas
    pub fn count(&self) -> usize {
        self.appearances.len()
    }
}

impl LoadedAppearance {
    /// Busca um framegroup por nome
    pub fn get_framegroup(&self, name: &str) -> Option<&LoadedFrameGroup> {
        self.framegroups.iter().find(|fg| fg.name == name)
    }

    /// Retorna todos os framegroups
    pub fn all_framegroups(&self) -> &[LoadedFrameGroup] {
        &self.framegroups
    }

    /// Retorna os nomes de todos os framegroups
    pub fn framegroup_names(&self) -> impl Iterator<Item = &String> {
        self.framegroups.iter().map(|fg| &fg.name)
    }
}

impl LoadedFrameGroup {
    /// Busca uma animação por direção
    pub fn get_animation(&self, direction: Option<Direction>) -> Option<&LoadedAnimation> {
        self.animations.get(&direction)
    }

    /// Retorna a animação para uma direção ou None (padrão)
    pub fn get_animation_or_default(&self, direction: Option<Direction>) -> Option<&LoadedAnimation> {
        self.animations.get(&direction).or_else(|| self.animations.get(&None))
    }
}
