# Yggdrasil Appearances Manager

Sistema de compilação de sprites para o projeto Yggdrasil com recorte automático de spritesheets.

## 📋 Visão Geral

O **Appearances Manager** compila o arquivo `appearances.json` em arquivos binários otimizados:

- **`appearances.dat`**: Metadados binários de todas as appearances
- **`XXXXX.spr`**: Sprites individuais compactadas (Gzip) - uma por arquivo

### 🆕 Recorte Automático de Spritesheets

O compilador **recorta automaticamente** cada spritesheet em sprites individuais:
- Lê o PNG completo
- Divide em pedaços de `size × size` pixels
- Compacta cada sprite individualmente
- Salva como arquivos `.spr` numerados sequencialmente

**Vantagens:**
- ✅ Sprites pequenas e independentes
- ✅ Carregamento sob demanda (lazy loading)
- ✅ Menor uso de memória
- ✅ Cache eficiente por sprite
- ✅ Reutilização entre appearances

## 🎯 Estrutura do `appearances.json`

### Formato Completo

```json
{
  "version": 2,
  "appearances": [
    {
      "id": 55,
      "name": "leiden",
      "size": 64,
      "offset": { "x": 0, "y": -8 },
      "framegroups": [
        {
          "name": "idle",
          "spritesheet": "assets/characters/leiden/idle.png",
          "orientation": "horizontal",
          "animations": {
            "north": { "frame_count": 1, "duration": 1000 },
            "east": { "frame_count": 1, "duration": 1000 },
            "south": { "frame_count": 1, "duration": 1000 },
            "west": { "frame_count": 1, "duration": 1000 }
          }
        },
        {
          "name": "walk",
          "spritesheet": "assets/characters/leiden/walk.png",
          "animations": {
            "north": { "frame_count": 8, "duration": 100 },
            "east": { "frame_count": 8, "duration": 100 },
            "south": { "frame_count": 8, "duration": 100 },
            "west": { "frame_count": 8, "duration": 100 }
          }
        }
      ]
    }
  ]
}
```

### 📐 Hierarquia

```
Appearance
  ├── id: ID único
  ├── name: Nome descritivo
  ├── size: Tamanho base (32, 64, etc.)
  ├── offset: Deslocamento de renderização (opcional)
  └── framegroups: Lista de grupos de animação
        ├── name: "idle", "walk", "attack", etc.
        ├── spritesheet: Caminho do PNG original
        ├── orientation: "vertical" ou "horizontal" (padrão: vertical)
        └── animations: Mapa de direção → animação
              ├── direction: "north", "east", "south", "west", "null"
              └── Animation
                    ├── frame_count: Número de frames
                    └── duration: Milissegundos por frame (opcional)
```

## 🧩 Componentes

### Appearance
| Campo | Tipo | Descrição |
|-------|------|-----------|
| `id` | u32 | ID único da appearance |
| `name` | string | Nome descritivo |
| `size` | u32 | Tamanho base em pixels (32, 64, etc.) |
| `offset` | Offset | Deslocamento de renderização (opcional) |
| `framegroups` | FrameGroup[] | Lista de grupos de animação |

### FrameGroup
| Campo | Tipo | Descrição |
|-------|------|-----------|
| `name` | string | Nome do grupo ("idle", "walk", etc.) |
| `spritesheet` | string | Caminho do PNG que será recortado |
| `orientation` | Orientation | Layout do spritesheet (padrão: "vertical") |
| `animations` | Map | Mapa de direção para animação |

### Animation
| Campo | Tipo | Descrição |
|-------|------|-----------|
| `frame_count` | u32 | Número de frames da animação |
| `duration` | u32? | Milissegundos por frame (opcional) |

### Direções Suportadas

Use como chaves no objeto `animations`:

**Sem direção:**
- `"null"` - Para itens, efeitos omnidirecionais

**4 Direções cardinais:**
- `"north"` - Norte (↑)
- `"east"` - Leste (→)
- `"south"` - Sul (↓)
- `"west"` - Oeste (←)

**8 Direções completas:**
- `"northeast"` - Nordeste (↗)
- `"southeast"` - Sudeste (↘)
- `"southwest"` - Sudoeste (↙)
- `"northwest"` - Noroeste (↖)

## 📏 Orientação de Spritesheets

### Vertical (Padrão)

Frames em **colunas**, direções em **linhas**:

```
┌─────────────────────────────────┐
│ [N1][N2][N3][N4][N5][N6][N7][N8] │ ← Norte
├─────────────────────────────────┤
│ [E1][E2][E3][E4][E5][E6][E7][E8] │ ← Leste
├─────────────────────────────────┤
│ [S1][S2][S3][S4][S5][S6][S7][S8] │ ← Sul
├─────────────────────────────────┤
│ [W1][W2][W3][W4][W5][W6][W7][W8] │ ← Oeste
└─────────────────────────────────┘
```

**Dimensões:** `size × frame_count` × `size × num_directions`

**Exemplo:** 8 frames, 64px, 4 direções = **512×256 pixels**

### Horizontal

Direções em **colunas**, frames em **linhas**:

```
┌───────────────────┐
│ [N][E][S][W] │ ← Frame 1
├───────────────────┤
│ [N][E][S][W] │ ← Frame 2
├───────────────────┤
│ [N][E][S][W] │ ← Frame 3
└───────────────────┘
```

**Dimensões:** `size × num_directions` × `size × frame_count`

**Exemplo:** 1 frame, 64px, 4 direções = **256×64 pixels**

## 🔄 Processo de Compilação

```
appearances.json
      ↓
┌─────────────────────────────────────┐
│  1. Parse JSON                      │
│  2. Para cada FrameGroup:           │
│     ├─ Carrega o spritesheet        │
│     ├─ Valida dimensões             │
│     ├─ Recorta em sprites 64×64     │
│     ├─ Compacta cada sprite (Gzip)  │
│     └─ Salva como XXXXX.spr         │
│  3. Gera appearances.dat com IDs    │
└─────────────────────────────────────┘
      ↓
compiled/
├── appearances.dat  (metadados)
├── 00001.spr       (sprite 1)
├── 00002.spr       (sprite 2)
└── ...
```

## 🚀 Uso

### Compilar

```bash
cargo run -p yggdrasil-appearances-manager -- \
  --input assets/appearances/appearances.json \
  --output assets/appearances/compiled \
  --base-path .
```

### Argumentos

| Argumento | Curto | Descrição | Padrão |
|-----------|-------|-----------|--------|
| `--input` | `-i` | Arquivo JSON de entrada | `assets/appearances/appearances.json` |
| `--output` | `-o` | Pasta de saída | `assets/appearances/compiled` |
| `--base-path` | `-b` | Base para paths relativos | `.` |

### Exemplo de Output

```
🎮 Yggdrasil Appearances Manager
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📄 Input:  assets/appearances/appearances.json
📂 Output: assets/appearances/compiled
🗂️  Base:   .

📖 Parsing appearances.json... ✓ 1 appearances found
🔨 Compiling sprites... ✓

✅ Compilation successful!
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📊 Summary:
   • Appearances: 1
   • Total sprites: 36
   • appearances.dat: 1.5 KB
   • Total .spr files: 245 KB

📁 Output files:
   • assets/appearances/compiled/appearances.dat
   • assets/appearances/compiled/00001.spr ... 00036.spr
```

## 📂 Estrutura de Arquivos

```
assets/appearances/
├── appearances.json       # ✏️ Editável - Configuração source
├── tiles/                 # 📁 Spritesheets originais
│   ├── grass.png
│   ├── stone.png
│   └── ...
├── characters/
│   └── leiden/
│       ├── idle.png       # 256×64 (horizontal)
│       └── walk.png       # 512×256 (vertical)
└── compiled/              # ⚙️ GERADO - Não editar!
    ├── appearances.dat    # Metadados binários
    ├── 00001.spr         # Leiden idle north
    ├── 00002.spr         # Leiden idle east
    ├── 00003.spr         # Leiden idle south
    ├── 00004.spr         # Leiden idle west
    ├── 00005.spr         # Leiden walk north frame 1
    ├── 00006.spr         # Leiden walk north frame 2
    └── ...               # 36 sprites no total
```

## 📦 Exemplos Práticos

### Tile Estático (Sem Animação)

```json
{
  "id": 1,
  "name": "grass",
  "size": 32,
  "framegroups": [
    {
      "name": "default",
      "spritesheet": "assets/tiles/grass.png",
      "animations": {
        "null": { "frame_count": 1 }
      }
    }
  ]
}
```

**Spritesheet:** `grass.png` = 32×32 pixels
**Resultado:** 1 sprite (00001.spr)

---

### Item Animado (Sem Direção)

```json
{
  "id": 7,
  "name": "coin",
  "size": 32,
  "framegroups": [
    {
      "name": "spin",
      "spritesheet": "assets/items/coin_spin.png",
      "animations": {
        "null": { "frame_count": 8, "duration": 80 }
      }
    }
  ]
}
```

**Spritesheet:** `coin_spin.png` = 256×32 pixels (8 frames × 32px)
**Resultado:** 8 sprites (00007.spr a 00014.spr)

---

### Personagem Completo (Idle Horizontal + Walk Vertical)

```json
{
  "id": 55,
  "name": "leiden",
  "size": 64,
  "framegroups": [
    {
      "name": "idle",
      "spritesheet": "assets/characters/leiden/idle.png",
      "orientation": "horizontal",
      "animations": {
        "north": { "frame_count": 1, "duration": 1000 },
        "east": { "frame_count": 1, "duration": 1000 },
        "south": { "frame_count": 1, "duration": 1000 },
        "west": { "frame_count": 1, "duration": 1000 }
      }
    },
    {
      "name": "walk",
      "spritesheet": "assets/characters/leiden/walk.png",
      "animations": {
        "north": { "frame_count": 8, "duration": 100 },
        "east": { "frame_count": 8, "duration": 100 },
        "south": { "frame_count": 8, "duration": 100 },
        "west": { "frame_count": 8, "duration": 100 }
      }
    }
  ]
}
```

**Spritesheets:**
- `idle.png` = 256×64 pixels (4 dirs × 1 frame, horizontal)
- `walk.png` = 512×256 pixels (8 frames × 4 dirs, vertical)

**Resultado:** 36 sprites
- Idle: 4 sprites (1 por direção)
- Walk: 32 sprites (8 frames × 4 direções)

---

### Projétil com 8 Direções

```json
{
  "id": 302,
  "name": "magic_missile",
  "size": 24,
  "framegroups": [
    {
      "name": "fly",
      "spritesheet": "assets/projectiles/magic_missile.png",
      "animations": {
        "north": { "frame_count": 4, "duration": 80 },
        "northeast": { "frame_count": 4, "duration": 80 },
        "east": { "frame_count": 4, "duration": 80 },
        "southeast": { "frame_count": 4, "duration": 80 },
        "south": { "frame_count": 4, "duration": 80 },
        "southwest": { "frame_count": 4, "duration": 80 },
        "west": { "frame_count": 4, "duration": 80 },
        "northwest": { "frame_count": 4, "duration": 80 }
      }
    }
  ]
}
```

**Spritesheet:** `magic_missile.png` = 96×192 pixels (4 frames × 8 dirs)
**Resultado:** 32 sprites (4 frames × 8 direções)

## 🔧 Formato Binário

### `appearances.dat`

```
[Header]
version: u32
appearance_count: u32

[Para cada Appearance]
id: u32
name: String (length u32 + bytes UTF-8)
offset_x: i32
offset_y: i32
size: u32
framegroup_count: u32

  [Para cada FrameGroup]
  name: String
  animation_count: u32

    [Para cada Animation]
    has_direction: u8 (0 = sem direção, 1 = com direção)
    direction: u8 (apenas se has_direction == 1)
    sprite_id_count: u32
    sprite_ids: [u32; sprite_id_count]
    duration: u32
```

### `XXXXX.spr`

```
[Header]
width: u32
height: u32
compressed_size: u32

[Data]
compressed_pixels: Vec<u8>  # RGBA compactado com Gzip
```

## 📚 Biblioteca: `yggdrasil-appearancelib`

### Compilação

```rust
use yggdrasil_appearancelib::{parse_appearances_json, compile_appearances};

// Parse JSON
let appearances = parse_appearances_json("appearances.json")?;

// Compile
let result = compile_appearances(&appearances, ".", "output")?;

println!("✅ Compiled {} appearances into {} sprites",
    result.appearances_count, result.sprites_count);
```

### Carregamento

```rust
use yggdrasil_appearancelib::{AppearanceLoader, load_all};

// Carrega database + todas as sprites
let (database, mut loader) = load_all("compiled")?;

// Busca appearance
let leiden = database.get_appearance(55).unwrap();

// Busca framegroup
let walk = leiden.get_framegroup("walk").unwrap();

// Busca animação por direção
let walk_north = walk.get_animation(Some(Direction::North)).unwrap();

// Carrega sprites da animação
for sprite_id in &walk_north.sprite_ids {
    let sprite = loader.load_sprite(*sprite_id)?;
    // sprite.pixels contém RGBA descompactado
}
```

## ✅ Validações

O compilador verifica automaticamente:

- ✅ Spritesheets existem no caminho especificado
- ✅ Dimensões corretas baseadas em `orientation`
  - Vertical: `size × frame_count` × `size × num_directions`
  - Horizontal: `size × num_directions` × `size × frame_count`
- ✅ Formatos de imagem suportados (PNG, JPG, etc.)
- ❌ Erro detalhado com caminho e dimensões esperadas vs reais

## 🛠️ Desenvolvimento

```bash
# Build
cargo build -p yggdrasil-appearances-manager

# Run
cargo run -p yggdrasil-appearances-manager

# Tests
cargo test -p yggdrasil-appearancelib

# Watch mode
cargo watch -x "run -p yggdrasil-appearances-manager"
```

## 💡 Dicas

1. **Organize por tipo:** Separe tiles, personagens, efeitos em pastas
2. **Nomeie consistentemente:** Use padrões como `{nome}_{ação}.png`
3. **Teste dimensões:** Confira se `width = size × frames` e `height = size × directions`
4. **Use vertical por padrão:** Mais comum e natural para animações
5. **Reutilize spritesheets:** Um spritesheet pode servir múltiplas appearances
6. **Versione o JSON:** Mantenha o `appearances.json` no git, não a pasta `compiled/`

## 🐛 Erros Comuns

### "Invalid sprite dimensions"

```
Error: Invalid sprite dimensions for appearance 'assets/walk.png'
animation 'spritesheet': expected 512x256, got 256x512
```

**Solução:** Verifique a orientação! Use `"orientation": "horizontal"` se necessário.

### "Sprite not found"

```
Error: Sprite not found: assets/tiles/grass.png
```

**Solução:** Certifique-se de que o arquivo existe e o `--base-path` está correto.

### "unknown variant `null`"

```
Error: unknown variant `null`, expected one of `north`, `east`...
```

**Solução:** Use `"null"` (com aspas) como chave no JSON para sprites sem direção.

## 📄 Licença

Parte do projeto Yggdrasil Game Server.
