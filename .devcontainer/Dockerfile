FROM ubuntu:24.04

# Install dependencies
RUN apt-get update && apt-get install -y \
    curl git build-essential pkg-config libssl-dev clang bash-completion \
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

# Install Homebrew (Linuxbrew) for stellar-cli only
RUN /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)" \
    && echo 'eval "$(/home/linuxbrew/.linuxbrew/bin/brew shellenv)"' >> /root/.profile \
    && eval "$(/home/linuxbrew/.linuxbrew/bin/brew shellenv)" \
    && brew install stellar-cli

# Install stellar cli via cargo
RUN brew install stellar-cli

# Enable soroban completion
RUN soroban completion bash > /etc/bash_completion.d/soroban
