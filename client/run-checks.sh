#!/bin/bash
set -e

CLIENT_DIR="$(cd "$(dirname "$0")" && pwd)"
TAURI_DIR="$CLIENT_DIR/src-tauri"

total=5
current=0

echo "Running checks..."

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