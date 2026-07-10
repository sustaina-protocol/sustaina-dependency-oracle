# Development Guide

## Setup

### Prerequisites

- **Rust:** 1.70+ with `wasm32-unknown-unknown` target
- **Node.js:** 20+ with npm
- **Docker:** 24+ (optional, for containerized dev)
- **Git:** For version control

### Initial Setup

```bash
# Clone repository
git clone https://github.com/sustaina-protocol/sustaina-dependency-oracle.git
cd sustaina-dependency-oracle

# Install all dependencies
make install

# Verify everything works
make test
```

## Project Structure

```
sustaina-dependency-oracle/
├── contracts/              # Soroban smart contracts
│   └── identity-registry/
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs      # Main contract
│           ├── crypto.rs   # Cryptographic utilities
│           └── test.rs     # Tests
├── oracle-service/         # Node.js Oracle
│   ├── package.json
│   ├── tsconfig.json
│   ├── Dockerfile
│   ├── src/
│   │   ├── index.ts        # Express server
│   │   ├── oidc.ts         # GitHub OIDC verification
│   │   └── signer.ts       # Ed25519 signing
│   └── .env.example
├── cli/                    # Rust CLI
│   └── sustaina-cli/
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs     # CLI entry point
│           ├── parser.rs   # Cargo.toml parsing
│           ├── resolver.rs # Registry resolution
│           └── errors.rs   # Error types
├── apps/                   # Frontend applications
│   └── explorer-ui/
│       ├── package.json
│       ├── tsconfig.json
│       ├── next.config.js
│       ├── tailwind.config.js
│       └── src/
│           ├── app/
│           │   ├── layout.tsx
│           │   ├── page.tsx
│           │   └── globals.css
│           └── store/
│               └── useGraphStore.ts
├── .github/workflows/      # CI/CD
│   ├── test.yml
│   └── docker-publish.yml
├── Makefile               # Build automation
├── README.md              # Project overview
├── ARCHITECTURE.md        # Technical architecture
├── SECURITY.md            # Security analysis
└── DEPLOYMENT.md          # Deployment guide
```

## Development Workflows

### Smart Contract Development

#### Edit Contract

```bash
cd contracts/identity-registry/src
# Edit lib.rs, crypto.rs, or test.rs
```

#### Build

```bash
cd contracts/identity-registry
cargo build --target wasm32-unknown-unknown --release
# Output: target/wasm32-unknown-unknown/release/sustaina_identity_registry.wasm
```

#### Test

```bash
cd contracts/identity-registry
cargo test
# Or run specific test
cargo test test_initialize_contract
```

#### Format & Lint

```bash
cd contracts/identity-registry
cargo fmt
cargo clippy -- -D warnings
```

#### Run with Soroban CLI

```bash
cd contracts/identity-registry

# Start soroban CLI with local ledger
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/sustaina_identity_registry.wasm \
  --network standalone

# Get the contract ID and test it
soroban contract invoke --id <CONTRACT_ID> -- initialize --oracle_pub_key <KEY>
```

### Oracle Service Development

#### Start Development Server

```bash
cd oracle-service

# Setup environment
cp .env.example .env
# Edit .env if needed

# Install dependencies
npm install

# Start dev server with hot reload
npm run dev
# Server runs on http://localhost:3000
```

#### Test the Endpoint

```bash
# Terminal 1: Start Oracle
npm run dev

# Terminal 2: Test
curl -X GET http://localhost:3000/health

# Test verify endpoint (will fail without valid token, but tests connectivity)
curl -X POST http://localhost:3000/verify \
  -H "Content-Type: application/json" \
  -d '{
    "oidcToken": "invalid-token",
    "repoName": "owner/repo",
    "stellarAddress": "GBRPYHIL2CI3WHZDTOOQFC6EB4RBMPUTZEK2WNW2HXES2ELAPCSTEVE"
  }'
```

#### Build TypeScript

```bash
cd oracle-service
npm run build
# Output: dist/index.js
```

#### Type Checking

```bash
cd oracle-service
npm run type-check
# Checks TypeScript without emitting files
```

#### Linting & Formatting

```bash
cd oracle-service
npm run lint      # ESLint
npm run fmt       # Prettier format
```

#### Docker Development

```bash
cd oracle-service

# Build image
docker build -t sustaina-oracle:dev .

# Run interactively
docker run -it \
  -p 3000:3000 \
  -e ORACLE_SECRET_KEY=$ORACLE_SECRET_KEY \
  sustaina-oracle:dev
```

### CLI Tool Development

#### Build CLI

```bash
cd cli/sustaina-cli
cargo build
# Output: target/debug/sustaina

# Or release build
cargo build --release
# Output: target/release/sustaina
```

#### Test CLI

```bash
cd cli/sustaina-cli

# Run tests
cargo test

# Or specific test
cargo test test_parse_basic_manifest

# Test binary
./target/debug/sustaina --help
./target/debug/sustaina fund --manifest ../../Cargo.toml
```

#### Format & Lint

```bash
cd cli/sustaina-cli
cargo fmt
cargo clippy -- -D warnings
```

#### Debug Execution

```bash
# Run with debug output
RUST_LOG=debug ./target/debug/sustaina fund --manifest Cargo.toml

# Run under debugger
rust-gdb ./target/debug/sustaina
```

### Frontend Development

#### Start Development Server

```bash
cd apps/explorer-ui

# Install dependencies
npm install

# Start dev server
npm run dev
# Opens http://localhost:3000
```

#### Edit Components

```bash
# All changes hot-reload automatically
cd apps/explorer-ui/src/app
vim page.tsx  # Edit the main page
```

#### Type Checking

```bash
cd apps/explorer-ui
npm run type-check
```

#### Build for Production

```bash
cd apps/explorer-ui
npm run build
# Output: .next/ directory
```

#### Linting

```bash
cd apps/explorer-ui
npm run lint
```

## Testing Strategy

### Unit Tests

**Smart Contract:**
```bash
cd contracts/identity-registry
cargo test test_initialize_contract
```

**CLI:**
```bash
cd cli/sustaina-cli
cargo test test_parse_basic_manifest
```

**Oracle:** (Add with jest/vitest)
```bash
cd oracle-service
npm test  # After setting up test framework
```

### Integration Tests

```bash
# Full workflow test
cd contracts/identity-registry
cargo test test_register_and_resolve
```

### End-to-End Tests

```bash
# Test complete flow:
# 1. Deploy contract
# 2. Run Oracle
# 3. Call resolve
# (Add E2E test scripts here)
```

### Run All Tests

```bash
make test
# Runs tests for all components
```

## Code Style Guide

### Rust

```rust
// Use doc comments for public items
/// Verifies a GitHub OIDC token.
pub fn verify_token(token: &str) -> Result<Claims, Error> {
    // Implementation
}

// Follow Rust naming conventions
fn compute_hash(data: &[u8]) -> Vec<u8> { }

// Use match instead of if-else for enums
match result {
    Ok(value) => { },
    Err(e) => { },
}
```

### TypeScript

```typescript
// Use interfaces for objects
interface VerifyRequest {
  oidcToken: string;
  repoName: string;
  stellarAddress: string;
}

// Use async/await
async function verifyToken(token: string): Promise<boolean> {
  // Implementation
}

// Use strict typing
const PORT: number = 3000;
```

### React/Next.js

```typescript
// Use 'use client' for client components
'use client';

import React, { useState } from 'react';

// Use functional components
export default function MyComponent() {
  const [state, setState] = useState(null);
  return <div>{state}</div>;
}

// Use TypeScript for props
interface Props {
  title: string;
  count: number;
}
```

## Common Development Tasks

### Add a New CLI Command

1. Add variant to `Commands` enum in `main.rs`
2. Implement handler function
3. Add to main match statement
4. Add tests

```rust
// In cli/sustaina-cli/src/main.rs
#[derive(Subcommand)]
enum Commands {
    MyNewCommand {
        #[arg(short, long)]
        option: String,
    }
}

// Handle it
Commands::MyNewCommand { option } => {
    handle_my_command(&option).await?
}
```

### Add a New Oracle Endpoint

1. Add route in `index.ts`
2. Implement handler function
3. Add logging

```typescript
// In oracle-service/src/index.ts
app.post('/new-endpoint', async (req: Request, res: Response) => {
  logger.debug('New endpoint called');
  res.json({ status: 'success' });
});
```

### Add a New Contract Function

1. Add method to `impl IdentityRegistry`
2. Add to contract trait
3. Add tests
4. Update documentation

```rust
// In contracts/identity-registry/src/lib.rs
#[contractimpl]
impl IdentityRegistry {
    pub fn my_function(env: Env, param: String) -> Result<(), RegistryError> {
        // Implementation
        Ok(())
    }
}
```

### Add Frontend Component

1. Create component in `src/components/`
2. Export from `src/components/index.ts`
3. Use in page

```typescript
// Create src/components/MyComponent.tsx
export default function MyComponent() {
  return <div>My Component</div>;
}

// Use in page.tsx
import MyComponent from '@/components/MyComponent';
```

## Debugging

### Rust Debugging

```bash
# Run with backtrace
RUST_BACKTRACE=1 cargo test

# Use rust-gdb
rust-gdb ./target/debug/sustaina

# Print debug info
dbg!(variable);
```

### TypeScript Debugging

```bash
# Run with node debugger
node inspect dist/index.js

# Or use VS Code debugger (set breakpoints)
```

### Contract Debugging

```bash
# Use soroban CLI debug mode
soroban contract invoke \
  --id <CONTRACT_ID> \
  --network standalone \
  -- my_function \
  --param value \
  --verbose
```

## Performance Optimization

### Contract

```rust
// Use efficient data structures
use soroban_sdk::map::Map;

// Minimize storage operations
let key = compute_key_once();
// Reuse key instead of recomputing

// Use appropriate numeric types
// u32 instead of u64 when possible
```

### Oracle

```typescript
// Cache JWKS to avoid repeated fetches
const cachedPublicKeys = new Map();

// Use connection pooling
const pool = createPool();

// Batch operations when possible
```

### CLI

```rust
// Use rayon for parallel processing
use rayon::prelude::*;

deps.par_iter()
  .map(|dep| resolve(dep))
  .collect()
```

### Frontend

```typescript
// Use React.memo for expensive components
export const MyComponent = React.memo(function MyComponent(props) {
  return <div>{props.value}</div>;
});

// Optimize images
<Image src={...} loading="lazy" />

// Use dynamic imports
const HeavyComponent = dynamic(() => import('./Heavy'));
```

## Continuous Integration

GitHub Actions workflows run automatically:

- **On push:** Run tests and lint
- **On PR:** Run tests and lint before merge
- **On tag:** Build and publish Docker image

View workflows in `.github/workflows/`

### Run Tests Locally (Before Pushing)

```bash
make check-all
# Runs: clean, lint, test, build
```

## Documentation

### Code Documentation

Use doc comments:

```rust
/// Registers a new identity.
///
/// # Arguments
/// * `repo_name` - Repository identifier
/// * `owner` - Stellar address
///
/// # Returns
/// Result indicating success or error
pub fn register_identity(...) -> Result<(), Error> { }
```

### Generate Docs

```bash
# Rust
cargo doc --open

# TypeScript (optional)
npm run docs
```

## Troubleshooting Development Issues

### Issue: `wasm32-unknown-unknown` target not found

```bash
rustup target add wasm32-unknown-unknown
```

### Issue: Port 3000 already in use

```bash
# Find and kill process
lsof -i :3000
kill -9 <PID>

# Or use different port
PORT=3001 npm run dev
```

### Issue: Node modules conflicts

```bash
# Clean and reinstall
rm -rf node_modules package-lock.json
npm install
```

### Issue: Cargo build fails

```bash
# Clean build artifacts
cargo clean

# Update toolchain
rustup update
```

### Issue: Tests fail intermittently

```bash
# Run tests sequentially
cargo test -- --test-threads=1

# Run with backtrace
RUST_BACKTRACE=full cargo test
```

## Release Process

### Prepare Release

1. Bump version numbers in `Cargo.toml` and `package.json`
2. Update `CHANGELOG.md`
3. Create release branch: `git checkout -b release/v1.0.0`
4. Run full test suite: `make check-all`
5. Commit: `git commit -m "Release v1.0.0"`

### Create Release

1. Push branch: `git push origin release/v1.0.0`
2. Create Pull Request
3. Get code review
4. Merge to main
5. Tag release: `git tag -a v1.0.0 -m "Version 1.0.0"`
6. Push tag: `git push origin v1.0.0`

### Publish

- Docker image automatically published by GitHub Actions
- CLI binary available in GitHub Releases
- npm packages published if setup

## Resources

- [Soroban Documentation](https://developers.stellar.org/docs/smart-contracts)
- [Rust Book](https://doc.rust-lang.org/book/)
- [TypeScript Handbook](https://www.typescriptlang.org/docs/)
- [Next.js Documentation](https://nextjs.org/docs)
- [Ed25519 Specification](https://tools.ietf.org/html/rfc8032)

## Support

For development questions:
- Check existing issues on GitHub
- Open a discussion on GitHub Discussions
- See ARCHITECTURE.md for design documentation
- See SECURITY.md for security considerations
