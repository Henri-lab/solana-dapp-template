import { PublicKey, Connection } from '@solana/web3.js';

export const PROGRAM_ID = new PublicKey(
  import.meta.env.VITE_PROGRAM_ID || 'FqkHhvwGbuFBXmUBXpyDQGgGkU34fnhJxzttH1As4Nw9'
);

export const NETWORKS = {
  localnet: 'http://localhost:8899',
  devnet: 'https://api.devnet.solana.com',
  'mainnet-beta': 'https://api.mainnet-beta.solana.com',
} as const;

export const getConnection = (network: keyof typeof NETWORKS = 'localnet'): Connection => {
  return new Connection(NETWORKS[network]);
};

export const getGameStateAddress = (authority: PublicKey): [PublicKey, number] => {
  return PublicKey.findProgramAddressSync(
    [Buffer.from('game'), authority.toBuffer()],
    PROGRAM_ID
  );
};

export const getPlayerAddress = (authority: PublicKey): [PublicKey, number] => {
  return PublicKey.findProgramAddressSync(
    [Buffer.from('player'), authority.toBuffer()],
    PROGRAM_ID
  );
};