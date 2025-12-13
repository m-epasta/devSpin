###############################################################################
# DevSpin LintRunner Container                                               #
###############################################################################

# PURPOSE:
# This Dockerfile builds a multi-language linting and formatting container that
# bundles lintRunner (a V language tool) with all the necessary language toolchains
# to enable cross-language code quality checks in devSpin CLI projects.

# WHAT IT DOES:
# 1. Installs system dependencies and base tools (curl, git, build-essential, etc.)
# 2. Compiles and installs V language compiler for building lintRunner
# 3. Installs complete toolchain for each supported language:
#    - Rust: rustup with clippy and rustfmt for Rust linting/formatting
#    - Go: go compiler with gofmt for Go formatting
#    - Node.js: npm with ESLint, Prettier, TypeScript for JS/TS linting/formatting
#    - Python: pip with pylint and black for Python linting/formatting
# 4. Builds lintRunner binary from source using V compiler
# 5. Creates non-root user and mounts /project as workspace

# SUPPORTED LANGUAGES via lintRunner:
# - V: v fmt (V language's built-in formatter)
# - JavaScript/TypeScript: ESLint + Prettier + TypeScript compiler
# - Python: pylint + black
# - Go: gofmt
# - Rust: cargo check + cargo fmt
# - JSON/YAML/TOML: prettier OR cargo (for TOML)

# BUILD REQUIREMENTS:
# Must be built from parent directory (devSpin root) due to COPY ../lintRunner
# Use the included build script: ./devspin-cli/build-docker.sh

# DEPLOYMENT:
# This container is automatically used by devSpin CLI when lint features are enabled,
# providing a consistent, portable multi-language development environment.

###############################################################################

FROM ubuntu:22.04

# Prevent interactive prompts during package installation
ENV DEBIAN_FRONTEND=noninteractive

# Install all system dependencies in one RUN layer for better caching
RUN apt-get update && apt-get install -y \
    curl \
    git \
    wget \
    unzip \
    build-essential \
    ca-certificates \
    gnupg \
    python3 \
    python3-pip \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# ============================================
# Install V Language
# ============================================
RUN git clone --depth 1 https://github.com/vlang/v /opt/vlang && \
    cd /opt/vlang && \
    make && \
    ln -s /opt/vlang/v /usr/local/bin/v && \
    rm -rf /opt/vlang/.git

# ============================================
# Install Rust
# ============================================
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup component add clippy rustfmt && \
    rustup self uninstall -y 2>/dev/null || true  # Clean up installation files

# ============================================
# Install Go
# ============================================
RUN wget -q https://go.dev/dl/go1.21.5.linux-amd64.tar.gz && \
    tar -C /usr/local -xzf go1.21.5.linux-amd64.tar.gz && \
    rm go1.21.5.linux-amd64.tar.gz
ENV PATH="/usr/local/go/bin:${PATH}"

# ============================================
# Install Node.js (for ESLint, Prettier)
# ============================================
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
    apt-get install -y nodejs && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Install global JS/TS tools with version pinning for reproducibility
RUN npm install -g \
    eslint@^8.0.0 \
    prettier@^3.0.0 \
    typescript@^5.0.0

# ============================================
# Install Python tools
# ============================================
RUN pip3 install --no-cache-dir pylint black

# ============================================
# Copy and build lintRunner
# ============================================
WORKDIR /app

# Copy lintRunner source - NOTE: This requires building from devSpin root to include lintRunner source
COPY ../lintRunner /app/lintRunner

RUN cd /app/lintRunner && v -prod . -o /usr/local/bin/lintRunner && \
    rm -rf /app/lintRunner  # Clean up source after build

# ============================================
# Set up project workspace
# ============================================
WORKDIR /project

# Create non-root user for security (recommended)
RUN groupadd -r lintrunner && useradd -r -g lintrunner lintrunner && \
    chown -R lintrunner:lintrunner /project
USER lintrunner

# Entrypoint is the lintRunner binary
ENTRYPOINT ["lintRunner"]

# Default command shows help
CMD ["--help"]
