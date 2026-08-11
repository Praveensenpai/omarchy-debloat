#!/usr/bin/env bash

set -e

REPO="Praveensenpai/omarchy-debloat"
INSTALL_DIR="$HOME/.local/bin"

CYAN='\033[0;36m'
GREEN='\033[0;32m'
PURPLE='\033[0;35m'
BLUE='\033[0;34m'
YELLOW='\033[0;33m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${PURPLE}🚀 Installing omarchy-debloat (Rust CLI)...${NC}"

mkdir -p "$INSTALL_DIR"

ARCH=$(uname -m)
if [ "$ARCH" != "x86_64" ]; then
    echo -e "${RED}❌ Unsupported architecture: ${ARCH}. Currently only x86_64 is supported.${NC}"
    exit 1
fi

DOWNLOAD_URL="https://github.com/${REPO}/releases/latest/download/omarchy-debloat-x86_64-unknown-linux-gnu.tar.gz"

if command -v curl &>/dev/null && curl -sI "$DOWNLOAD_URL" | grep -q "302\|200"; then
    echo -e "${BLUE}📦 Downloading binary release from GitHub...${NC}"
    TMP_DIR=$(mktemp -d)
    curl -fsSL "$DOWNLOAD_URL" | tar -xz -C "$TMP_DIR"
    mv "$TMP_DIR/omarchy-debloat" "$INSTALL_DIR/omarchy-debloat"
    rm -rf "$TMP_DIR"
elif command -v cargo &>/dev/null; then
    echo -e "${YELLOW}⚙️ Release binary not found online. Building from source via cargo...${NC}"
    cargo install --git "https://github.com/${REPO}.git" --bin omarchy-debloat --root "$HOME/.local"
else
    echo -e "${RED}❌ Unable to download binary release or build via cargo.${NC}"
    exit 1
fi

chmod +x "$INSTALL_DIR/omarchy-debloat"

echo -e "${GREEN}✔ omarchy-debloat installed successfully to ${INSTALL_DIR}/omarchy-debloat!${NC}"
echo -e "${CYAN}Run 'omarchy-debloat --help' to get started.${NC}"
