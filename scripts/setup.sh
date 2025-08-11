#!/bin/bash

set -e

echo "🚀 Setting up Solana Game DApp..."

# Check if required tools are installed
check_command() {
    if ! command -v $1 &> /dev/null; then
        echo "❌ $1 is not installed. Please install it first."
        exit 1
    fi
}

echo "📋 Checking prerequisites..."
check_command "solana"
check_command "anchor"
check_command "node"
check_command "yarn"

# Set Solana to localnet
echo "🔧 Configuring Solana CLI..."
solana config set --url localhost

# Generate keypair if not exists
if [ ! -f ~/.config/solana/id.json ]; then
    echo "🔑 Generating new keypair..."
    solana-keygen new --no-bip39-passphrase
fi

# Install dependencies
echo "📦 Installing root dependencies..."
yarn install

echo "📦 Installing frontend dependencies..."
cd app && yarn install && cd ..

# Build program
echo "🏗️ Building Anchor program..."
anchor build

# Generate IDL
echo "📄 Generating IDL types..."
anchor idl parse --file target/idl/anchor_20250808.json --out app/src/idl/

echo "✅ Setup complete!"
echo ""
echo "🎮 Next steps:"
echo "1. Start local validator: yarn dev:validator"
echo "2. Deploy program: yarn deploy"  
echo "3. Start frontend: yarn dev:frontend"
echo "4. Or run both: yarn dev"