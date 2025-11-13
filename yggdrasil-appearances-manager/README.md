# Yggdrasil Appearances Manager

Sistema de compilação de sprites para o projeto Yggdrasil.

## 📋 Visão Geral

O **Appearances Manager** é uma ferramenta que compila o arquivo `appearances.json` (formato v2) em arquivos binários otimizados para carregamento rápido em runtime:

- **`appearances.dat`**: Arquivo binário com metadados de todas as appearances
- **`XXXXX.spr`**: Arquivos individuais contendo pixels compactados (Gzip) de cada sprite

## 🎯 Estrutura do `appearances.json` v2

```json
{
  "version": 2,
  "appearances": [
    {
      "id": 1,
      "name": "warrior",
      "size": 64,
      "animations": {
        "idle": {
          "path": "assets/sprites/creatures/warrior/idle.png",
          "frames": 1,
          "directions": 4,
          "duration": 1000
        },
        "walk": {
          "path": "assets/sprites/creatures/warrior/walk.png",
          "frames": 3,
          "directions": 4,
          "duration": 150
        }
      }
    }
  ]
}
```

### Campos

#### Appearance
- **`id`**: ID único (u32) - usado para referenciar em `items.json`
- **`name`**: Nome descritivo (string)
- **`size`**: Tamanho base do sprite em pixels (u32)
- **`animations`**: Mapa de nome → configuração de animação

#### Animation
- **`path`**: Caminho relativo do arquivo de sprite
- **`frames`**: Número de frames da animação (padrão: 1)
- **`directions`**: 0 = sem direção, 4 = N/S/E/W, 8 = 8 direções (padrão: 0)
- **`duration`**: Duração de cada frame em milissegundos (opcional)

### Layout de Sprites

#### 🎨 Ordem das Direções: **North → South → East → West**

As sprites com direções devem ser organizadas **VERTICALMENTE** (uma direção por linha):

```
┌─────────────────────────────────────┐
│  [N][N][N][N]  ← Linha 0: North    │
│  [S][S][S][S]  ← Linha 1: South    │
│  [E][E][E][E]  ← Linha 2: East     │
│  [W][W][W][W]  ← Linha 3: West     │
└─────────────────────────────────────┘
```

**Correspondência com o enum `Direction`:**
- `Direction::North` (0) → Linha 0
- `Direction::South` (1) → Linha 1
- `Direction::East` (2) → Linha 2
- `Direction::West` (3) → Linha 3

---

#### Exemplo Prático: `walk.png` (3 frames, 64px, 4 direções)

**Dimensões:** 192×256 pixels (64×3 wide, 64×4 tall)

```
┌────────┬────────┬────────┐
│ North1 │ North2 │ North3 │  y=0-63    (Direction::North)
├────────┼────────┼────────┤
│ South1 │ South2 │ South3 │  y=64-127  (Direction::South)
├────────┼────────┼────────┤
│ East1  │ East2  │ East3  │  y=128-191 (Direction::East)
├────────┼────────┼────────┤
│ West1  │ West2  │ West3  │  y=192-255 (Direction::West)
└────────┴────────┴────────┘
```

**Fórmula de Cálculo:**
```
Largura total  = size × frames       (64 × 3 = 192px)
Altura total   = size × directions   (64 × 4 = 256px)

Posição X do frame = frame_index × size
Posição Y da direção = direction_index × size
```

---

#### Sprites SEM direções (`directions: 0`)

**Exemplo:** `explosion.png` (6 frames, 64px, sem direção)

**Dimensões:** 384×64 pixels (64×6 wide, 64 tall)

```
┌────────┬────────┬────────┬────────┬────────┬────────┐
│ frame1 │ frame2 │ frame3 │ frame4 │ frame5 │ frame6 │
└────────┴────────┴────────┴────────┴────────┴────────┘
```

**Fórmula:**
```
Largura = size × frames   (64 × 6 = 384px)
Altura  = size            (64px)
```

---

#### ❌ Layout INCORRETO (não use!):

```
NÃO faça assim (direções em colunas):
[N][S][E][W]  ← frame 1
[N][S][E][W]  ← frame 2
[N][S][E][W]  ← frame 3
```

**Use sempre direções em LINHAS!**

## 🚀 Uso

### Compilar appearances

```bash
cargo run -p yggdrasil-appearances-manager -- \
  --input yggdrasil-client/assets/appearances/appearances.json \
  --output yggdrasil-client/assets/appearances/compiled \
  --base-path .
```

**Nota:** O output padrão agora é `assets/appearances/compiled/` para manter os arquivos compilados separados dos sources.

### Argumentos

- `--input, -i`: Caminho do `appearances.json` (padrão: `assets/appearances/appearances.json`)
- `--output, -o`: Pasta de destino (padrão: `assets/appearances/compiled`)
- `--base-path, -b`: Caminho base para resolver paths relativos (padrão: `.`)

### Exemplo de Output

```
🎮 Yggdrasil Appearances Manager
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📄 Input:  yggdrasil-client/assets/appearances/appearances.json
📂 Output: yggdrasil-client/assets/appearances/compiled
🗂️  Base:   .

📖 Parsing appearances.json... ✓ 4 appearances found
🔨 Compiling sprites... ✓

✅ Compilation successful!
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📊 Summary:
   • Appearances: 4
   • Unique sprites: 6
   • appearances.dat: 512 bytes (0.50 KB)
   • Total .spr files: 2.3 MB

📁 Output files:
   • assets/appearances/appearances.dat
   • assets/appearances/00001.spr ... 00006.spr
```

## 📂 Estrutura de Arquivos

```
assets/appearances/
├── appearances.json     # Fonte (JSON v2) - editado manualmente
├── tiles/              # Sprites de tiles (sources)
│   ├── 1.png
│   ├── 2.png
│   └── ...
├── creatures/          # Sprites de criaturas (sources)
│   ├── warrior/
│   │   ├── idle.png
│   │   ├── walk.png
│   │   └── attack.png
│   └── ...
└── compiled/           # Arquivos binários compilados (gerados)
    ├── appearances.dat  # Metadados binários
    ├── 00001.spr       # Sprite ID 1 (compactada)
    ├── 00002.spr       # Sprite ID 2 (compactada)
    └── ...
```

**Importante:** A pasta `compiled/` é gerada automaticamente e deve estar no `.gitignore`.

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

  [Animations] (repetido animation_count vezes)
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

## 📦 Biblioteca: `yggdrasil-appearancelib`

A lógica core está em uma biblioteca reutilizável:

```rust
use yggdrasil_appearancelib::{parse_appearances_json, compile_appearances};

// Parse JSON
let appearances = parse_appearances_json("appearances.json")?;

// Compile
let result = compile_appearances(&appearances, ".", "output")?;

println!("Compiled {} appearances", result.appearances_count);
```

## ✅ Validações

O compilador verifica automaticamente:

- ✅ Arquivos de sprite existem
- ✅ Dimensões batem com `frames × directions × size`
- ✅ Formatos de imagem suportados (PNG, JPG, etc.)
- ❌ Reporta erros claros com detalhes do problema

## 🎮 Casos de Uso

### Criatura com animações
```json
{
  "id": 1,
  "name": "warrior",
  "size": 64,
  "animations": {
    "idle": { "path": "...", "frames": 1, "directions": 4 },
    "walk": { "path": "...", "frames": 3, "directions": 4, "duration": 150 },
    "attack": { "path": "...", "frames": 4, "directions": 4, "duration": 100 }
  }
}
```

### Item estático
```json
{
  "id": 100,
  "name": "sword",
  "size": 32,
  "animations": {
    "default": { "path": "assets/sprites/items/sword.png", "frames": 1 }
  }
}
```

### Efeito sem direção
```json
{
  "id": 200,
  "name": "explosion",
  "size": 64,
  "animations": {
    "explode": { "path": "...", "frames": 6, "duration": 80 }
  }
}
```

### Projétil com direções
```json
{
  "id": 300,
  "name": "arrow",
  "size": 32,
  "animations": {
    "fly": { "path": "...", "directions": 4 }
  }
}
```

## 🛠️ Desenvolvimento

### Build
```bash
cargo build -p yggdrasil-appearances-manager
```

### Run
```bash
cargo run -p yggdrasil-appearances-manager
```

### Tests
```bash
cargo test -p yggdrasil-appearancelib
```

## 📝 Licença

Parte do projeto Yggdrasil Game Server.
