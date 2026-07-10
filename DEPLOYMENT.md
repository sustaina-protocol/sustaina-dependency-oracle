# Deployment Guide

## Overview

This guide covers deploying Sustaina components to different environments (local development, testnet, and mainnet).

## Prerequisites

### Global Requirements

- Git
- Docker & Docker Compose (for containerized deployments)
- Rust 1.70+ (for smart contracts)
- Node.js 20+ (for Oracle and Frontend)

### Stellar CLI

```bash
# Install Soroban CLI
cargo install --locked stellar-cli

# Verify installation
soroban --version
```

### Environment

```bash
# Testnet
export STELLAR_NETWORK=testnet
export SOROBAN_RPC_URL=https://soroban-testnet.stellar.org:443
export SOROBAN_NETWORK_PASSPHRASE="Test SDF Network ; September 2015"

# Mainnet (when ready)
export STELLAR_NETWORK=public
export SOROBAN_RPC_URL=https://soroban-mainnet.stellar.org:443
export SOROBAN_NETWORK_PASSPHRASE="Public Global Stellar Network ; September 2015"
```

## 1. Smart Contract Deployment

### Build the Contract

```bash
cd contracts/identity-registry

# Build for wasm32 target
cargo build --target wasm32-unknown-unknown --release

# Verify build output
ls -lh target/wasm32-unknown-unknown/release/sustaina_identity_registry.wasm
```

### Deploy to Testnet

```bash
# Set your Stellar account that will deploy the contract
export SOROBAN_ACCOUNT=<your-stellar-public-key>

# Generate Ed25519 keypair for the Oracle (if not already done)
openssl genpkey -algorithm ed25519 -outform DER | base64 > oracle_key.b64
export ORACLE_SECRET_KEY=$(cat oracle_key.b64)
export ORACLE_PUBLIC_KEY=$(stellar-key-derive $ORACLE_SECRET_KEY)

# Deploy contract
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/sustaina_identity_registry.wasm \
  --network testnet \
  --source $SOROBAN_ACCOUNT

# Output: Contract ID (save this!)
# CONTRACT_ID=CABC...XXXX
export CONTRACT_ID=<output-contract-id>
```

### Initialize Contract

```bash
# Initialize with Oracle public key
soroban contract invoke \
  --id $CONTRACT_ID \
  --network testnet \
  --source $SOROBAN_ACCOUNT \
  -- initialize \
  --oracle_pub_key $ORACLE_PUBLIC_KEY
```

### Verify Deployment

```bash
# Call resolve to verify it's working
soroban contract invoke \
  --id $CONTRACT_ID \
  --network testnet \
  -- resolve \
  --repo_name "test/repo"
```

## 2. Oracle Service Deployment

### Generate Ed25519 Keys

```bash
# Generate Ed25519 private key (32 bytes, base64-encoded)
openssl genpkey -algorithm ed25519 -outform DER | base64

# Save this key securely - it's the ORACLE_SECRET_KEY
export ORACLE_SECRET_KEY=<output-from-above>
```

### Local Deployment

```bash
cd oracle-service

# Copy environment template
cp .env.example .env

# Edit .env with your ORACLE_SECRET_KEY
# vim .env

# Install dependencies
npm install

# Development mode
npm run dev
# Server listens on http://localhost:3000

# Test the endpoint
curl -X POST http://localhost:3000/verify \
  -H "Content-Type: application/json" \
  -d '{
    "oidcToken": "test-token",
    "repoName": "test/repo",
    "stellarAddress": "GBRPYHIL2CI3WHZDTOOQFC6EB4RBMPUTZEK2WNW2HXES2ELAPCSTEVE"
  }'
```

### Docker Deployment

```bash
cd oracle-service

# Build image
docker build -t sustaina-oracle:latest .

# Run container
docker run -p 3000:3000 \
  -e PORT=3000 \
  -e ORACLE_SECRET_KEY=$ORACLE_SECRET_KEY \
  sustaina-oracle:latest

# Test
curl http://localhost:3000/health
```

### Docker Compose (Full Stack)

```yaml
# docker-compose.yml
version: '3.8'

services:
  oracle:
    build: ./oracle-service
    ports:
      - "3000:3000"
    environment:
      PORT: 3000
      ORACLE_SECRET_KEY: ${ORACLE_SECRET_KEY}
      STELLAR_RPC_URL: https://soroban-testnet.stellar.org
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:3000/health"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 10s

  frontend:
    build: ./apps/explorer-ui
    ports:
      - "3001:3000"
    environment:
      NEXT_PUBLIC_ORACLE_URL: http://localhost:3000
      NEXT_PUBLIC_REGISTRY_CONTRACT: ${CONTRACT_ID}
      NEXT_PUBLIC_SOROBAN_RPC: https://soroban-testnet.stellar.org
```

Deploy:
```bash
export ORACLE_SECRET_KEY=<your-key>
export CONTRACT_ID=<your-contract-id>

docker-compose up -d
```

### Production Oracle Deployment

**AWS ECS:**

```bash
# Push to ECR
aws ecr get-login-password --region us-east-1 | docker login \
  --username AWS \
  --password-stdin <account-id>.dkr.ecr.us-east-1.amazonaws.com

docker tag sustaina-oracle:latest \
  <account-id>.dkr.ecr.us-east-1.amazonaws.com/sustaina-oracle:latest

docker push <account-id>.dkr.ecr.us-east-1.amazonaws.com/sustaina-oracle:latest

# Create ECS task definition and service
# (Use AWS Console or Terraform)
```

**Key Management:**

Store ORACLE_SECRET_KEY in AWS Secrets Manager:

```bash
aws secretsmanager create-secret \
  --name sustaina/oracle/secret-key \
  --secret-string "$ORACLE_SECRET_KEY"
```

Reference in ECS task definition:

```json
{
  "name": "ORACLE_SECRET_KEY",
  "valueFrom": "arn:aws:secretsmanager:us-east-1:account-id:secret:sustaina/oracle/secret-key:secret"
}
```

## 3. CLI Tool Deployment

### Build Release Binary

```bash
cd cli/sustaina-cli

# Build optimized release
cargo build --release

# Binary location: target/release/sustaina
cp target/release/sustaina /usr/local/bin/sustaina

# Verify
sustaina --version
```

### Install from Source

```bash
git clone https://github.com/sustaina-protocol/sustaina-dependency-oracle.git
cd sustaina-dependency-oracle/cli/sustaina-cli
cargo install --path .

# Now available as `sustaina` command globally
```

## 4. Frontend Deployment

### Build Static Export

```bash
cd apps/explorer-ui

# Build optimized next.js app
npm run build

# Export static site
npx next export

# Output in: out/ directory
```

### Vercel Deployment

```bash
# Install Vercel CLI
npm i -g vercel

# Deploy
cd apps/explorer-ui
vercel

# Configure environment variables:
# NEXT_PUBLIC_ORACLE_URL=https://oracle.sustaina.dev
# NEXT_PUBLIC_REGISTRY_CONTRACT=CABC...XXXX
# NEXT_PUBLIC_SOROBAN_RPC=https://soroban-testnet.stellar.org
```

### AWS S3 + CloudFront

```bash
# Build
cd apps/explorer-ui
npm run build && npx next export

# Upload to S3
aws s3 sync out/ s3://sustaina-explorer-ui/

# Invalidate CloudFront cache
aws cloudfront create-invalidation \
  --distribution-id <DISTRIBUTION_ID> \
  --paths "/*"
```

### Docker Deployment

```dockerfile
# Dockerfile for frontend
FROM node:20-alpine AS builder
WORKDIR /app
COPY package*.json ./
RUN npm ci
COPY . .
RUN npm run build

FROM node:20-alpine
WORKDIR /app
COPY --from=builder /app/public ./public
COPY --from=builder /app/.next/standalone ./
COPY --from=builder /app/.next/static ./.next/static

EXPOSE 3000
CMD ["node", "server.js"]
```

Build and run:
```bash
docker build -t sustaina-explorer:latest .
docker run -p 3000:3000 \
  -e NEXT_PUBLIC_ORACLE_URL=https://oracle.sustaina.dev \
  -e NEXT_PUBLIC_REGISTRY_CONTRACT=$CONTRACT_ID \
  sustaina-explorer:latest
```

## 5. GitHub Action Workflow

Create `.github/workflows/register.yml` in user repositories:

```yaml
name: Register with Sustaina

on:
  workflow_dispatch:  # Manual trigger

jobs:
  register:
    runs-on: ubuntu-latest
    permissions:
      id-token: write  # Required for OIDC token
    steps:
      - name: Get OIDC Token
        id: oidc
        uses: actions/github-script@v7
        with:
          script: |
            const fs = require('fs');
            const token = await core.getIDToken();
            console.log('::set-output name=token::' + token);

      - name: Register with Sustaina
        run: |
          curl -X POST https://oracle.sustaina.dev/verify \
            -H "Content-Type: application/json" \
            -d '{
              "oidcToken": "${{ steps.oidc.outputs.token }}",
              "repoName": "${{ github.repository }}",
              "stellarAddress": "${{ secrets.STELLAR_ADDRESS }}"
            }'
```

Users add to their repository and run manually after setting `STELLAR_ADDRESS` secret.

## 6. Monitoring & Logging

### Oracle Service Monitoring

```bash
# View logs
docker logs -f sustaina-oracle

# Health check
curl http://localhost:3000/health

# Prometheus metrics (add to monitoring)
curl http://localhost:3000/metrics
```

### Contract Event Indexing

```bash
# Index contract events
soroban events \
  --id $CONTRACT_ID \
  --network testnet \
  --limit 100
```

### Frontend Monitoring

Use Vercel Analytics or Datadog:

```bash
# Datadog
npm install @datadog/browser-rum

# Environment variable
NEXT_PUBLIC_DATADOG_APPLICATION_ID=<app-id>
```

## 7. Upgrade Procedures

### Contract Upgrade

Soroban contracts are immutable. To deploy a new version:

1. Build new WASM
2. Deploy as new contract
3. Update environment variables (CONTRACT_ID)
4. Call initialize on new contract

### Oracle Key Rotation

1. Generate new Ed25519 key
2. Deploy new Oracle instance with new key
3. Call `initialize` on contract with new public key
4. Redirect traffic to new Oracle
5. Keep old Oracle running for a transition period

### Frontend Updates

```bash
# Build and deploy
cd apps/explorer-ui
npm run build
npm run deploy  # or vercel deploy
```

## 8. Troubleshooting

### Contract Deployment Fails

```bash
# Check account has sufficient XLM
soroban account info --source $SOROBAN_ACCOUNT --network testnet

# Fund account on testnet
curl -X POST "https://friendbot.stellar.org?addr=$SOROBAN_ACCOUNT"
```

### OIDC Token Verification Fails

```bash
# Verify JWKS is accessible
curl https://token.actions.githubusercontent.com/.well-known/jwks.json

# Check token claims
# Decode JWT at jwt.io

# Verify repository claim matches
```

### Contract Initialization Fails

```bash
# Verify contract is deployed
soroban contract info --id $CONTRACT_ID --network testnet

# Check public key format
echo $ORACLE_PUBLIC_KEY | base64 -d | od -An -tx1
```

### Oracle Service Port Conflict

```bash
# Change port
export PORT=3001
npm run dev

# Or kill process on port 3000
lsof -i :3000
kill -9 <PID>
```

## Backup & Recovery

### Contract State Backup

```bash
# Export contract state
soroban contract invoke \
  --id $CONTRACT_ID \
  --network testnet \
  -- resolve \
  --repo_name "*" > contract_state.json
```

### Oracle Key Backup

```bash
# Secure backup of ORACLE_SECRET_KEY
echo $ORACLE_SECRET_KEY | gpg -c > oracle_key.gpg

# Store in secure location (password manager, vault, etc.)
```

## Rollback Procedures

### Frontend Rollback

Vercel:
```bash
vercel rollback
```

AWS S3:
```bash
# Restore previous version from S3 versioning
aws s3api list-object-versions --bucket sustaina-explorer-ui
aws s3api get-object --bucket sustaina-explorer-ui --key index.html \
  --version-id <VERSION_ID> index.html
```

### Oracle Rollback

```bash
# Revert to previous Docker image
docker run -p 3000:3000 sustaina-oracle:previous-tag
```

## Maintenance Checklist

- [ ] Weekly: Review Oracle access logs
- [ ] Monthly: Update dependencies
- [ ] Monthly: Check for security vulnerabilities
- [ ] Quarterly: Rotate Oracle keys
- [ ] Quarterly: Review registered identities
- [ ] Semi-annually: Contract security audit
- [ ] Annually: Disaster recovery drill
