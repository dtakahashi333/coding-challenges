#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -e

# Move to the rust directory if the script is run from the project root
# If you keep this script inside the /rust folder, you can remove the next line
# cd "$(dirname "$0")/rust"

echo "🎨 Checking formatting..."
cargo fmt --all -- --check

echo "🔍 Running Clippy (Linter)..."
cargo clippy --all-targets --all-features -- -D warnings

echo "🧪 Running tests..."
cargo test --all-features --verbose

echo "✅ All checks passed! Ready to commit."