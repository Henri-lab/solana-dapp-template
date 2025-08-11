import { PublicKey } from '@solana/web3.js';

export interface GameState {
  authority: PublicKey;
  players: number;
  totalScore: number;
  isActive: boolean;
  createdAt: number;
}

export interface Player {
  authority: PublicKey;
  score: number;
  gamesPlayed: number;
  joinedAt: number;
}

export interface ProgramAccount<T> {
  publicKey: PublicKey;
  account: T;
}

export type Network = 'localnet' | 'devnet' | 'mainnet-beta';