# Quick Start Guide

Get the Sustaina Dependency Oracle running in minutes.

## 5-Minute Setup

### 1. Install Dependencies
```bash
make install
```

### 2. Run Tests
```bash
make test
```

### 3. Start Local Development

Terminal 1 - Oracle Service:
```bash
cd oracle-service
npm run dev
# Listens on http://localhost:3000
```

Terminal 2 - Frontend:
```bash
cd apps/explorer-ui
npm run dev
# Open http://localhost:3000
```

Terminal 3 - CLI:
```bash
cd cli/sustaina-cli
cargo build --release
./target/release/sustaina fund --manifest ../../Cargo.toml
```

## Common Commands

```bash
# Build everything
make build

# Run all tests
make test

# Code quality checks
make check-all

# Format code
make fmt

# Lint code
make lint

# View help
make help
```

## Project Structure at a Glance

```
sustaina-dependency-oracle/
├── contracts/          → Smart contract (Soroban)
├── oracle-service/     → Node.js Oracle verification
├── cli/               → Rust CLI tool
├── apps/explorer-ui/  → Next.js frontend
└── docs/              → 12+ documentation files
```

## Documentation Map

Start with these files in order:

1. **README.md** - Project overview
2. **ARCHITECTURE.md** - How it works
3. **DEVELOPMENT.md** - Development setup
4. **DEPLOYMENT.md** - How to deploy

For specific topics:
- **SECURITY.md** - Security analysis
- **TESTING.md** - How to test
- **CONTRIBUTING.md** - How to contribute

## Deployment

### Testnet

```bash
# 1. Set environment
export STELLAR_NETWORK=testnet
export SOROBAN_RPC_URL=https://soroban-testnet.stellar.org

# 2. Follow DEPLOYMENT.md for detailed steps
```

### Production

See **DEPLOYMENT.md** for complete procedures including:
- Contract deployment
- Oracle service setup
- Key management
- Monitoring configuration

## Key Endpoints

### Oracle Service
- `POST /verify` - Verify OIDC and generate signature
- `GET /health` - Health check

### Frontend
- http://localhost:3001 (local development)

### CLI Commands
- `sustaina fund` - Analyze dependencies
- `sustaina status` - Show registration status
- `sustaina deploy` - Generate deployment XDR

## Troubleshooting

### Port Already in Use
```bash
# Kill process on port 3000
lsof -i :3000
kill -9 <PID>
```

### Dependencies Not Installed
```bash
# Reinstall
make clean
make install
```

### Tests Failing
```bash
# Run with backtrace
RUST_BACKTRACE=1 cargo test
```

## Need Help?

- **Development:** See DEVELOPMENT.md
- **Deployment:** See DEPLOYMENT.md
- **Security:** See SECURITY.md
- **Testing:** See TESTING.md
- **Contributing:** See CONTRIBUTING.md
- **Architecture:** See ARCHITECTURE.md

## Docker Development

Run everything locally with Docker:

```bash
# Copy environment
cp .env.example .env

# Start services
docker-compose up

# Services will be available at:
# - Oracle: http://localhost:3000
# - Frontend: http://localhost:3001
```

## Git Workflow

```bash
# Create feature branch
git checkout -b feature/my-feature

# Make changes
# Test locally
make check-all

# Commit
git commit -m "feat: add new feature"

# Push
git push origin feature/my-feature

# Create Pull Request on GitHub
```

## Environment Setup

### Required
```bash
# Oracle private key (generate with):
openssl genpkey -algorithm ed25519 -outform DER | base64
# Save to ORACLE_SECRET_KEY in .env
```

### Optional
See `.env.example` for all available options.

## Next Steps

1. Run `make install`
2. Run `make test`
3. Read ARCHITECTURE.md
4. Start local dev with `make oracle-dev`
5. Follow DEPLOYMENT.md for production

## Quick Reference

```bash
make install       # One-time setup
make build        # Build all
make test         # Test all
make lint         # Lint code
make fmt          # Format code
make clean        # Clean build files
make check-all    # Full quality check
```
make install        # Install dependencies
make build         # Build all
make test          # Test all
make lint          # Lint code
make fmt           # Format code
make clean         # Clean build files
make check-all     # Full quality check
```

For detailed guides, see the documentation files listed above.
