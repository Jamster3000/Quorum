#!/bin/bash
set -e

total=5
current=0

echo "Running checks..."

# FMT
((current++))
echo ""
echo "[$current/$total] Formatting Rust code (src-tauri FMT)"
cd src-tauri
cargo fmt --all
cd ..

# Clippy
((current++))
echo "[$current/$total] Linting Rust code (src-tauri Clippy)"
cargo clippy -- -W warnings

# Cargo check
((current++))
echo "[$current/$total] Cargo check..."
cargo check

# TypeScript checking
((current++))
cd client
echo "[$current/$total] Type checking..."
npm run check

# Storybook tests
((current++))
echo "[$current/$total] Storybook tests..."
npm run test:storybook

echo ""
echo "All checks passed!"