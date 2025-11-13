pub mod compiler;
pub mod error;
pub mod loaded_types;
pub mod loader;
pub mod parser;
pub mod sprite;
pub mod types;

// Compiler exports
pub use compiler::{compile_appearances, CompilationResult};

// Loader exports
pub use loaded_types::{AppearanceDatabase, LoadedAnimation, LoadedAppearance, LoadedSprite};
pub use loader::{load_all, load_database_only, AppearanceLoader};

// Common exports
pub use error::{AppearanceError, Result};
pub use parser::parse_appearances_json;
pub use types::{Animation, Appearance, AppearancesFile, SpriteData, SpriteMetadata};
