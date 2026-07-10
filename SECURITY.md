# Security Analysis

## Executive Summary

Sustaina implements a cryptographically-secure mapping between GitHub repositories and Stellar addresses. The system uses GitHub Actions OIDC for ownership verification, Ed25519 signatures for cryptographic proof, and Soroban smart contracts for immutable storage.

**Security Level:** High - Cryptographic guarantees prevent unauthorized registration

## Threat Model

### Primary Threat: Identity Spoofing

**Description:** An attacker registers a high-value repository (e.g., `serde-rs/serde`) under their own Stellar address, capturing all dependency funding.

**Attack Vectors:**
1. Forge a GitHub OIDC token
2. Intercept communication with the Oracle
3. Compromise the Oracle service
4. Compromise the Oracle's private key
5. Exploit a contract vulnerability
6. Gain commit access to the target repository

### Secondary Threat: Oracle Compromise

**Description:** The Oracle service is compromised, allowing arbitrary signatures.

**Attack Vectors:**
1. Compromise the server hosting the Oracle
2. Extract the ORACLE_SECRET_KEY from environment
3. Man-in-the-middle the Oracle's external communication

### Tertiary Threat: Signature Replay

**Description:** An attacker captures a valid signature and reuses it to register a different address for the same repository.

**Attack Vectors:**
1. Capture network traffic containing a valid signature
2. Replay with a different `owner` address
3. Contract stores both addresses for the same repo

## Defense Mechanisms

### 1. GitHub OIDC Verification

**How it works:**

1. GitHub Actions generates a JWT when a workflow runs
2. The JWT contains cryptographic claims about the workflow context
3. JWT is signed by GitHub's Ed25519 private key
4. The Oracle fetches GitHub's JWKS from `https://token.actions.githubusercontent.com/.well-known/jwks.json`
5. The Oracle verifies the JWT signature using GitHub's public key
6. The Oracle checks the `repository` claim matches the requested repository

**Security Properties:**

- **Non-repudiation:** GitHub cannot deny issuing the token
- **Authenticity:** Token cannot be forged without GitHub's private key
- **Integrity:** Token claims cannot be modified without invalidating the signature
- **Freshness:** Tokens expire quickly (typically 5-10 minutes)

**Verified Claims:**

```javascript
{
  "iss": "https://token.actions.githubusercontent.com",      // Issuer
  "repository": "owner/repo",                                 // Repository identifier
  "repository_owner": "owner",                                // Repository owner
  "actor": "username",                                        // User who triggered workflow
  "run_id": "123456",                                         // Workflow run ID
  "exp": 1234567890,                                          // Expiration time
  "iat": 1234567800,                                          // Issued at
  "jti": "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"               // Unique token ID
}
```

### 2. Oracle Signature

**How it works:**

1. Oracle verifies the OIDC token (see above)
2. Oracle constructs the payload: `repo_name || stellar_address_xdr`
3. Oracle computes SHA-256 hash of the payload
4. Oracle signs the hash using its Ed25519 private key
5. Oracle returns the 64-byte signature in hexadecimal

**Security Properties:**

- **Deterministic:** Same input always produces the same signature
- **Unforgeable:** Only the Oracle (with its private key) can create valid signatures
- **Tamper-proof:** Any modification to the payload invalidates the signature
- **One-way:** Signature cannot be reversed to discover the private key

**Payload Construction:**

The payload is constructed identically in both the Oracle (Node.js) and the Contract (Rust):

```
payload = concatenate(
  bytes_of(repo_name),
  xdr_bytes_of(stellar_address)
)
payload_hash = sha256(payload)
```

This deterministic construction ensures the contract's verification will always match the Oracle's signing.

### 3. Smart Contract Verification

**How it works:**

1. Maintainer calls `register_identity(repo_name, owner, oracle_signature)`
2. Contract fetches the stored Oracle public key
3. Contract recomputes the exact payload hash
4. Contract verifies the Ed25519 signature against the public key
5. If valid, contract stores the mapping persistently
6. If invalid, contract reverts with error

**Security Properties:**

- **Atomic:** Registration either succeeds completely or fails completely
- **Immutable:** Once stored, the mapping cannot be modified without authorization
- **Auditable:** All registrations emit events that can be indexed

**Contract Code:**

```rust
pub fn register_identity(
    env: Env,
    repo_name: String,
    owner: Address,
    oracle_signature: BytesN<64>,
) -> Result<(), RegistryError> {
    let oracle_pub_key = env.storage().instance().get(&DataKey::OraclePubKey)
        .ok_or(RegistryError::NotInitialized)?;

    let payload_hash = crypto::compute_payload_hash(&env, &repo_name, &owner);

    env.crypto().ed25519_verify(
        &oracle_pub_key,
        &payload_hash.into(),
        &oracle_signature,
    );

    // ... store mapping ...
}
```

### 4. Authorization Enforcement

**Update Operations:**

Only the current owner can update an identity:

```rust
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

    // This call requires authorization from current_owner
    current_owner.require_auth();

    // ... update mapping ...
}
```

The `require_auth()` call ensures that only the current owner's signature can authorize the update.

## Attack Analysis

### Attack 1: OIDC Token Forgery

**Threat:** Attacker forges a GitHub OIDC token to claim ownership of a repository they don't own.

**Why it fails:**

- GitHub OIDC tokens are signed by GitHub's Ed25519 private key
- GitHub's private key is protected in HSM-grade hardware security modules
- The attacker would need to compromise GitHub's infrastructure to forge a token
- Likelihood: **Extremely Low** - GitHub's key security is world-class

**Mitigation:**
- Monitor for suspicious OIDC token sources
- Verify JWKS updates are coming from GitHub's official domain

### Attack 2: Man-in-the-Middle (MITM)

**Threat:** Attacker intercepts communication between the maintainer's GitHub Action and the Oracle, modifying the request or response.

**Why it fails:**

- Communication uses HTTPS, which is encrypted and authenticated
- Certificate pinning could be added for additional protection
- The signature includes the specific `owner` address; changing it would require re-signing

**Mitigation:**
- Use HTTPS for all communication
- Implement certificate pinning in clients
- Add logging/monitoring for unusual requests

### Attack 3: Oracle Private Key Compromise

**Threat:** Attacker gains access to ORACLE_SECRET_KEY and can sign arbitrary payloads.

**Why it fails:**

- This is a catastrophic compromise requiring physical security failure
- The impact would affect all future registrations until key rotation
- The key should be stored in an HSM or secure key management service

**Mitigation:**
- Store ORACLE_SECRET_KEY in AWS KMS, Azure Key Vault, or Hardware Security Module
- Rotate keys periodically (e.g., quarterly)
- Implement key rotation ceremony with multi-party authorization
- Monitor access logs for unauthorized key access
- Use separate keys for testnet and mainnet

### Attack 4: Signature Replay

**Threat:** Attacker captures a valid signature from a registration and replays it with a different address.

**Why it fails:**

- The signature is bound to a specific `owner` address
- The contract computes the payload hash including the address
- A replayed signature with a different address will have an invalid hash
- Signature verification will fail

**Example:**
```
Valid signature for:
  repo="owner/repo"
  owner="GBZX...VALID"

Attacker tries to use same signature with:
  repo="owner/repo"
  owner="GABC...ATTACKER"

payload_hash_original = sha256(repo || GBZX...VALID)
payload_hash_replay = sha256(repo || GABC...ATTACKER)
payload_hash_replay != payload_hash_original
=> Signature verification fails
```

**Mitigation:**
- This is automatically prevented by the contract design
- No additional mitigations needed

### Attack 5: Contract Logic Exploit

**Threat:** A vulnerability in the contract logic allows unauthorized registration.

**Why it fails:**

- The contract is written in Rust with strict type checking
- Soroban enforces memory safety and prevents buffer overflows
- The contract has been tested with unit tests and should undergo audit

**Mitigation:**
- Conduct professional smart contract audit
- Implement fuzz testing
- Use formal verification if possible
- Have a bug bounty program

### Attack 6: Compromised Repository

**Threat:** Attacker gains commit access to the target repository and runs a malicious workflow.

**Why it fails:**

- The attacker still needs their own Stellar address
- Even if they register a malicious address, only it will receive funds for that repo
- The repository owner can immediately update the address back
- This doesn't affect other repositories

**Mitigation:**
- Encourage maintainers to use branch protection rules
- Encourage 2FA for GitHub account security
- Educate maintainers about phishing attacks
- Suggest regular reviews of registered addresses

## Compliance & Best Practices

## Secure Coding Practices

- Input validation on all endpoints
- No hardcoded secrets
- Proper error handling without information disclosure
- Use of standard cryptographic libraries (jose, Stellar SDK)
- Type safety (TypeScript, Rust)

## Dependency Management

- Lock files for reproducible builds (package-lock.json, Cargo.lock)
- Regular dependency updates
- Scanning for known vulnerabilities

## Logging & Monitoring

- Structured logging with pino
- Log all registration attempts (success and failure)
- No sensitive data in logs (ORACLE_SECRET_KEY never logged)

## Environment Configuration

- .env.example provided (no secrets in repo)
- Example shows all required variables
- Clear documentation on key generation

## Recommended Security Measures

### Immediate (Phase 1)

1. Code review by security experts
2. Static analysis with SAST tools
3. Dependency vulnerability scanning
4. Implement rate limiting on /verify endpoint
5. Add request logging and monitoring

### Short-term (Phase 2)

1. Use Hardware Security Module (HSM) for ORACLE_SECRET_KEY
2. Implement key rotation ceremony and procedure
3. Set up bug bounty program
4. Conduct professional smart contract audit
5. Implement certificate pinning in clients

### Long-term (Phase 3)

1. Multi-signature Oracle (require 2+ signatures)
2. Threshold cryptography (m-of-n keys)
3. Decentralized Oracle network
4. Governance token for Registry updates
5. DAO-controlled contract upgrades

## Incident Response

### Scenario: Oracle Key Compromised

1. **Immediate:** Revoke the compromised key in the smart contract
2. **Deploy:** New Oracle instance with new Ed25519 key
3. **Update:** Call `initialize()` with new Oracle public key
4. **Review:** Audit all registrations since compromise
5. **Revert:** Any suspicious registrations

### Scenario: Contract Vulnerability Discovered

1. **Pause:** Disable /verify endpoint to prevent new registrations
2. **Investigate:** Determine impact and scope
3. **Fix:** Deploy patched contract
4. **Migrate:** Transfer state to new contract if needed
5. **Resume:** Re-enable endpoint with fixed contract

## Testing

Sustaina includes comprehensive tests:

- Unit tests for cryptographic functions
- Integration tests for registration flow
- OIDC token verification tests
- Signature computation tests

Run tests with:
```bash
make test
```

## Conclusion

Sustaina provides strong cryptographic security for mapping GitHub repositories to Stellar addresses. The use of GitHub OIDC for ownership verification and Ed25519 signatures for integrity creates a trustworthy system.

The main security dependencies are:

1. **GitHub's security** - OIDC tokens and JWKS integrity
2. **Oracle's security** - Protection of ORACLE_SECRET_KEY
3. **Contract correctness** - Smart contract code quality

With proper operational security practices (HSM-based key storage, regular audits, monitoring), Sustaina can safely handle funding for open-source dependencies.
