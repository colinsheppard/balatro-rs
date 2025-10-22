#!/bin/bash
# Setup script for pylatro development environment

echo "🚀 Setting up pylatro development environment..."

# Create virtual environment
echo "📦 Creating Python virtual environment..."
python3 -m venv .env

# Activate virtual environment
echo "🔧 Activating virtual environment..."
source .env/bin/activate

# Install maturin
echo "🛠 Installing maturin..."
pip install maturin

# Build and install pylatro with Python 3.14 compatibility
echo "🔨 Building pylatro package..."
PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 maturin develop

echo "✅ Setup complete!"
echo ""
echo "To activate the environment in the future:"
echo "  source .env/bin/activate"
echo ""
echo "To test the installation:"
echo "  python test/main.py"
echo ""
echo "To rebuild after changes:"
echo "  PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 maturin develop"
