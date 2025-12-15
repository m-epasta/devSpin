#!/usr/bin/env bash
set -euo pipefail

# build-docker.sh - build the spn-demo Docker image
# Usage: ./build-docker.sh [image-name[:tag]] [dockerfile] [context]
# Defaults: image-name=spn-demo:latest  dockerfile=Dockerfile  context=.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

IMAGE_NAME="${1:-spn-demo:latest}"
DOCKERFILE="${2:-Dockerfile}"
CONTEXT="${3:-.}"

echo "Building image: $IMAGE_NAME"
echo "Dockerfile: $DOCKERFILE"
echo "Context: $CONTEXT"

default_builder() {
  docker build -t "$IMAGE_NAME" -f "$DOCKERFILE" "$CONTEXT"
}

# Ensure docker is available
if ! command -v docker >/dev/null 2>&1; then
  echo "Error: docker is not installed or not on PATH." >&2
  exit 2
fi

# Run build
if default_builder; then
  echo "Built $IMAGE_NAME"
  exit 0
else
  echo "Docker build failed" >&2
  exit 1
fi
