#!/bin/bash
# Script to run the Balatro REPL with proper environment setup

set -e

echo "🎮 Starting Balatro REPL..."

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ] || [ ! -d "repl" ]; then
    echo "❌ Error: Please run this script from the pylatro directory"
    echo "   cd pylatro && ./run_repl.sh"
    exit 1
fi

# Activate the virtual environment and run the REPL
echo "🔧 Activating virtual environment..."
if [ -f ".env/bin/activate" ]; then
    source .env/bin/activate
    echo "✅ Activated pylatro development environment"
    echo "🐍 Python: $(python --version)"
    echo "🚀 Starting REPL..."
    python repl/repl.py "$@"
else
    echo "❌ Error: Virtual environment not found. Please run ./setup.sh first"
    exit 1
fi
