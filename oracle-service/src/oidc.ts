import { jwtVerify, importSPKI } from 'jose';
import pino from 'pino';

const logger = pino();

const GITHUB_JWKS_URL = 'https://token.actions.githubusercontent.com/.well-known/jwks.json';

interface GitHubOIDCPayload {
  iss: string;
  sub: string;
  aud: string;
  iat: number;
  exp: number;
  repository: string;
  repository_owner: string;
  run_id: string;
  run_number: number;
  ref: string;
  sha: string;
  actor: string;
  event: string;
  workflow: string;
  head_ref?: string;
  base_ref?: string;
  [key: string]: any;
}

let cachedPublicKeys: Map<string, string> | null = null;
let keysCacheTime: number = 0;
const KEYS_CACHE_TTL = 3600000; // 1 hour

/**
 * Fetches and caches the GitHub JWKS public keys.
 * Implements caching to reduce network calls.
 */
async function getGithubPublicKeys(): Promise<Map<string, string>> {
  const now = Date.now();

  if (cachedPublicKeys && now - keysCacheTime < KEYS_CACHE_TTL) {
    logger.debug('Using cached GitHub public keys');
    return cachedPublicKeys;
  }

  try {
    const response = await fetch(GITHUB_JWKS_URL);
    const jwks = await response.json();

    cachedPublicKeys = new Map();
    for (const key of jwks.keys) {
      if (key.kid) {
        cachedPublicKeys.set(key.kid, JSON.stringify(key));
      }
    }

    keysCacheTime = now;
    logger.debug('GitHub public keys fetched and cached');
    return cachedPublicKeys;
  } catch (error) {
    logger.error({ error }, 'Failed to fetch GitHub public keys');
    throw new Error('Unable to verify OIDC token: GitHub keys unavailable');
  }
}

/**
 * Verifies a GitHub Actions OIDC token.
 *
 * Steps:
 * 1. Fetch GitHub's public keys from their JWKS endpoint
 * 2. Decode and validate the JWT signature
 * 3. Check token expiration
 * 4. Verify the repository claim matches the expected repository
 *
 * @param token - The JWT from GitHub Actions
 * @param expectedRepo - The expected repository in format "owner/repo"
 * @returns true if token is valid and repository matches; false otherwise
 */
export async function verifyGithubOidc(token: string, expectedRepo: string): Promise<boolean> {
  try {
    logger.debug({ expectedRepo }, 'Verifying GitHub OIDC token');

    // Extract the header to find the key ID (kid)
    const parts = token.split('.');
    if (parts.length !== 3) {
      logger.warn('Invalid JWT format: expected 3 parts');
      return false;
    }

    const header = JSON.parse(
      Buffer.from(parts[0], 'base64url').toString('utf-8'),
    );
    const kid = header.kid;

    if (!kid) {
      logger.warn('Missing kid in JWT header');
      return false;
    }

    // Fetch GitHub's public keys
    const publicKeysMap = await getGithubPublicKeys();
    const publicKeyPem = publicKeysMap.get(kid);

    if (!publicKeyPem) {
      logger.warn({ kid }, 'Key ID not found in GitHub JWKS');
      return false;
    }

    const publicKey = JSON.parse(publicKeyPem);
    const key = await importSPKI(JSON.stringify(publicKey), 'RS256');

    // Verify and decode the JWT
    const verified = await jwtVerify(token, key, {
      algorithms: ['RS256'],
      issuer: 'https://token.actions.githubusercontent.com',
    });

    const payload = verified.payload as GitHubOIDCPayload;

    // Validate token expiration (jose should handle this, but explicit check for safety)
    if (!payload.exp || payload.exp * 1000 < Date.now()) {
      logger.warn('Token has expired');
      return false;
    }

    // Check repository claim
    if (payload.repository !== expectedRepo) {
      logger.warn(
        { expected: expectedRepo, received: payload.repository },
        'Repository mismatch',
      );
      return false;
    }

    logger.info({ repo: payload.repository, actor: payload.actor }, 'OIDC token verified');
    return true;
  } catch (error: any) {
    logger.error({ error: error.message }, 'OIDC token verification failed');
    return false;
  }
}
