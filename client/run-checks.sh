#!/bin/bash
set -e

echo "Running checks..."

# FMT
echo ""
echo "[1/4] Formatting Rust code (src-tauri FMT)"
cd src-tauri
cargo fmt --all
cd ..

# Clippy
echo "[2/4] Linting Rust code (src-tauri Clippy)"
cd src-tauri
cargo clippy -- -W warnings
cd ..

# TypeScript checking
echo "[3/4] Type checking..."
npm run check

# Storybook tests
echo "[4/4] Storybook tests..."
npm run test:storybook

echo ""
echo "All checks passed!"