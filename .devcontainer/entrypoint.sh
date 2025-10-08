#!/bin/bash
set -e

# Ensure DEVSEED exists
if [ -z "$DEVSEED" ]; then
    echo "ERROR: DEVSEED environment variable not set"
    exit 1
fi

# Write Stellar identity TOML
mkdir -p /root/.config/stellar/identity
cat > /root/.config/stellar/identity/stellar_ahmet.toml <<EOT
seed_phrase = "$DEVSEED"
EOT

# Continue with CMD
exec "$@"
