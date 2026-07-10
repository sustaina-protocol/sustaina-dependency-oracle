use serde::{Deserialize, Serialize};

use crate::errors::CliError;

/// Result of resolving a dependency's on-chain address
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedDependency {
    pub name: String,
    pub is_registered: bool,
    pub address: String,
}

/// Resolves a list of dependencies by querying the Sustaina Registry contract.
///
/// This function would typically make RPC calls to the Soroban registry contract
/// to look up addresses for each dependency.
///
/// # Arguments
/// * `deps` - List of dependency names (crates.io:name format)
/// * `registry_rpc` - Soroban RPC endpoint URL
///
/// # Returns
/// A vector of resolved dependencies with registration status
pub async fn resolve_addresses(
    deps: &[String],
    _registry_rpc: &str,
) -> Result<Vec<ResolvedDependency>, Box<dyn std::error::Error>> {
    let mut resolved = Vec::new();

    // TODO: Implement actual RPC resolution via soroban-rs
    // For now, return mock data with specific known dependencies
    let known_addresses = vec![
        ("crates.io:soroban-sdk", true, "GBUQWP3BOUZX34ULNQG23RQ6F4MAGAFE2QZENA3MWPosted"),
        ("crates.io:serde", false, ""),
    ];

    for dep in deps {
        let found = known_addresses.iter().find(|k| k.0 == dep);

        if let Some((_, is_registered, address)) = found {
            resolved.push(ResolvedDependency {
                name: dep.clone(),
                is_registered: *is_registered,
                address: address.to_string(),
            });
        } else {
            // Unknown dependency - not registered
            resolved.push(ResolvedDependency {
                name: dep.clone(),
                is_registered: false,
                address: String::new(),
            });
        }
    }

    if resolved.is_empty() {
        return Err(Box::new(CliError::NoRegisteredDependencies));
    }

    Ok(resolved)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_resolve_dependencies() {
        let deps = vec!["crates.io:soroban-sdk".to_string()];
        let resolved = resolve_addresses(&deps, "https://example.com").await;

        assert!(resolved.is_ok());
        let result = resolved.unwrap();
        assert!(!result.is_empty());
    }
}
