import { Keypair } from '@stellar/stellar-sdk';
import crypto from 'crypto';
import pino from 'pino';

const logger = pino();

/**
 * Creates a SHA-256 hash of the identity payload.
 *
 * This mirrors the computation in the Soroban contract's crypto module.
 * The payload is constructed as:
 *   1. Repository name bytes
 *   2. Stellar address XDR serialization (as would be produced by the contract)
 *
 * @param repoName - Repository identifier
 * @param stellarAddress - Stellar address string
 * @returns SHA-256 hash as Buffer
 */
function computePayloadHash(repoName: string, stellarAddress: string): Buffer {
  // Convert repo name to bytes
  const repoNameBytes = Buffer.from(repoName, 'utf-8');

  // For Stellar addresses, we use the canonical account ID encoding
  // The contract uses to_xdr which produces the 32-byte canonical form
  let addressBytes: Buffer;
  try {
    const keypair = Keypair.fromPublicKey(stellarAddress);
    // Stellar addresses are 56 characters, encoding 32 bytes in base32
    addressBytes = keypair.rawPublicKey();
  } catch (error) {
    logger.error({ stellarAddress }, 'Invalid Stellar address');
    throw new Error('Invalid Stellar address format');
  }

  // Concatenate: repo name + address bytes
  const payload = Buffer.concat([repoNameBytes, addressBytes]);

  // Return SHA-256 hash
  return crypto.createHash('sha256').update(payload).digest();
}

/**
 * Signs an identity proof using the Oracle's Ed25519 private key.
 *
 * This generates the cryptographic proof that links a GitHub repository
 * to a Stellar address, which is then verified on-chain by the registry contract.
 *
 * @param repoName - Repository identifier (e.g., "owner/repo")
 * @param stellarAddress - Stellar address to receive funds
 * @param oracleSecretKey - Base64-encoded Ed25519 secret key
 * @returns 64-byte Ed25519 signature as Buffer
 */
export function signIdentityPayload(
  repoName: string,
  stellarAddress: string,
  oracleSecretKey: string,
): Buffer {
  try {
    logger.debug({ repoName, stellarAddress }, 'Generating identity signature');

    // Compute the payload hash (must match contract's compute_payload_hash)
    const payloadHash = computePayloadHash(repoName, stellarAddress);

    // Create a Keypair from the secret key
    // The secret key should be base64-encoded raw Ed25519 secret (32 bytes)
    const secretKeyBuffer = Buffer.from(oracleSecretKey, 'base64');
    if (secretKeyBuffer.length !== 32) {
      throw new Error(`Invalid secret key length: expected 32 bytes, got ${secretKeyBuffer.length}`);
    }

    // Sign the payload hash using Ed25519
    const signature = crypto.sign('sha256', payloadHash, {
      key: secretKeyBuffer,
      format: 'der',
    });

    if (signature.length !== 64) {
      throw new Error(`Invalid signature length: expected 64 bytes, got ${signature.length}`);
    }

    logger.debug({ repoName }, 'Identity signature generated successfully');
    return signature;
  } catch (error: any) {
    logger.error({ error: error.message, repoName }, 'Failed to generate signature');
    throw error;
  }
}

/**
 * Retrieves the public key corresponding to the Oracle's secret key.
 *
 * This is useful for contract initialization and verification.
 *
 * @param oracleSecretKey - Base64-encoded Ed25519 secret key
 * @returns The 32-byte Ed25519 public key as a Buffer
 */
export function getOraclePublicKey(oracleSecretKey: string): Buffer {
  try {
    const secretKeyBuffer = Buffer.from(oracleSecretKey, 'base64');
    if (secretKeyBuffer.length !== 32) {
      throw new Error(`Invalid secret key length: expected 32 bytes, got ${secretKeyBuffer.length}`);
    }

    // Derive public key from secret key using Ed25519
    const keypair = crypto.createPrivateKey({
      key: secretKeyBuffer,
      format: 'der',
      type: 'ed25519',
    });

    const publicKey = crypto.createPublicKey(keypair);
    return publicKey.export({ format: 'der', type: 'spki' });
  } catch (error: any) {
    logger.error({ error: error.message }, 'Failed to derive public key');
    throw error;
  }
}
