import express, { Request, Response } from 'express';
import dotenv from 'dotenv';
import pino from 'pino';
import { verifyGithubOidc } from './oidc';
import { signIdentityPayload } from './signer';

dotenv.config();

const app = express();
const logger = pino();

app.use(express.json());

const PORT = process.env.PORT || 3000;
const ORACLE_SECRET_KEY = process.env.ORACLE_SECRET_KEY;

if (!ORACLE_SECRET_KEY) {
  logger.error('Missing required environment variable: ORACLE_SECRET_KEY');
  process.exit(1);
}

/**
 * POST /verify
 *
 * Verifies a GitHub OIDC token and generates an Oracle signature.
 *
 * Request body:
 * - oidcToken: string - JWT from GitHub Actions
 * - repoName: string - Repository identifier (e.g., "org/repo")
 * - stellarAddress: string - Destination Stellar address
 *
 * Response:
 * - status: "success" | "error"
 * - repoName: string
 * - stellarAddress: string
 * - oracleSignature: string (hex-encoded Ed25519 signature)
 */
app.post('/verify', async (req: Request, res: Response): Promise<void> => {
  try {
    const { oidcToken, repoName, stellarAddress } = req.body;

    // Validate input parameters
    if (!oidcToken || !repoName || !stellarAddress) {
      logger.warn('Missing required parameters in /verify request');
      res.status(400).json({ error: 'Missing required fields: oidcToken, repoName, stellarAddress' });
      return;
    }

    logger.info({ repoName, stellarAddress }, 'Processing identity verification request');

    // Step 1: Verify GitHub Action OIDC Token
    const isValid = await verifyGithubOidc(oidcToken, repoName);
    if (!isValid) {
      logger.warn({ repoName }, 'Invalid OIDC token or repository mismatch');
      res.status(401).json({ error: 'Invalid OIDC token or repository mismatch' });
      return;
    }

    logger.debug({ repoName }, 'OIDC token verified successfully');

    // Step 2: Generate Ed25519 Signature for Soroban Smart Contract
    const signatureBuffer = signIdentityPayload(repoName, stellarAddress, ORACLE_SECRET_KEY);

    logger.info({ repoName, stellarAddress }, 'Identity signature generated');

    res.json({
      status: 'success',
      repoName,
      stellarAddress,
      oracleSignature: signatureBuffer.toString('hex'),
    });
  } catch (err: any) {
    logger.error({ error: err.message, stack: err.stack }, 'Error processing /verify request');
    res.status(500).json({ error: err.message || 'Internal server error' });
  }
});

/**
 * Health check endpoint
 */
app.get('/health', (_req: Request, res: Response): void => {
  res.json({ status: 'ok', timestamp: new Date().toISOString() });
});

app.listen(PORT, () => {
  logger.info(`🛡️ Sustaina Oracle Service listening on port ${PORT}`);
});
