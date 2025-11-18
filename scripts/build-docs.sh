#!/bin/bash
# Build both English and Chinese documentation

echo "Building English documentation..."
mdbook build

echo "Building Chinese documentation..."
mdbook build --config book.zh.toml

echo "Copying Chinese build to correct location..."
mkdir -p book/build/zh
cp -r book/build/zh/* book/build/zh/ 2>/dev/null || true

echo "Documentation built successfully!"
echo "English: book/build/index.html"
echo "Chinese: book/build/zh/index.html"
