#!/bin/bash
# Build script for devSpin lintRunner Docker image
# This Dockerfile is distributed with devspin-cli and builds the lintRunner tool

set -e

IMAGE_NAME="devspin/lintrunner"
VERSION="0.1.0"

# Build from parent directory to include lintRunner source
cd "$(dirname "$0")/.."

echo "Building lintRunner Docker image from devSpin root..."
docker build -f devspin-cli/Dockerfile -t ${IMAGE_NAME}:${VERSION} -t ${IMAGE_NAME}:latest .

echo ""
echo "✅ Build complete!"
echo ""
echo "The image is now available as: ${IMAGE_NAME}:${VERSION}"
echo ""
echo "This image is automatically used by devSpin when:"
echo "  spin-features:"
echo "    lint:"
echo "      enabled: true"
echo ""
echo "Manual usage:"
echo "  docker run --rm -v \$(pwd):/project ${IMAGE_NAME} -o lint"
echo "  docker run --rm -v \$(pwd):/project ${IMAGE_NAME} -o format -f"
