use yggdrasil_appearancelib::{load_all, load_database_only};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Yggdrasil Appearance Loader Example ===\n");

    // Exemplo 1: Carregar apenas o database (lazy loading)
    println!("📖 Example 1: Loading database only (lazy loading sprites)");
    println!("─────────────────────────────────────────────────────────");

    let (database, mut loader) = load_database_only("assets/appearances/compiled")?;

    println!("✓ Loaded database version: {}", database.version);
    println!("✓ Total appearances: {}", database.count());
    println!();

    // Lista todas as appearances
    println!("Appearances loaded:");
    for appearance in database.all_appearances() {
        println!(
            "  • ID {}: {} (size: {}px)",
            appearance.id, appearance.name, appearance.size
        );

        for (anim_name, animation) in &appearance.animations {
            println!(
                "    └─ {}: {}x{}, {} frames, {} directions, {}ms",
                anim_name,
                animation.width,
                animation.height,
                animation.frames,
                animation.directions,
                if animation.duration > 0 {
                    format!("{}", animation.duration)
                } else {
                    "static".to_string()
                }
            );
        }
    }
    println!();

    // Exemplo 2: Carregar sprite sob demanda
    if let Some(appearance) = database.get_appearance(1) {
        println!("📦 Example 2: Loading sprite on-demand");
        println!("─────────────────────────────────────────────────────────");
        println!("Loading sprites for appearance: {}", appearance.name);

        for (anim_name, animation) in &appearance.animations {
            let sprite = loader.load_sprite(animation.sprite_id)?;
            println!(
                "  ✓ Loaded sprite {} ({}): {}x{}, {} bytes",
                sprite.sprite_id,
                anim_name,
                sprite.width,
                sprite.height,
                sprite.pixels.len()
            );
        }

        println!();
        println!("Cache stats:");
        println!("  • Sprites cached: {}", loader.cached_sprite_count());
        println!(
            "  • Cache size: {:.2} MB",
            loader.cache_size_bytes() as f64 / 1024.0 / 1024.0
        );
        println!();
    }

    // Exemplo 3: Carregar tudo de uma vez
    println!("🚀 Example 3: Loading everything at once");
    println!("─────────────────────────────────────────────────────────");

    let (database2, loader2) = load_all("assets/appearances/compiled")?;

    println!("✓ Database version: {}", database2.version);
    println!("✓ Appearances: {}", database2.count());
    println!("✓ Sprites pre-cached: {}", loader2.cached_sprite_count());
    println!(
        "✓ Total cache size: {:.2} MB",
        loader2.cache_size_bytes() as f64 / 1024.0 / 1024.0
    );
    println!();

    // Exemplo 4: Buscar appearance e animação específicas
    println!("🔍 Example 4: Querying specific appearance");
    println!("─────────────────────────────────────────────────────────");

    if let Some(warrior) = database2.get_appearance(1) {
        println!("Found appearance: {}", warrior.name);

        if let Some(idle_anim) = warrior.get_animation("idle") {
            println!("  Animation 'idle':");
            println!("    • Sprite ID: {}", idle_anim.sprite_id);
            println!("    • Dimensions: {}x{}", idle_anim.width, idle_anim.height);
            println!("    • Frames: {}", idle_anim.frames);
            println!("    • Directions: {}", idle_anim.directions);

            if let Some(sprite) = loader2.get_cached_sprite(idle_anim.sprite_id) {
                println!("    • Pixel data: {} RGBA bytes", sprite.pixels.len());
                println!("    • Expected pixels: {}", idle_anim.width * idle_anim.height * 4);
            }
        }

        println!();
        println!("All animations for '{}':", warrior.name);
        for anim_name in warrior.animation_names() {
            println!("  • {}", anim_name);
        }
    }

    println!();
    println!("✅ All examples completed successfully!");

    Ok(())
}
