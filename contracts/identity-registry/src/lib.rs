#![no_std]

use soroban_sdk::{contract, contractimpl, contracterror, Address, BytesN, Env, String, Bytes};

mod crypto;

/// Registry errors indicating operational failures or security violations.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum RegistryError {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    InvalidSignature = 3,
    IdentityNotFound = 4,
    UnauthorizedUpdate = 5,
}

/// Storage keys for contract state.
#[soroban_sdk::contracttype]
pub enum DataKey {
    OraclePubKey,
    RepoIdentity(BytesN<32>),
}

/// Identity registry for mapping GitHub repositories to Stellar addresses.
#[contract]
pub struct IdentityRegistry;

#[contractimpl]
impl IdentityRegistry {
    /// Initializes the contract with the trusted Node.js Oracle's Ed25519 public key.
    ///
    /// # Arguments
    /// * `env` - The contract environment
    /// * `oracle_pub_key` - The Ed25519 public key of the Oracle service
    ///
    /// # Errors
    /// Returns `AlreadyInitialized` if the contract is already initialized
    pub fn initialize(env: Env, oracle_pub_key: BytesN<32>) -> Result<(), RegistryError> {
        if env.storage().instance().has(&DataKey::OraclePubKey) {
            return Err(RegistryError::AlreadyInitialized);
        }

        env.storage().instance().set(&DataKey::OraclePubKey, &oracle_pub_key);
        Ok(())
    }

    /// Registers a new identity mapping a repository to its maintainer's Stellar address.
    ///
    /// This function verifies the Oracle's Ed25519 signature over the payload hash,
    /// ensuring only authorized maintainers can register their addresses.
    ///
    /// # Arguments
    /// * `env` - The contract environment
    /// * `repo_name` - The GitHub repository identifier (e.g., "org/repo")
    /// * `owner` - The Stellar address of the repository maintainer
    /// * `oracle_signature` - The Ed25519 signature from the Oracle service
    ///
    /// # Errors
    /// * `NotInitialized` - Contract not yet initialized with Oracle public key
    /// * `InvalidSignature` - Signature verification failed
    pub fn register_identity(
        env: Env,
        repo_name: String,
        owner: Address,
        oracle_signature: BytesN<64>,
    ) -> Result<(), RegistryError> {
        let oracle_pub_key: BytesN<32> = env
            .storage()
            .instance()
            .get(&DataKey::OraclePubKey)
            .ok_or(RegistryError::NotInitialized)?;

        // Reconstruct the exact payload the Oracle signed
        let payload_hash = crypto::compute_payload_hash(&env, &repo_name, &owner);

        // Verify Ed25519 signature
        env.crypto().ed25519_verify(
            &oracle_pub_key,
            &payload_hash.into(),
            &oracle_signature,
        );

        // Store the mapping persistently
        let repo_hash = env.crypto().sha256(&repo_name.clone().into());
        env.storage().persistent().set(&DataKey::RepoIdentity(repo_hash.clone()), &owner);
        
        // Set ledger entry lifetime to ~6 months (3,110,400 ledgers)
        env.storage().persistent().bump(&DataKey::RepoIdentity(repo_hash), 3_110_400);

        env.events().publish(("identity", "registered"), (repo_name, owner));
        Ok(())
    }

    /// Updates an existing identity, allowing the current owner to change the destination address.
    ///
    /// Only the currently registered owner can call this function (enforced via `require_auth()`).
    ///
    /// # Arguments
    /// * `env` - The contract environment
    /// * `repo_name` - The GitHub repository identifier
    /// * `new_owner` - The new Stellar address to receive funds
    ///
    /// # Errors
    /// * `IdentityNotFound` - Repository not registered
    /// * Authorization fails if caller is not the current owner
    pub fn update_identity(
        env: Env,
        repo_name: String,
        new_owner: Address,
    ) -> Result<(), RegistryError> {
        let repo_hash = env.crypto().sha256(&repo_name.clone().into());

        let current_owner: Address = env
            .storage()
            .persistent()
            .get(&DataKey::RepoIdentity(repo_hash.clone()))
            .ok_or(RegistryError::IdentityNotFound)?;

        // Enforce authorization - only current owner can update
        current_owner.require_auth();

        env.storage().persistent().set(&DataKey::RepoIdentity(repo_hash.clone()), &new_owner);
        env.storage().persistent().bump(&DataKey::RepoIdentity(repo_hash), 3_110_400);

        env.events().publish(("identity", "updated"), (repo_name, new_owner));
        Ok(())
    }

    /// Resolves a repository name to its registered Stellar address.
    ///
    /// # Arguments
    /// * `env` - The contract environment
    /// * `repo_name` - The GitHub repository identifier
    ///
    /// # Returns
    /// The Stellar address registered for this repository
    ///
    /// # Errors
    /// Returns `IdentityNotFound` if the repository is not registered
    pub fn resolve(env: Env, repo_name: String) -> Result<Address, RegistryError> {
        let repo_hash = env.crypto().sha256(&repo_name.into());
        let owner: Address = env
            .storage()
            .persistent()
            .get(&DataKey::RepoIdentity(repo_hash))
            .ok_or(RegistryError::IdentityNotFound)?;
        Ok(owner)
    }
}
