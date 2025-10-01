FROM ubuntu:24.04

# Install dependencies
RUN apt-get update && apt-get install -y \
    curl git build-essential pkg-config libssl-dev clang \
    && rm -rf /var/lib/apt/lists/*

# Install rustc and cargo using rustup
RUN apt-get update && \
    apt-get install -y curl build-essential && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    . "$HOME/.cargo/env" && \
    rustup target add wasm32v1-none && \
    rm -rf /var/lib/apt/lists/*

# Make sure Cargo is in PATH for all future commands
ENV PATH="/root/.cargo/bin:${PATH}"
# Install stellar cli
RUN cargo install --locked stellar-cli@23.1.3
# Install soroban cli
RUN soroban completion bash > /etc/bash_completion.d/soroban


