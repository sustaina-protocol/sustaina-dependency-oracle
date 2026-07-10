# Testing Guide

This document describes how to test all components of the Sustaina Dependency Oracle.

## Overview

The project includes tests for:
- Smart Contract (unit tests)
- Oracle Service (API and logic tests)
- CLI Tool (unit and integration tests)
- Frontend (component tests)

## Running All Tests

```bash
# Run complete test suite for all components
make test

# Full quality check (format, lint, test, build)
make check-all
```

## Component-Specific Testing

### Smart Contract Tests

**Location:** `contracts/identity-registry/src/test.rs`

```bash
cd contracts/identity-registry

# Run all tests
cargo test

# Run specific test
cargo test test_initialize_contract

# Run tests with output
cargo test -- --nocapture

# Run tests sequentially (useful for debugging)
cargo test -- --test-threads=1

# Run with backtrace
RUST_BACKTRACE=1 cargo test

# Generate test coverage (requires cargo-tarpaulin)
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

**Test Categories:**
- Initialization tests
- Registration tests
- Update tests
- Resolution tests
- Error handling tests
- Authorization tests

### Oracle Service Tests

**Location:** `oracle-service/src/` (currently no test files, can be added)

```bash
cd oracle-service

# Type checking
npm run type-check

# Manual testing of endpoints
curl -X GET http://localhost:3000/health

# Test OIDC verification (with valid token)
curl -X POST http://localhost:3000/verify \
  -H "Content-Type: application/json" \
  -d '{
    "oidcToken": "<valid-token>",
    "repoName": "owner/repo",
    "stellarAddress": "GBRPYHIL2CI3WHZDTOOQFC6EB4RBMPUTZEK2WNW2HXES2ELAPCSTEVE"
  }'

# Test with invalid token (should fail gracefully)
curl -X POST http://localhost:3000/verify \
  -H "Content-Type: application/json" \
  -d '{
    "oidcToken": "invalid",
    "repoName": "owner/repo",
    "stellarAddress": "GBRPYHIL2CI3WHZDTOOQFC6EB4RBMPUTZEK2WNW2HXES2ELAPCSTEVE"
  }'
```

**To Add Tests:**
```bash
npm install --save-dev jest @types/jest ts-jest
```

### CLI Tool Tests

**Location:** `cli/sustaina-cli/src/`

```bash
cd cli/sustaina-cli

# Run all tests
cargo test

# Run specific test module
cargo test parser::

# Run with backtrace
RUST_BACKTRACE=1 cargo test

# Test with debug output
RUST_LOG=debug cargo test

# Run tests sequentially
cargo test -- --test-threads=1
```

**Test Examples:**
```bash
# Test dependency parsing
cargo test test_parse_basic_manifest

# Test error handling
cargo test test_invalid_percentage
```

### Frontend Tests

**Location:** `apps/explorer-ui/src/` (component tests can be added)

```bash
cd apps/explorer-ui

# Type checking
npm run type-check

# Build check (catches many issues)
npm run build

# Manual component testing
npm run dev
# Open http://localhost:3000

# Build for production
npm run build && npm start
```

**To Add Tests:**
```bash
npm install --save-dev jest @testing-library/react @testing-library/jest-dom
```

## Integration Testing

### End-to-End Workflow

Test the complete flow locally:

1. **Start the Oracle**
   ```bash
   cd oracle-service
   npm install
   npm run dev
   ```

2. **Start the Frontend**
   ```bash
   cd apps/explorer-ui
   npm install
   npm run dev
   # Open http://localhost:3000
   ```

3. **Test CLI**
   ```bash
   cd cli/sustaina-cli
   cargo build --release
   
   # Test with sample Cargo.toml
   ./target/release/sustaina fund --manifest ../../Cargo.toml
   ./target/release/sustaina status --manifest ../../Cargo.toml
   ```

4. **Deploy and test contract locally**
   ```bash
   cd contracts/identity-registry
   cargo test
   ```

## Test Coverage

### Smart Contract Coverage

## Smart Contract Coverage

Current test coverage:
- initialize() - Tested
- register_identity() - Tested
- update_identity() - Tested
- resolve() - Tested
- Error cases - Covered
- Authorization - Tested

**To improve coverage:**
```bash
# Using cargo-tarpaulin
cargo tarpaulin --out Html --output-dir coverage

# Using cargo-llvm-cov
cargo llvm-cov --html
```

### CLI Coverage

## CLI Coverage

Current test coverage:
- Dependency parsing - Tested
- Registry resolution - Partial
- Error handling - Covered

### Frontend Coverage

## Frontend Coverage

Current test coverage:
- Manual/visual testing - Present
- Type checking - Enabled
- Build validation - Configured

**To add automated tests:**
```bash
npm install --save-dev jest @testing-library/react
npm test
```

## Testing Best Practices

### Unit Tests

```rust
// Good: Clear, focused test
#[test]
fn test_initialize_with_valid_key() {
    let env = Env::default();
    let oracle_pub_key = BytesN::from_array(&env, &[1; 32]);
    let registry_id = env.register_contract(None, IdentityRegistry);
    let client = IdentityRegistryClient::new(&env, &registry_id);
    
    let result = client.initialize(&oracle_pub_key);
    assert!(result.is_ok());
}

// Bad: Too many concerns
#[test]
fn test_everything() {
    // Tests initialization, registration, and resolution together
}
```

### Testing Errors

```rust
// Good: Test both success and failure
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
```

## CI/CD Testing

Tests run automatically on:
- **Push to any branch** - Full test suite
- **Pull request** - Full test suite + linting
- **Release tag** - Full test suite + Docker build

See `.github/workflows/test.yml` for details.

## Debugging Tests

### Verbose Output

```bash
# Show test output
cargo test -- --nocapture

# Show ignored tests too
cargo test -- --include-ignored
```

### Single-threaded Execution

```bash
# Useful when tests interfere with each other
cargo test -- --test-threads=1
```

### Backtrace

```bash
# Full backtrace on panic
RUST_BACKTRACE=full cargo test

# Even more verbose
RUST_BACKTRACE=full RUST_LOG=debug cargo test
```

### Using Debugger

```bash
# Debug with rust-gdb
rust-gdb ./target/debug/sustaina-cli

# With breakpoint
(gdb) break main
(gdb) run
(gdb) continue
```

## Performance Testing

### Contract Performance

```bash
# Measure contract build time
cargo build --target wasm32-unknown-unknown --release --timings

# Check WASM size
ls -lh target/wasm32-unknown-unknown/release/sustaina_identity_registry.wasm
```

### CLI Performance

```bash
# Time CLI execution
time ./target/release/sustaina fund --manifest Cargo.toml

# Memory usage
/usr/bin/time -v ./target/release/sustaina fund
```

## Test Maintenance

### Regular Checks

- **Weekly:** Run `make check-all` before commits
- **Monthly:** Update test dependencies
- **Quarterly:** Review test coverage and add missing tests
- **Before release:** Run full integration tests

### Updating Tests

When modifying code:
1. Update or add tests
2. Ensure tests still pass: `make test`
3. Check coverage remains high
4. Update documentation if needed

## Common Issues

### "test failed to compile"

```bash
# Clean build artifacts
cargo clean

# Update toolchain
rustup update
```

### "test timed out"

```bash
# Increase timeout
cargo test -- --test-threads=1 --timeout 60
```

### "permission denied" on compiled tests

```bash
# Make test binary executable
chmod +x ./target/debug/sustaina-*
```

## Adding New Tests

### For Smart Contracts

1. Open `contracts/identity-registry/src/test.rs`
2. Add test function:
   ```rust
   #[test]
   fn test_my_feature() {
       // setup
       let env = Env::default();
       
       // execute
       let result = my_function(&env);
       
       // assert
       assert!(result.is_ok());
   }
   ```
3. Run: `cargo test`

### For CLI

1. Open `cli/sustaina-cli/src/` relevant file
2. Add test module:
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;
       
       #[test]
       fn test_my_feature() {
           // test code
       }
   }
   ```
3. Run: `cargo test`

### For Frontend

1. Create `src/components/MyComponent.test.tsx`:
   ```typescript
   import { render, screen } from '@testing-library/react';
   import MyComponent from './MyComponent';
   
   describe('MyComponent', () => {
     it('renders correctly', () => {
       render(<MyComponent />);
       expect(screen.getByText(/test/i)).toBeInTheDocument();
     });
   });
   ```
2. Run: `npm test`

## Resources

- [Rust Testing Book](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Soroban Testing Docs](https://developers.stellar.org/docs/smart-contracts/testing)
- [Jest Documentation](https://jestjs.io/)
- [React Testing Library](https://testing-library.com/react)

## Quick Reference

```bash
# All tests
make test

# Quality checks
make check-all

# Specific component tests
make contract-test     # Smart contract
make cli-test         # CLI tool
make oracle-build     # Oracle type check

# Code quality
make lint            # Linting
make fmt             # Format code
```

---

**Last Updated:** 2024
**Version:** 1.0.0
