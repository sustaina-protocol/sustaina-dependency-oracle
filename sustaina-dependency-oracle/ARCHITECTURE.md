# Sustaina Architecture

## Overview

Sustaina implements a cryptographically-secured mapping between GitHub repositories and Stellar addresses, enabling automated funding of open-source dependencies.

## System Components

### 1. Smart Contract (Soroban)

**Location:** `contracts/identity-registry/`

The smart contract serves as the source of truth for repository-to-address mappings.

#### Key Functions

- `initialize(oracle_pub_key)` - Initializes the contract with the trusted Oracle's public key
- `register_identity(repo_name, owner, oracle_signature)` - Registers a new repository mapping
- `update_identity(repo_name, new_owner)` - Updates an existing mapping (requires authorization)
- `resolve(repo_name)` -> Address - Resolves a repository name to its Stellar address

#### Storage

```rust
enum DataKey {
    OraclePubKey,                    // 32-byte Ed25519 public key
    RepoIdentity(BytesN<32>),        // Hash of repo → Stellar Address
}
```

#### Security Model

- **Only the Oracle can register** - Signatures must be valid Ed25519 signatures from the initialized public key
- **Only the owner can update** - Updates require `require_auth()` from the current owner
- **Immutable verification** - Signature verification happens in the contract; no trust assumed in the Oracle after verification

### 2. Oracle Service (Node.js)

**Location:** `oracle-service/`

The Oracle is a trusted service that verifies GitHub OIDC tokens and generates cryptographic signatures.

#### Endpoints

**POST /verify**

Request:
```json
{
  "oidcToken": "<JWT from GitHub Actions>",
  "repoName": "owner/repo",
  "stellarAddress": "GBZX..."
}
```

Response:
```json
{
  "status": "success",
  "repoName": "owner/repo",
  "stellarAddress": "GBZX...",
  "oracleSignature": "<hex-encoded 64-byte Ed25519 signature>"
}
```

**GET /health**

Health check endpoint returning `{ status: "ok", timestamp: "..." }`

#### Security Model

- **GitHub OIDC Verification** - Validates JWT signature against GitHub's public keys from `https://token.actions.githubusercontent.com/.well-known/jwks.json`
- **Repository Claim Matching** - Ensures the JWT's `repository` claim matches the requested repository
- **Deterministic Signing** - Signs the exact same payload the contract will verify

#### Payload Computation

```
Payload = Hash(repo_name_bytes || address_xdr_bytes)
Signature = Ed25519_Sign(Oracle_Private_Key, Payload)
```

### 3. CLI Tool (Rust)

**Location:** `cli/sustaina-cli/`

Command-line interface for analyzing dependencies and generating Split contract deployments.

#### Commands

```bash
# Analyze dependencies and show funding status
sustaina fund --manifest Cargo.toml --split-percentage 10

# Show dependency status
sustaina status --manifest Cargo.toml

# Generate Soroban CLI command for deploying Split
sustaina deploy \
  --manifest Cargo.toml \
  --split-percentage 10 \
  --from GBZX... \
  --factory-id CABC... \
  --output deploy.sh
```

#### Dependency Resolution

1. Parses `Cargo.toml` to extract direct and dev dependencies
2. Queries the Soroban Registry contract for each dependency
3. Returns list with registration status and address
4. Calculates basis points (BPS) per dependency for Split contract

### 4. Frontend (Next.js)

**Location:** `apps/explorer-ui/`

Interactive web interface for visualizing dependency graphs and generating funding configurations.

#### Features

- **Dependency Graph Visualization** - Uses React Flow to display dependency tree
- **Status Color Coding** - Green for registered, red for unregistered
- **Interactive Configuration** - Adjust split percentages and view BPS calculations
- **XDR Generation** - Generate and copy Soroban CLI commands

#### State Management

Uses Zustand for global state:
- `nodes` - Graph nodes representing dependencies
- `edges` - Graph edges showing relationships
- `selectedNodeId` - Currently selected dependency
- `setNodes`, `setEdges`, `setSelectedNodeId` - State setters

## Data Flow

### Registration Flow

```
┌─────────────────────────────────────────────────────────────┐
│ 1. Maintainer pushes GitHub Action workflow                  │
└──────────────────┬──────────────────────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────────────────────┐
│ 2. GitHub Actions generates OIDC token                       │
│    - Signed by GitHub's Ed25519 key                         │
│    - Contains: repository, actor, iss, exp, etc.            │
└──────────────────┬──────────────────────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────────────────────┐
│ 3. Workflow calls Oracle /verify endpoint                    │
│    - Sends: oidcToken, repoName, stellarAddress             │
└──────────────────┬──────────────────────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────────────────────┐
│ 4. Oracle verifies OIDC token signature                      │
│    - Fetches GitHub JWKS from .well-known/jwks.json         │
│    - Validates signature and claims                         │
│    - Checks repository claim matches requested repo         │
└──────────────────┬──────────────────────────────────────────┘
                   │
        ┌──────────┴──────────┐
        │                     │
    Invalid             Valid
        │                     │
        ▼                     ▼
    Reject              ┌─────────────────────────┐
                        │ 5. Oracle computes      │
                        │    payload hash:        │
                        │ Hash(repo || address)   │
                        └────────────┬────────────┘
                                     │
                                     ▼
                        ┌─────────────────────────┐
                        │ 6. Oracle signs with    │
                        │    Ed25519 private key  │
                        └────────────┬────────────┘
                                     │
                                     ▼
                        ┌─────────────────────────┐
                        │ 7. Return signature     │
                        │    (hex-encoded)        │
                        └────────────┬────────────┘
                                     │
                                     ▼
┌─────────────────────────────────────────────────────────────┐
│ 8. Maintainer calls smart contract register_identity()      │
│    - Sends: repo_name, owner, oracle_signature              │
└──────────────────┬──────────────────────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────────────────────┐
│ 9. Contract verifies signature                              │
│    - Recomputes payload hash                                │
│    - Verifies signature against Oracle public key           │
└──────────────────┬──────────────────────────────────────────┘
                   │
        ┌──────────┴──────────┐
        │                     │
    Invalid             Valid
        │                     │
        ▼                     ▼
    Revert              ┌─────────────────────────┐
                        │ 10. Store mapping:      │
                        │ Hash(repo) → address    │
                        │ (Persistent storage)    │
                        └────────────┬────────────┘
                                     │
                                     ▼
                        ┌─────────────────────────┐
                        │ 11. Emit registered     │
                        │     event               │
                        └─────────────────────────┘
```

### Funding Flow

```
┌──────────────────────────┐
│ Developer's Cargo.toml   │
└────────────┬─────────────┘
             │
             ▼
┌──────────────────────────────────────┐
│ sustaina-cli parse dependencies      │
└────────────┬─────────────────────────┘
             │
             ▼
┌──────────────────────────────────────┐
│ Query Registry contract via RPC      │
│ For each dep: resolve(crates.io:dep) │
└────────────┬─────────────────────────┘
             │
    ┌────────┴────────┐
    │                 │
    ▼                 ▼
┌──────────┐     ┌──────────────┐
│Registered│     │Unregistered  │
│Includes: │     │Includes:     │
│- Address │     │- None        │
│- Status  │     │- Status      │
└──────────┘     └──────────────┘
    │                 │
    └────────┬────────┘
             │
             ▼
┌──────────────────────────────────┐
│ Calculate BPS per dependency     │
│ BPS = (percentage * 100) / count │
└────────────┬────────────────────┘
             │
             ▼
┌────────────────────────────────────────┐
│ Generate Soroban Deploy Command        │
│ For each registered:                   │
│   soroban contract invoke              │
│   --id <factory_id>                    │
│   -- deploy_split                      │
│   --recipient <address> --bps <bps>   │
└────────────┬───────────────────────────┘
             │
             ▼
┌────────────────────────────────────────┐
│ Developer executes command             │
│ Deploys sustaina-core Split contract   │
│ Routes revenue to maintainers          │
└────────────────────────────────────────┘
```

## Security Considerations

### Threat Model

**Threat:** An attacker claims ownership of a high-value repository and redirects all dependency funds to their own wallet.

**Attack Vector:**
1. Attacker finds a popular repository (e.g., `torvalds/linux`)
2. Attacker somehow bypasses OIDC verification
3. Attacker registers their own Stellar address
4. All future revenue is routed to attacker

### Defense: GitHub OIDC

GitHub Actions OIDC provides cryptographic proof of repository ownership:

1. **GitHub signs the token** - JWT is signed with GitHub's Ed25519 private key
2. **Public key available** - GitHub publishes its JWKS at a well-known HTTPS URL
3. **Verifiable claims** - The `repository` claim in the JWT identifies the exact repository
4. **Time-limited** - Tokens expire in seconds
5. **No replay** - Each workflow run generates a unique token with unique JTI claim

### Defense: Oracle Signature

The Oracle service adds an additional verification layer:

1. **Private key offline** - Oracle's Ed25519 private key is never exposed
2. **Deterministic signing** - The Oracle always signs the same payload
3. **Contract verification** - The contract independently verifies the signature

### Recommendations

1. **Rotate Oracle keys regularly** - Update the Oracle public key in the contract periodically
2. **Monitor OIDC issuer** - Watch GitHub's JWKS URL for changes
3. **Audit maintenances** - Regularly review registered repositories and owners
4. **Time-limited storage** - Set ledger entry TTL to expire old entries (~6 months)

## Cryptographic Primitives

### Ed25519 Signatures

The system uses Ed25519 for both OIDC token signing (by GitHub) and identity proof signing (by Oracle):

- **Algorithm:** Edwards-curve Digital Signature Algorithm (EdDSA)
- **Key Size:** 32 bytes (256 bits)
- **Signature Size:** 64 bytes
- **Properties:** Deterministic, collision-resistant, secure against forgery

### SHA-256 Hashing

Used for:
1. Computing repository hash for storage
2. Computing payload hash for signing
3. JWKS key ID matching

## Performance Considerations

- **Contract Read:** O(1) - Direct storage lookup by repo hash
- **Contract Write:** O(1) - Single storage update
- **OIDC Verification:** ~100-200ms - Includes JWKS fetch (cached for 1 hour)
- **CLI Dependency Parsing:** O(n) where n = number of dependencies

## Deployment

### Testnet

```bash
# Deploy Oracle to testnet
docker build -t sustaina-oracle:testnet oracle-service/
docker run -e ORACLE_SECRET_KEY=... sustaina-oracle:testnet

# Deploy contract to testnet
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/sustaina_identity_registry.wasm \
  --network testnet
```

### Production Considerations

- **High Availability** - Run multiple Oracle instances with load balancer
- **Key Management** - Use HSM or secure key storage for ORACLE_SECRET_KEY
- **Rate Limiting** - Implement rate limiting on /verify endpoint
- **Logging** - Log all registration attempts (success and failure)
- **Monitoring** - Alert on unusual registration patterns
- **Disaster Recovery** - Have backup plans for key rotation

## Future Enhancements

1. **Batch Registration** - Register multiple repositories in one transaction
2. **Multi-sig Oracles** - Require signatures from multiple Oracles
3. **Time-based Expiry** - Automatically expire old mappings
4. **Dependency Graph** - Store full dependency tree on-chain
5. **Governance** - Community-controlled Registry contract
