#!/bin/bash
# Thundermail Multi-Platform Build Script
# This script builds Thundermail for all supported platforms
# Requirements:
#   - Rust (stable)
#   - For Linux builds: Cross-compilers (see below)
#   - For Windows builds: mingw-w64 (brew install mingw-w64)
#   - For macOS builds: Standard Xcode tools

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "=========================================="
echo "  Thundermail Multi-Platform Build"
echo "=========================================="

# Build function
build_target() {
    local target=$1
    local name=$2
    
    echo -e "\n${YELLOW}Building for $name...${NC}"
    
    if cargo build --release --target "$target" -p thundermail 2>&1; then
        echo -e "${GREEN}✓ $name build successful${NC}"
        
        # Copy to release directory
        mkdir -p release
        if [[ "$target" == *"windows"* ]]; then
            cp "target/$target/release/thundermail.exe" "release/thundermail-$name.exe"
        else
            cp "target/$target/release/thundermail" "release/thundermail-$name"
        fi
        return 0
    else
        echo -e "${RED}✗ $name build failed${NC}"
        return 1
    fi
}

# Install cross-compilation targets
echo "Installing Rust cross-compilation targets..."
rustup target add x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu x86_64-pc-windows-gnu aarch64-pc-windows-gnu x86_64-apple-darwin 2>/dev/null || true

# Track results
FAILED=()

# Build for each platform
echo -e "\n${YELLOW}Starting builds...${NC}\n"

# macOS (native - works on macOS only)
if [[ "$OSTYPE" == "darwin"* ]]; then
    build_target "aarch64-apple-darwin" "darwin-arm64" || FAILED+=("darwin-arm64")
    build_target "x86_64-apple-darwin" "darwin-intel" || FAILED+=("darwin-intel")
fi

# Linux (requires cross-compilers)
if command -v x86_64-linux-gnu-gcc &> /dev/null; then
    build_target "x86_64-unknown-linux-gnu" "linux-x64" || FAILED+=("linux-x64")
else
    echo -e "${YELLOW}⚠ Linux x64 build skipped: x86_64-linux-gnu-gcc not found${NC}"
    echo "  Install with: brew install x86_64-linux-gnu-gcc (if available)"
fi

if command -v aarch64-linux-gnu-gcc &> /dev/null; then
    build_target "aarch64-unknown-linux-gnu" "linux-arm64" || FAILED+=("linux-arm64")
else
    echo -e "${YELLOW}⚠ Linux ARM64 build skipped: aarch64-linux-gnu-gcc not found${NC}"
    echo "  Install with: brew install aarch64-linux-gnu-gcc (if available)"
fi

# Windows (requires mingw-w64)
if command -v x86_64-w64-mingw32-gcc &> /dev/null; then
    build_target "x86_64-pc-windows-gnu" "windows-x64" || FAILED+=("windows-x64")
else
    echo -e "${YELLOW}⚠ Windows x64 build skipped: mingw-w64 not found${NC}"
    echo "  Install with: brew install mingw-w64"
fi

if command -v aarch64-w64-mingw32-gcc &> /dev/null; then
    build_target "aarch64-pc-windows-gnu" "windows-arm64" || FAILED+=("windows-arm64")
else
    echo -e "${YELLOW}⚠ Windows ARM64 build skipped: aarch64-w64-mingw32-gcc not found${NC}"
    echo "  Note: Full mingw-w64 with ARM64 support required"
fi

# Summary
echo -e "\n=========================================="
echo "  Build Summary"
echo "=========================================="

if [ -d "release" ]; then
    echo -e "\n${GREEN}Built artifacts:${NC}"
    ls -lh release/
else
    echo -e "${YELLOW}No release artifacts found${NC}"
fi

if [ ${#FAILED[@]} -eq 0 ]; then
    echo -e "\n${GREEN}All builds successful!${NC}"
    exit 0
else
    echo -e "\n${RED}Failed builds:${NC}"
    for f in "${FAILED[@]}"; do
        echo "  - $f"
    done
    exit 1
fi
