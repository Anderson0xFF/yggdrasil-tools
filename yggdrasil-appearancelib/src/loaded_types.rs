use std::collections::HashMap;

/// Appearance carregada do arquivo .dat
#[derive(Default, Debug, Clone)]
pub struct LoadedAppearance {
    pub id:         u32,
    pub name:       String,
    pub size:       u32,
    pub animations: HashMap<String, LoadedAnimation>,
}

/// Animação carregada
#[derive(Default, Debug, Clone)]
pub struct LoadedAnimation {
    pub name:       String,
    pub sprite_id:  u32,
    pub width:      u32,
    pub height:     u32,
    pub frames:     u32,
    pub directions: u32,
    pub duration:   u32,
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
    /// Busca uma animação por nome
    pub fn get_animation(&self, name: &str) -> Option<&LoadedAnimation> {
        self.animations.get(name)
    }

    /// Retorna todas as animações
    pub fn all_animations(&self) -> impl Iterator<Item = &LoadedAnimation> {
        self.animations.values()
    }

    /// Retorna os nomes de todas as animações
    pub fn animation_names(&self) -> impl Iterator<Item = &String> {
        self.animations.keys()
    }
}
