# Start from Rust slim
FROM rust:slim

# Install dependencies
RUN apt-get update && apt-get install -y \
    curl git build-essential pkg-config libssl-dev clang bash-completion \
    && rm -rf /var/lib/apt/lists/*

# Install Rust nightly & WASM target
RUN rustup install nightly && rustup default nightly
RUN rustup target add wasm32v1-none

# Install Linuxbrew & stellar-cli
RUN /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)" \
    && echo 'eval "$(/home/linuxbrew/.linuxbrew/bin/brew shellenv)"' >> /root/.profile \
    && eval "$(/home/linuxbrew/.linuxbrew/bin/brew shellenv)" \
    && brew install stellar-cli

# Ensure Cargo is in PATH
ENV PATH="/usr/local/cargo/bin:${PATH}"

# Create entrypoint script to write DEVSEED TOML at runtime
COPY <<EOF /entrypoint.sh
#!/bin/bash
set -e

# Check if DEVSEED exists
if [ -z "\$DEVSEED" ]; then
    echo "ERROR: DEVSEED environment variable not set"
    exit 1
fi

# Write the Stellar identity TOML
mkdir -p /root/.config/stellar/identity
cat > /root/.config/stellar/identity/stellar_ahmet.toml <<EOT
seed_phrase = "\$DEVSEED"
EOT

# Pass control to CMD
exec "\$@"
EOF

RUN chmod +x /entrypoint.sh
ENTRYPOINT ["/entrypoint.sh"]

# Default command
CMD ["bash"]
