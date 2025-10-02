# Start from a valid Rust base image (slim variant is lighter)
FROM rust:slim

# Install dependencies needed for building Soroban contracts
RUN apt-get update && apt-get install -y \
    curl git build-essential pkg-config libssl-dev clang bash-completion \
    && rm -rf /var/lib/apt/lists/*

# Install and set Rust nightly as default
RUN rustup install nightly && rustup default nightly

# Add WASM target for Soroban smart contracts (nightly supports it best)
RUN rustup target add wasm32v1-none

# Install Homebrew (Linuxbrew) for stellar-cli
RUN /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)" \
    && echo 'eval "$(/home/linuxbrew/.linuxbrew/bin/brew shellenv)"' >> /root/.profile \
    && eval "$(/home/linuxbrew/.linuxbrew/bin/brew shellenv)" \
    && brew install stellar-cli

# Ensure Cargo is in PATH
ENV PATH="/usr/local/cargo/bin:${PATH}"
