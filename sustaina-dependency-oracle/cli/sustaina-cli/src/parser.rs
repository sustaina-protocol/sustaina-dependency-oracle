use std::fs;
use std::path::Path;
use toml::Value;

use crate::errors::CliError;

/// Represents a dependency from Cargo.toml
#[derive(Debug, Clone)]
pub struct Dependency {
    pub name: String,
    pub version: String,
}

/// Extracts all dependencies from a Cargo.toml manifest.
///
/// Includes both direct dependencies and dev-dependencies.
///
/// # Arguments
/// * `path` - Path to the Cargo.toml file
///
/// # Returns
/// A vector of dependency names (crates.io format)
///
/// # Errors
/// Returns an error if the file cannot be read or parsed
pub fn extract_dependencies(path: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    if !path.exists() {
        return Err(Box::new(CliError::FileNotFound(path.to_string_lossy().to_string())));
    }

    let contents = fs::read_to_string(path)?;
    let parsed: Value = toml::from_str(&contents)?;

    let mut deps_list = Vec::new();

    // Extract regular dependencies
    if let Some(deps) = parsed.get("dependencies").and_then(|v| v.as_table()) {
        for (name, _) in deps {
            deps_list.push(format!("crates.io:{}", name));
        }
    }

    // Extract dev dependencies
    if let Some(dev_deps) = parsed.get("dev-dependencies").and_then(|v| v.as_table()) {
        for (name, _) in dev_deps {
            deps_list.push(format!("crates.io:{}", name));
        }
    }

    deps_list.sort();
    deps_list.dedup();

    Ok(deps_list)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_parse_basic_manifest() {
        let dir = TempDir::new().unwrap();
        let manifest_path = dir.path().join("Cargo.toml");
        let mut file = File::create(&manifest_path).unwrap();

        writeln!(
            file,
            r#"
[package]
name = "test"
version = "0.1.0"

[dependencies]
tokio = "1.0"
serde = "1.0"

[dev-dependencies]
assert_matches = "1.5"
            "#
        )
        .unwrap();

        let deps = extract_dependencies(&manifest_path).unwrap();
        assert!(deps.contains(&"crates.io:tokio".to_string()));
        assert!(deps.contains(&"crates.io:serde".to_string()));
        assert!(deps.contains(&"crates.io:assert_matches".to_string()));
    }
}
