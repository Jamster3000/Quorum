#!/bin/bash
set -e

# This script lives in client/
CLIENT_DIR="$(cd "$(dirname "$0")" && pwd)"

# src-tauri is either inside client/ (usual Tauri layout) or next to it
if [ -d "$CLIENT_DIR/src-tauri" ]; then
    TAURI_DIR="$CLIENT_DIR/src-tauri"
elif [ -d "$CLIENT_DIR/../src-tauri" ]; then
    TAURI_DIR="$(cd "$CLIENT_DIR/../src-tauri" && pwd)"
else
    echo "Could not find src-tauri in $CLIENT_DIR or its parent folder."
    exit 1
fi

total=5
current=0

echo "Running checks..."
echo "Tauri dir: $TAURI_DIR"
echo "Client dir: $CLIENT_DIR"

# FMT
current=$((current + 1))
echo ""
echo "[$current/$total] Formatting Rust code (src-tauri FMT)"
cd "$TAURI_DIR"
cargo fmt --all

# Clippy
current=$((current + 1))
echo "[$current/$total] Linting Rust code (src-tauri Clippy)"
cargo clippy -- -W warnings

# Cargo check
current=$((current + 1))
echo "[$current/$total] Cargo check..."
cargo check

# TypeScript checking
current=$((current + 1))
echo "[$current/$total] Type checking..."
cd "$CLIENT_DIR"
npm run check

# Storybook tests
current=$((current + 1))
echo "[$current/$total] Storybook tests..."
npm run test:storybook

echo ""
echo "All checks passed!"