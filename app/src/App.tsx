import React from 'react';
import { SolanaProvider } from '@/contexts/SolanaProvider';
import Header from '@/components/Header';
import './App.css';

const App: React.FC = () => {
  return (
    <SolanaProvider>
      <div className="app">
        <Header />
        <main className="main">
          <div className="container">
            <h2>Welcome to Solana Game DApp</h2>
            <p>Connect your wallet to start playing!</p>
            
            <div className="game-section">
              <h3>Game Features</h3>
              <ul>
                <li>Initialize Game State</li>
                <li>Create Player Profile</li>
                <li>Update Player Score</li>
                <li>View Leaderboard</li>
              </ul>
            </div>
          </div>
        </main>
      </div>
    </SolanaProvider>
  );
};

export default App;