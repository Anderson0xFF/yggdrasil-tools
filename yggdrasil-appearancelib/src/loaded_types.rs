use std::collections::HashMap;

/// Orientação dos frames na spritesheet
///
/// Define como os frames e direções estão organizados na imagem.
///
/// # Vertical (padrão)
/// As direções ficam em linhas, os frames em colunas:
/// ```text
/// ┌─────────────────────────────────────┐
/// │  [N][N][N][N]  ↑ Linha 0: North     │
/// │  [S][S][S][S]  ↓ Linha 1: South     │
/// │  [E][E][E][E]  → Linha 2: East      │
/// │  [W][W][W][W]  ← Linha 3: West      │
/// └─────────────────────────────────────┘
/// ```
///
/// # Horizontal
/// As direções ficam em colunas, os frames em linhas:
/// ```text
/// ┌────────────────────────────────────┐
/// │   ↑  │   ↓   │   →   │   ←   │     │
/// │──────┼───────┼───────┼───────┼─────│
/// |  [N][S][E][W]  ← Linha 0: Frame 0  │
/// │  [N][S][E][W]  ← Linha 1: Frame 1  │
/// │  [N][S][E][W]  ← Linha 2: Frame 2  │
/// │  [N][S][E][W]  ← Linha 3: Frame 3  │
/// └────────────────────────────────────┘
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameOrientation {
    /// Direções em linhas, frames em colunas (padrão)
    Vertical,
    /// Direções em colunas, frames em linhas
    Horizontal,
}

impl Default for FrameOrientation {
    fn default() -> Self {
        FrameOrientation::Vertical
    }
}

/// Offset para ajustar a posição de renderização da sprite
///
/// Este offset é subtraído da posição de renderização da sprite.
/// Exemplo: Se a sprite deve ser renderizada em (10, 10) e o offset é (2, 3),
/// a sprite será renderizada em (10 - 2, 10 - 3) = (8, 7).
#[derive(Default, Debug, Clone, Copy)]
pub struct Offset {
    pub x: i32,
    pub y: i32,
}

/// Appearance carregada do arquivo .dat
#[derive(Default, Debug, Clone)]
pub struct LoadedAppearance {
    pub id:         u32,
    pub name:       String,
    pub offset:     Offset,
    pub size:       u32,
    pub animations: HashMap<String, LoadedAnimation>,
}

/// Animação carregada
#[derive(Default, Debug, Clone)]
pub struct LoadedAnimation {
    pub name:        String,
    pub sprite_id:   u32,
    pub width:       u32,
    pub height:      u32,
    pub frames:      u32,
    pub directions:  u32,
    pub duration:    u32,
    pub orientation: FrameOrientation,
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

impl LoadedAnimation {
    /// Calcula a posição de um frame específico na spritesheet
    ///
    /// Retorna (x, y) em pixels do canto superior esquerdo do frame.
    ///
    /// # Parâmetros
    /// - `frame`: Índice do frame (0 a frames-1)
    /// - `direction`: Índice da direção (0 a directions-1)
    pub fn get_frame_position(&self, frame: u32, direction: u32) -> (u32, u32) {
        match self.orientation {
            FrameOrientation::Vertical => {
                // Direções em linhas, frames em colunas
                let x = frame * self.width;
                let y = direction * self.height;
                (x, y)
            }
            FrameOrientation::Horizontal => {
                // Direções em colunas, frames em linhas
                let x = direction * self.width;
                let y = frame * self.height;
                (x, y)
            }
        }
    }

    /// Retorna as dimensões totais da spritesheet em pixels
    ///
    /// Retorna (largura_total, altura_total)
    pub fn get_spritesheet_size(&self) -> (u32, u32) {
        match self.orientation {
            FrameOrientation::Vertical => {
                // frames colunas x directions linhas
                // Largura = frames * width_do_frame
                // Altura = directions * height_do_frame
                (self.width * self.frames, self.height * self.directions)
            }
            FrameOrientation::Horizontal => {
                // directions colunas x frames linhas
                // Largura = directions * width_do_frame
                // Altura = frames * height_do_frame
                (self.width * self.directions, self.height * self.frames)
            }
        }
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
