#!/bin/bash
# Build script for lintRunner Docker image

set -e

IMAGE_NAME="devspin/lintrunner"
VERSION="0.1.0"

echo "Building lintRunner Docker image..."
docker build -t ${IMAGE_NAME}:${VERSION} -t ${IMAGE_NAME}:latest .

echo ""
echo "Build complete!"
echo ""
echo "Usage examples:"
echo "  # Lint a Rust project"
echo "  docker run --rm -v \$(pwd):/project ${IMAGE_NAME} -o lint"
echo ""
echo "  # Format a specific file"
echo "  docker run --rm -v \$(pwd):/project ${IMAGE_NAME} -o format myfile.rs"
echo ""
echo "  # Auto-format without prompts"
echo "  docker run --rm -v \$(pwd):/project ${IMAGE_NAME} -o format -f"
