#!/bin/bash

set -e

echo "🚀 Deploying Solana Game DApp..."

# Build the program
echo "🏗️ Building program..."
anchor build

# Deploy to configured network
echo "📡 Deploying to network..."
anchor deploy

# Copy IDL to frontend
echo "📄 Copying IDL to frontend..."
mkdir -p app/src/idl
cp target/idl/anchor_20250808.json app/src/idl/

# Update program ID in frontend
PROGRAM_ID=$(anchor keys list | grep "anchor_20250808" | cut -d' ' -f2)
echo "📝 Program ID: $PROGRAM_ID"

# Create .env file for frontend
cat > app/.env.local << EOF
VITE_PROGRAM_ID=$PROGRAM_ID
VITE_SOLANA_NETWORK=localnet
EOF

echo "✅ Deployment complete!"
echo "📝 Program ID: $PROGRAM_ID"
echo "🎮 Run 'yarn dev:frontend' to start the frontend"