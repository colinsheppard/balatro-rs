#!/bin/bash
# Convenience script to activate the pylatro development environment
# Usage: source activate_env.sh

if [ -f ".env/bin/activate" ]; then
    source .env/bin/activate
    echo "✅ Activated pylatro development environment"
    echo "🐍 Python: $(python --version)"
    echo "📦 Installed packages:"
    pip list | grep -E "(maturin|pylatro)" || echo "   No relevant packages found"
else
    echo "❌ Virtual environment not found. Run ./setup.sh first."
    exit 1
fi
