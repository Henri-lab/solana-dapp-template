/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_PROGRAM_ID: string
  readonly VITE_SOLANA_NETWORK: string
}

interface ImportMeta {
  readonly env: ImportMetaEnv
}