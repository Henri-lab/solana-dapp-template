#!/bin/bash

set -e

if [ -z "$1" ]; then
    echo "❌ Usage: ./scripts/rename-project.sh <new-project-name>"
    echo "📝 Example: ./scripts/rename-project.sh my-awesome-dapp"
    exit 1
fi

NEW_NAME=$1
OLD_NAME="anchor_20250808"
OLD_DISPLAY_NAME="anchor-20250808"

echo "🚀 Renaming project to: $NEW_NAME"

# Convert to snake_case for Rust
SNAKE_CASE=$(echo "$NEW_NAME" | tr '[:upper:]' '[:lower:]' | tr '-' '_')
# Convert to kebab-case for package names
KEBAB_CASE=$(echo "$NEW_NAME" | tr '[:upper:]' '[:lower:]' | tr '_' '-')

echo "📝 Snake case: $SNAKE_CASE"
echo "📝 Kebab case: $KEBAB_CASE"

# Update Cargo.toml files
echo "🦀 Updating Rust configurations..."
sed -i '' "s/name = \"$OLD_NAME\"/name = \"$SNAKE_CASE\"/g" programs/anchor-20250808/Cargo.toml
sed -i '' "s/name = \"$OLD_NAME\"/name = \"$SNAKE_CASE\"/g" programs/anchor-20250808/Cargo.toml

# Update lib.rs
sed -i '' "s/pub mod $OLD_NAME/pub mod $SNAKE_CASE/g" programs/anchor-20250808/src/lib.rs
sed -i '' "s/$OLD_NAME::/$SNAKE_CASE::/g" programs/anchor-20250808/src/lib.rs

# Update Anchor.toml
sed -i '' "s/$OLD_DISPLAY_NAME = /$KEBAB_CASE = /g" Anchor.toml

# Update package.json files
echo "📦 Updating package configurations..."
sed -i '' "s/\"name\": \"solana-game-dapp\"/\"name\": \"$KEBAB_CASE\"/g" package.json
sed -i '' "s/\"name\": \"solana-dapp-frontend\"/\"name\": \"$KEBAB_CASE-frontend\"/g" app/package.json

# Update frontend references
echo "⚛️ Updating frontend references..."
sed -i '' "s/anchor_20250808/$SNAKE_CASE/g" app/src/types/index.ts
sed -i '' "s/Anchor20250808/$(echo $SNAKE_CASE | sed 's/_\(.\)/\U\1/g')/g" app/src/types/index.ts

# Update test files
echo "🧪 Updating test files..."
if [ -f "tests/anchor-20250808.ts" ]; then
    mv "tests/anchor-20250808.ts" "tests/$KEBAB_CASE.ts"
fi
sed -i '' "s/$OLD_NAME/$SNAKE_CASE/g" "tests/$KEBAB_CASE.ts" 2>/dev/null || true

# Update directory names
echo "📁 Renaming directories..."
if [ -d "programs/anchor-20250808" ]; then
    mv "programs/anchor-20250808" "programs/$KEBAB_CASE"
fi

# Update README and docs
echo "📚 Updating documentation..."
sed -i '' "s/anchor-20250808/$KEBAB_CASE/g" README.md
sed -i '' "s/$OLD_NAME/$SNAKE_CASE/g" docs/API.md
sed -i '' "s/anchor-20250808/$KEBAB_CASE/g" docs/ARCHITECTURE.md

echo "✅ Project renamed successfully!"
echo ""
echo "📋 Next steps:"
echo "1. Review and commit your changes: git add . && git commit -m 'Rename project to $NEW_NAME'"
echo "2. Update your program ID: anchor keys list"
echo "3. Update .env files with new program ID"
echo "4. Run: ./scripts/setup.sh"
echo ""
echo "🎯 Your project is now ready: $NEW_NAME"