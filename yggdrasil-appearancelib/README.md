# Yggdrasil Appearance Library

Biblioteca core para compilação e carregamento de sprites do Yggdrasil Game Server.

## 📦 Funcionalidades

- ✅ **Compilação**: Converte `appearances.json` em arquivos binários otimizados
- ✅ **Carregamento**: Lê arquivos `.dat` e `.spr` compilados
- ✅ **Cache**: Sistema de cache automático para sprites carregadas
- ✅ **Compressão**: Pixels compactados com Gzip para economia de espaço
- ✅ **Validação**: Verifica dimensões e formatos automaticamente

---

## 🎨 Layout de Sprites

### Ordem das Direções: **North → South → East → West**

As sprites com direções devem ser organizadas **VERTICALMENTE** (uma direção por linha):

```
┌─────────────────────────────────────┐
│  [N][N][N][N]  ← Linha 0: North    │
│  [S][S][S][S]  ← Linha 1: South    │
│  [E][E][E][E]  ← Linha 2: East     │
│  [W][W][W][W]  ← Linha 3: West     │
└─────────────────────────────────────┘
```

**Correspondência com `Direction` enum:**
- `Direction::North` (0) → Linha 0
- `Direction::South` (1) → Linha 1
- `Direction::East` (2) → Linha 2
- `Direction::West` (3) → Linha 3

### Exemplo Prático: walk.png (3 frames, 64px)

**Dimensões:** 192×256 pixels (width: 64×3, height: 64×4)

```
┌────────┬────────┬────────┐
│ North1 │ North2 │ North3 │  y=0-63    (Direction::North = 0)
├────────┼────────┼────────┤
│ South1 │ South2 │ South3 │  y=64-127  (Direction::South = 1)
├────────┼────────┼────────┤
│ East1  │ East2  │ East3  │  y=128-191 (Direction::East = 2)
├────────┼────────┼────────┤
│ West1  │ West2  │ West3  │  y=192-255 (Direction::West = 3)
└────────┴────────┴────────┘
```

**Fórmulas de Cálculo:**
```rust
// Dimensões da sprite
width  = size × frames       // 64 × 3 = 192px
height = size × directions   // 64 × 4 = 256px

// Posição de um frame específico
frame_x = frame_index × size
frame_y = direction_index × size

// Exemplo: East direction (index=2), frame 2 (index=1), size=64
x = 1 × 64 = 64px
y = 2 × 64 = 128px
```

---

## 🚀 Uso Básico

### Compilar Appearances

```rust
use yggdrasil_appearancelib::{parse_appearances_json, compile_appearances};

// Parse JSON
let appearances = parse_appearances_json("appearances.json")?;

// Compile para binários
let result = compile_appearances(&appearances, ".", "output/compiled")?;

println!("Compiled {} appearances", result.appearances_count);
println!("Generated {} sprite files", result.sprites_count);
```

### Carregar Appearances (Lazy Loading)

```rust
use yggdrasil_appearancelib::load_database_only;

// Carrega apenas metadados (rápido)
let (database, mut loader) = load_database_only("assets/appearances/compiled")?;

println!("Loaded {} appearances", database.count());

// Carrega sprites sob demanda
if let Some(appearance) = database.get_appearance(1) {
    for animation in appearance.all_animations() {
        let sprite = loader.load_sprite(animation.sprite_id)?;
        println!("Loaded sprite: {}x{}", sprite.width, sprite.height);
    }
}
```

### Carregar Tudo de Uma Vez

```rust
use yggdrasil_appearancelib::load_all;

// Carrega database + pré-carrega todos os sprites
let (database, loader) = load_all("assets/appearances/compiled")?;

println!("Appearances: {}", database.count());
println!("Sprites cached: {}", loader.cached_sprite_count());
```

---

## 📊 API Completa

### Structs de Carregamento

#### `AppearanceDatabase`
```rust
pub struct AppearanceDatabase {
    pub version: u32,
    pub appearances: HashMap<u32, LoadedAppearance>,
}

impl AppearanceDatabase {
    pub fn get_appearance(&self, id: u32) -> Option<&LoadedAppearance>;
    pub fn all_appearances(&self) -> impl Iterator<Item = &LoadedAppearance>;
    pub fn count(&self) -> usize;
}
```

#### `LoadedAppearance`
```rust
pub struct LoadedAppearance {
    pub id: u32,
    pub name: String,
    pub size: u32,
    pub animations: HashMap<String, LoadedAnimation>,
}

impl LoadedAppearance {
    pub fn get_animation(&self, name: &str) -> Option<&LoadedAnimation>;
    pub fn all_animations(&self) -> impl Iterator<Item = &LoadedAnimation>;
    pub fn animation_names(&self) -> impl Iterator<Item = &String>;
}
```

#### `LoadedAnimation`
```rust
pub struct LoadedAnimation {
    pub name: String,
    pub sprite_id: u32,
    pub width: u32,
    pub height: u32,
    pub frames: u32,
    pub directions: u32,
    pub duration: u32, // ms por frame (0 = estático)
}
```

#### `LoadedSprite`
```rust
pub struct LoadedSprite {
    pub sprite_id: u32,
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>, // RGBA descompactado
}
```

### AppearanceLoader

```rust
impl AppearanceLoader {
    // Criação
    pub fn new<P: AsRef<Path>>(base_path: P) -> Self;

    // Carregamento
    pub fn load_database(&mut self) -> Result<AppearanceDatabase>;
    pub fn load_sprite(&mut self, sprite_id: u32) -> Result<&LoadedSprite>;

    // Pré-carregamento
    pub fn preload_sprites(&mut self, sprite_ids: &[u32]) -> Result<()>;
    pub fn preload_appearance_sprites(&mut self, appearance: &LoadedAppearance) -> Result<()>;

    // Cache
    pub fn get_cached_sprite(&self, sprite_id: u32) -> Option<&LoadedSprite>;
    pub fn clear_sprite_cache(&mut self);
    pub fn cached_sprite_count(&self) -> usize;
    pub fn cache_size_bytes(&self) -> usize;
}
```

### Funções Helper

```rust
// Carrega database + pré-carrega todos os sprites
pub fn load_all<P: AsRef<Path>>(base_path: P)
    -> Result<(AppearanceDatabase, AppearanceLoader)>;

// Carrega apenas database (lazy loading)
pub fn load_database_only<P: AsRef<Path>>(base_path: P)
    -> Result<(AppearanceDatabase, AppearanceLoader)>;
```

---

## 🎯 Exemplos Práticos

### Exemplo 1: Sistema de Renderização

```rust
use yggdrasil_appearancelib::load_database_only;

struct SpriteRenderer {
    database: AppearanceDatabase,
    loader: AppearanceLoader,
}

impl SpriteRenderer {
    fn new() -> Result<Self> {
        let (database, loader) = load_database_only("assets/appearances/compiled")?;
        Ok(Self { database, loader })
    }

    fn render_entity(&mut self, appearance_id: u32, animation: &str, direction: u32) -> Result<()> {
        let appearance = self.database.get_appearance(appearance_id)
            .ok_or("Appearance not found")?;

        let anim = appearance.get_animation(animation)
            .ok_or("Animation not found")?;

        let sprite = self.loader.load_sprite(anim.sprite_id)?;

        // Calcula posição do frame na sprite
        let frame_y = direction * appearance.size;

        // Renderiza sprite.pixels (RGBA)
        // ...

        Ok(())
    }
}
```

### Exemplo 2: Calcular Posição de Frame

```rust
use yggdrasil_appearancelib::LoadedAnimation;

fn calculate_frame_rect(
    animation: &LoadedAnimation,
    frame_index: u32,
    direction_index: u32,
    size: u32,
) -> (u32, u32, u32, u32) {
    let x = frame_index * size;
    let y = direction_index * size;

    (x, y, size, size)
}

// Exemplo: East direction (2), frame 1, size 64
let (x, y, w, h) = calculate_frame_rect(&animation, 1, 2, 64);
// Resultado: (64, 128, 64, 64)
```

### Exemplo 3: Animação com Timer

```rust
struct AnimatedSprite {
    animation: LoadedAnimation,
    current_frame: u32,
    elapsed_ms: u32,
}

impl AnimatedSprite {
    fn update(&mut self, delta_ms: u32) {
        if self.animation.duration == 0 {
            return; // Estático
        }

        self.elapsed_ms += delta_ms;

        if self.elapsed_ms >= self.animation.duration {
            self.elapsed_ms = 0;
            self.current_frame = (self.current_frame + 1) % self.animation.frames;
        }
    }

    fn get_current_frame_rect(&self, direction: u32, size: u32) -> (u32, u32, u32, u32) {
        let x = self.current_frame * size;
        let y = direction * size;
        (x, y, size, size)
    }
}
```

---

## 🔧 Formato Binário

### `appearances.dat`
```
[Header]
- version: u32
- appearance_count: u32

[Appearances] (repetido appearance_count vezes)
- id: u32
- name_length: u32
- name: String (UTF-8)
- size: u32
- animation_count: u32
  [Animations]
  - anim_name_length: u32
  - anim_name: String (UTF-8)
  - sprite_id: u32
  - width: u32
  - height: u32
  - frames: u32
  - directions: u32
  - duration: u32
```

### `XXXXX.spr`
```
[Header]
- width: u32
- height: u32
- compressed_size: u32

[Data]
- compressed_pixels: Vec<u8>  # RGBA compactado (Gzip)
```

---

## 📝 Performance

### Estratégias de Loading

1. **Lazy Loading** (recomendado para mobile/baixa memória):
   ```rust
   let (db, loader) = load_database_only("assets/appearances/compiled")?;
   // Carrega sprites sob demanda
   ```

2. **Eager Loading** (recomendado para desktop/alta memória):
   ```rust
   let (db, loader) = load_all("assets/appearances/compiled")?;
   // Todas as sprites já carregadas
   ```

3. **Hybrid Loading** (recomendado para jogos médios):
   ```rust
   let (db, mut loader) = load_database_only("assets/appearances/compiled")?;

   // Pré-carrega apenas o essencial
   loader.preload_sprites(&essential_sprite_ids)?;

   // Resto é lazy
   ```

### Benchmarks Típicos

- **Load database**: ~1-5ms (para 100 appearances)
- **Load sprite (cached)**: ~0.1μs (lookup no HashMap)
- **Load sprite (disk)**: ~1-10ms (depende do tamanho)
- **Decompression**: ~2-20ms (depende do tamanho)

### Tamanho em Memória

```
Appearance metadata: ~100 bytes
Animation metadata: ~50 bytes
Sprite (32×32 RGBA): ~4 KB (descompactado)
Sprite (64×64 RGBA): ~16 KB (descompactado)
Sprite (64×256 RGBA): ~64 KB (descompactado)
```

---

## 🧪 Testes

```bash
# Rodar testes
cargo test -p yggdrasil-appearancelib

# Rodar exemplo
cargo run --example loader_example
```

---

## 📦 Dependências

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
image = "0.25"
flate2 = "1.0"
byteorder = "1.5"
thiserror = "1.0"
```

---

## 📄 Licença

Parte do projeto Yggdrasil Game Server.
