# Sustaina Dependency Oracle

On-chain identity and dependency mapping for Rust/Soroban projects. Enable automatic, cryptographically-verified funding of open-source dependencies on Stellar.

## Overview

Sustaina solves the problem of funding open-source dependencies. Without a trusted, automated mapping between GitHub repositories and Stellar addresses, dependency funding remains manual and high-friction.

**The Solution:**
1. A GitHub Action generates a GitHub OIDC token proving repository ownership
2. The Oracle Service verifies the OIDC token against GitHub's public keys
3. The Oracle signs a cryptographic proof linking the repository to a Stellar address
4. The Smart Contract Registry stores the mapping permanently on-chain
5. Developers use the CLI to analyze dependencies and deploy revenue-sharing Splits

## Architecture

### Components

- **Smart Contract** (`contracts/identity-registry/`) - Soroban smart contract that stores and verifies identity mappings
- **Oracle Service** (`oracle-service/`) - Node.js service that verifies GitHub OIDC tokens and generates signatures
- **CLI Tool** (`cli/sustaina-cli/`) - Rust CLI for analyzing Cargo.toml and querying the registry
- **Frontend** (`apps/explorer-ui/`) - Next.js visualization for dependency graphs

### Data Flow

```
Cargo.toml Parser → Query Soroban Registry → Sustaina CLI → Deploy sustaina-core Split → Revenue Routing
```

## Quick Start

### Prerequisites

- Rust 1.70+
- Node.js 20+
- Docker (optional)

### Installation

```bash
# Clone the repository
git clone https://github.com/sustaina-protocol/sustaina-dependency-oracle.git
cd sustaina-dependency-oracle

# Install all dependencies
make install

# Build all components
make build
```

### Running Tests

```bash
# Run all tests
make test

# Run specific test suites
make contract-test
make cli-test
make oracle-build
```

### Development

#### Oracle Service

```bash
# Start in dev mode
cd oracle-service
cp .env.example .env
# Edit .env with your ORACLE_SECRET_KEY
npm run dev
```

#### Frontend

```bash
cd apps/explorer-ui
npm run dev
# Open http://localhost:3000
```

#### Contract Development

```bash
cd contracts/identity-registry
cargo build --target wasm32-unknown-unknown --release
cargo test
```

#### CLI

```bash
cd cli/sustaina-cli
cargo build --release

# Analyze dependencies
./target/release/sustaina fund --manifest Cargo.toml

# Show dependency status
./target/release/sustaina status

# Generate deployment XDR
./target/release/sustaina deploy --manifest Cargo.toml --from GCXXX --factory-id CABC
```

## Usage

### 1. Register a Repository

Add a GitHub Action to your repository:

```yaml
name: Register with Sustaina
on: [push]
jobs:
  register:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Request OIDC Token
        run: |
          TOKEN=$(curl -H "Authorization: bearer $ACTIONS_ID_TOKEN_REQUEST_TOKEN" \
            "$ACTIONS_ID_TOKEN_REQUEST_URL" | jq -r '.token')
          curl -X POST https://oracle.sustaina.dev/verify \
            -H "Content-Type: application/json" \
            -d "{
              \"oidcToken\": \"$TOKEN\",
              \"repoName\": \"${{ github.repository }}\",
              \"stellarAddress\": \"GBZX...XXXX\"
            }"
```

### 2. Analyze Dependencies

```bash
sustaina fund --manifest Cargo.toml --split-percentage 10
```

### 3. Deploy Split

```bash
sustaina deploy \
  --manifest Cargo.toml \
  --from GBZX...MAINTAINER \
  --factory-id CABC...FACTORY \
  --split-percentage 10
```

## Security Considerations

### Threat Model

An attacker could claim ownership of a high-value repository and steal all dependency funds routed via Sustaina Splits.

### Mitigation

The Oracle uses **GitHub Actions OIDC** to cryptographically verify repository ownership:

1. **Maintainer Context** - Maintainer adds a workflow to their repo that requests a JWT from GitHub
2. **Oracle Verification** - The Oracle verifies GitHub's signature using `https://token.actions.githubusercontent.com/.well-known/jwks.json`
3. **Contract Signing** - If valid, the Oracle signs a proof linking repo → Stellar address
4. **On-Chain Verification** - The contract strictly verifies the Oracle's signature

The entire chain is cryptographically verifiable and immutable.

## Configuration

### Environment Variables

#### Oracle Service

```bash
PORT=3000
ORACLE_SECRET_KEY=<base64-encoded-ed25519-secret-key>
STELLAR_RPC_URL=https://soroban-testnet.stellar.org
SOROBAN_NETWORK_PASSPHRASE="Test SDF Network ; September 2015"
```

Generate an Ed25519 key:
```bash
openssl genpkey -algorithm ed25519 -outform DER | base64
```

## Development

### Make Targets

```bash
make help              # Show all available targets
make install           # Install dependencies
make build             # Build all components
make test              # Run all tests
make lint              # Run linters
make fmt               # Format code
make clean             # Clean build artifacts
```

### Project Structure

```
sustaina-dependency-oracle/
├── contracts/              # Soroban smart contracts
│   └── identity-registry/
├── oracle-service/         # Node.js Oracle
├── cli/                    # Rust CLI
│   └── sustaina-cli/
├── apps/                   # Frontend applications
│   └── explorer-ui/
├── .github/workflows/      # CI/CD
└── Makefile               # Build automation
```

## Testing

The project includes comprehensive tests:

- **Contract Tests** - Unit tests for Soroban smart contracts
- **CLI Tests** - Dependency parsing and resolution tests
- **Oracle Tests** - OIDC verification and signing tests
- **Integration Tests** - End-to-end workflows

Run all tests with:
```bash
make test
```

## Contributing

We welcome contributions! Please:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Built for the Drips Wave mission: funding open-source dependencies
- Powered by Stellar and Soroban
- Secured with GitHub Actions OIDC

## Support

For questions and support:
- GitHub Issues: [Open an issue](https://github.com/sustaina-protocol/sustaina-dependency-oracle/issues)
- Documentation: See [ARCHITECTURE.md](./ARCHITECTURE.md) and [SECURITY.md](./SECURITY.md)
