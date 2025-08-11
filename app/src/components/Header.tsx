import React from 'react';
import { useWallet } from '@solana/wallet-adapter-react';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import { useSolana } from '@/contexts/SolanaProvider';

const Header: React.FC = () => {
  const { publicKey } = useWallet();
  const { network, setNetwork } = useSolana();

  return (
    <header className="header">
      <div className="container">
        <h1>Solana Game DApp</h1>
        
        <div className="controls">
          <select 
            value={network} 
            onChange={(e) => setNetwork(e.target.value as any)}
            className="network-select"
          >
            <option value="localnet">Localnet</option>
            <option value="devnet">Devnet</option>
            <option value="mainnet-beta">Mainnet</option>
          </select>
          
          <WalletMultiButton />
        </div>
      </div>
      
      {publicKey && (
        <div className="wallet-info">
          Connected: {publicKey.toString().slice(0, 8)}...{publicKey.toString().slice(-8)}
        </div>
      )}
    </header>
  );
};

export default Header;