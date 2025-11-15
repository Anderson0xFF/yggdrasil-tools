use crate::error::Result;
use crate::types::AppearancesFile;
use std::path::Path;

/// Carrega e parseia o arquivo appearances.json
pub fn parse_appearances_json<P: AsRef<Path>>(path: P) -> Result<AppearancesFile> {
    let contents = std::fs::read_to_string(path)?;
    let appearances: AppearancesFile = serde_json::from_str(&contents)?;
    Ok(appearances)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_minimal() {
        let json = r#"{
            "version": 2,
            "appearances": [
                {
                    "id": 1,
                    "name": "warrior",
                    "size": 64,
                    "framegroups": [
                        {
                            "name": "idle",
                            "spritesheet": "assets/sprites/creatures/warrior/idle.png",
                            "animations": {
                                "north": {
                                    "frame_count": 1,
                                    "duration": 1000
                                }
                            }
                        }
                    ]
                }
            ]
        }"#;

        let result: AppearancesFile = serde_json::from_str(json).unwrap();
        assert_eq!(result.version, 2);
        assert_eq!(result.appearances.len(), 1);
        assert_eq!(result.appearances[0].id, 1);
        assert_eq!(result.appearances[0].name, "warrior");
    }
}
