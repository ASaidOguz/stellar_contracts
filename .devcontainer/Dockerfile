# Base image
FROM rust:slim

# Install dependencies
RUN apt-get update && apt-get install -y \
    curl git build-essential pkg-config libssl-dev clang bash-completion \
    && rm -rf /var/lib/apt/lists/*

# Rust nightly + WASM target
RUN rustup install nightly && rustup default nightly
RUN rustup target add wasm32v1-none

# Linuxbrew + stellar-cli
RUN /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)" \
    && echo 'eval "$(/home/linuxbrew/.linuxbrew/bin/brew shellenv)"' >> /root/.profile \
    && eval "$(/home/linuxbrew/.linuxbrew/bin/brew shellenv)" \
    && brew install stellar-cli

# Ensure Cargo in PATH
ENV PATH="/usr/local/cargo/bin:${PATH}"

CMD ["bash"]
