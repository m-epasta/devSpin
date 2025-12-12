# DevSpin LintRunner Container
# A multi-language linting and formatting tool

FROM ubuntu:22.04

# Prevent interactive prompts during package installation
ENV DEBIAN_FRONTEND=noninteractive

# Install base dependencies
RUN apt-get update && apt-get install -y \
    curl \
    git \
    wget \
    unzip \
    build-essential \
    ca-certificates \
    gnupg \
    && rm -rf /var/lib/apt/lists/*

# ============================================
# Install V Language
# ============================================
RUN git clone --depth 1 https://github.com/vlang/v /opt/vlang && \
    cd /opt/vlang && \
    make && \
    ln -s /opt/vlang/v /usr/local/bin/v

# ============================================
# Install Rust
# ============================================
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup component add clippy rustfmt

# ============================================
# Install Go
# ============================================
RUN wget https://go.dev/dl/go1.21.5.linux-amd64.tar.gz && \
    tar -C /usr/local -xzf go1.21.5.linux-amd64.tar.gz && \
    rm go1.21.5.linux-amd64.tar.gz
ENV PATH="/usr/local/go/bin:${PATH}"

# ============================================
# Install Node.js (for ESLint, Prettier)
# ============================================
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
    apt-get install -y nodejs && \
    rm -rf /var/lib/apt/lists/*

# Install global JS/TS tools
RUN npm install -g eslint prettier typescript

# ============================================
# Install Python tools
# ============================================
RUN apt-get update && apt-get install -y python3 python3-pip && \
    pip3 install --no-cache-dir pylint black && \
    rm -rf /var/lib/apt/lists/*

# ============================================
# Copy and build lintRunner
# ============================================
WORKDIR /app
COPY . /app/lintRunner

RUN cd /app/lintRunner && v -prod . -o /usr/local/bin/lintRunner

# Set working directory for mounted projects
WORKDIR /project

# Entrypoint is the lintRunner binary
ENTRYPOINT ["lintRunner"]

# Default command shows help
CMD ["--help"]
