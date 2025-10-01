FROM rust:1.84-slim

# Install dependencies
RUN apt-get update && apt-get install -y \
    curl git build-essential pkg-config libssl-dev clang bash-completion \
    && rm -rf /var/lib/apt/lists/*

# Add WASM target for Soroban smart contracts
RUN rustup target add wasm32v1-none

# Install Homebrew (Linuxbrew) for stellar-cli
RUN /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)" \
    && echo 'eval "$(/home/linuxbrew/.linuxbrew/bin/brew shellenv)"' >> /root/.profile \
    && eval "$(/home/linuxbrew/.linuxbrew/bin/brew shellenv)" \
    && brew install stellar-cli

# Enable soroban completion (via stellar-cli)
RUN stellar completion bash > /etc/bash_completion.d/stellar \
    && stellar completion bash > /etc/bash_completion.d/soroban

# Ensure Cargo is in PATH
ENV PATH="/usr/local/cargo/bin:${PATH}"
