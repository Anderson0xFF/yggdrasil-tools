use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use yggdrasil_appearancelib::{compile_appearances, parse_appearances_json};

#[derive(Parser, Debug)]
#[command(name = "yggdrasil-appearances-manager")]
#[command(author, version, about = "Compile appearances.json into binary format", long_about = None)]
struct Args {
    /// Path to appearances.json file
    #[arg(short, long, default_value = "assets/appearances/appearances.json")]
    input: PathBuf,

    /// Output directory for compiled files
    #[arg(short, long, default_value = "assets/appearances/compiled")]
    output: PathBuf,

    /// Base path for resolving sprite paths (usually project root)
    #[arg(short, long, default_value = ".")]
    base_path: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("🎮 Yggdrasil Appearances Manager");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📄 Input:  {}", args.input.display());
    println!("📂 Output: {}", args.output.display());
    println!("🗂️  Base:   {}", args.base_path.display());
    println!();

    // Parse appearances.json
    print!("📖 Parsing appearances.json... ");
    let appearances = parse_appearances_json(&args.input)?;
    println!("✓ {} appearances found", appearances.appearances.len());

    // Compile
    print!("🔨 Compiling sprites... ");
    let result = compile_appearances(&appearances, &args.base_path, &args.output)?;
    println!("✓");

    // Summary
    println!();
    println!("✅ Compilation successful!");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📊 Summary:");
    println!("   • Appearances: {}", result.appearances_count);
    println!("   • Unique sprites: {}", result.sprites_count);
    println!(
        "   • appearances.dat: {} bytes ({:.2} KB)",
        result.dat_size,
        result.dat_size as f64 / 1024.0
    );
    println!(
        "   • Total .spr files: {} bytes ({:.2} MB)",
        result.total_spr_size,
        result.total_spr_size as f64 / 1024.0 / 1024.0
    );
    println!();
    println!("📁 Output files:");
    println!("   • {}/appearances.dat", args.output.display());
    println!(
        "   • {}/00001.spr ... {:05}.spr",
        args.output.display(),
        result.sprites_count
    );

    Ok(())
}
