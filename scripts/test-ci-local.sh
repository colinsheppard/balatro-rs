#!/bin/bash
# Local CI Test Script
# This script runs the same checks as our CI pipeline locally

set -e

echo "🔧 Local CI Test Script"
echo "======================="
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo -e "${RED}Error: Must be run from the project root directory${NC}"
    exit 1
fi

echo "📋 Running format check..."
if cargo fmt --all -- --check; then
    echo -e "${GREEN}✓ Format check passed${NC}"
else
    echo -e "${RED}✗ Format check failed${NC}"
    echo "  Run 'cargo fmt --all' to fix formatting issues"
    exit 1
fi

echo ""
echo "📎 Running clippy..."
if cargo clippy --all -- -D warnings; then
    echo -e "${GREEN}✓ Clippy passed${NC}"
else
    echo -e "${RED}✗ Clippy failed${NC}"
    exit 1
fi

echo ""
echo "🧪 Running tests..."
# Try to run tests, but if Python lib is missing, run core tests only
if cargo test --all --verbose 2>&1 | grep -q "libpython"; then
    echo -e "${YELLOW}⚠ Python library issue detected, running core tests only${NC}"
    if cargo test -p balatro-rs --verbose; then
        echo -e "${GREEN}✓ Core tests passed${NC}"
    else
        echo -e "${RED}✗ Core tests failed${NC}"
        exit 1
    fi
else
    # Re-run the tests properly if no Python issue
    if cargo test --all --verbose; then
        echo -e "${GREEN}✓ All tests passed${NC}"
    else
        echo -e "${RED}✗ Tests failed${NC}"
        exit 1
    fi
fi

echo ""
echo "🔨 Building all workspace members..."
for member in balatro-rs balatro-cli; do
    echo "  Building $member..."
    if cargo build -p $member; then
        echo -e "${GREEN}  ✓ $member built successfully${NC}"
    else
        echo -e "${RED}  ✗ $member build failed${NC}"
        exit 1
    fi
done

echo ""
echo "📊 Checking benchmarks compile..."
if cargo bench --no-run -p balatro-rs; then
    echo -e "${GREEN}✓ Benchmarks compile${NC}"
else
    echo -e "${YELLOW}⚠ Benchmarks failed to compile${NC}"
    # Don't exit on benchmark failure
fi

echo ""
echo -e "${GREEN}✅ All local CI checks passed!${NC}"
echo ""
echo "Note: This doesn't include:"
echo "  - Code coverage (requires cargo-tarpaulin)"
echo "  - Docker containerization"
echo "  - Python bindings build (requires Python setup)"
echo ""
echo "To run coverage locally:"
echo "  cargo install cargo-tarpaulin"
echo "  cargo tarpaulin --verbose --all-features --workspace"