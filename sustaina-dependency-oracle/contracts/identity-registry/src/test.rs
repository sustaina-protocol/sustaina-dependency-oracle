#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Env, BytesN, String};

#[test]
fn test_initialize_contract() {
    let env = Env::default();
    let oracle_pub_key = BytesN::from_array(&env, &[1; 32]);

    let registry_id = env.register_contract(None, IdentityRegistry);
    let client = IdentityRegistryClient::new(&env, &registry_id);

    let result = client.initialize(&oracle_pub_key);
    assert!(result.is_ok());
}

#[test]
fn test_initialize_twice_fails() {
    let env = Env::default();
    let oracle_pub_key = BytesN::from_array(&env, &[1; 32]);

    let registry_id = env.register_contract(None, IdentityRegistry);
    let client = IdentityRegistryClient::new(&env, &registry_id);

    client.initialize(&oracle_pub_key).expect("First init should succeed");

    let result = client.try_initialize(&oracle_pub_key);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().unwrap(), RegistryError::AlreadyInitialized);
}

#[test]
fn test_resolve_uninitialized_fails() {
    let env = Env::default();
    let registry_id = env.register_contract(None, IdentityRegistry);
    let client = IdentityRegistryClient::new(&env, &registry_id);

    let repo_name = String::from_str(&env, "crates.io:soroban-sdk");
    let result = client.try_resolve(&repo_name);
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().unwrap(), RegistryError::NotInitialized);
}

#[test]
fn test_resolve_nonexistent_identity_fails() {
    let env = Env::default();
    let oracle_pub_key = BytesN::from_array(&env, &[1; 32]);

    let registry_id = env.register_contract(None, IdentityRegistry);
    let client = IdentityRegistryClient::new(&env, &registry_id);

    client.initialize(&oracle_pub_key).expect("Init should succeed");

    let repo_name = String::from_str(&env, "nonexistent/repo");
    let result = client.try_resolve(&repo_name);
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().unwrap(), RegistryError::IdentityNotFound);
}
