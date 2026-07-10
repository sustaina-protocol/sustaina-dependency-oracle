use soroban_sdk::{Address, Bytes, BytesN, Env, String};

/// Computes the SHA-256 hash of the payload that the Oracle signs.
///
/// This ensures deterministic hashing across the Oracle service and the contract.
/// The payload is constructed by concatenating:
/// 1. The repository name bytes
/// 2. The serialized Stellar address (as XDR)
///
/// # Arguments
/// * `env` - The contract environment
/// * `repo_name` - The repository identifier
/// * `owner` - The Stellar address to receive funds
///
/// # Returns
/// A 32-byte SHA-256 hash of the concatenated payload
pub fn compute_payload_hash(env: &Env, repo_name: &String, owner: &Address) -> BytesN<32> {
    let mut payload = Bytes::new(env);

    // Append repository name as raw bytes
    payload.append(&repo_name.clone().into());

    // Serialize Stellar address to XDR bytes for deterministic hashing
    let owner_bytes = owner.to_xdr(env);
    payload.append(&owner_bytes);

    // Return SHA-256 hash of the complete payload
    env.crypto().sha256(&payload)
}
