#!/bin/bash
cargo build --release
# Determine the absolute path to the script's directory
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

SALIBREW_PATH="$SCRIPT_DIR/target/release/sali"

chmod +x target/release/sali
sudo ln -sf "$SALIBREW_PATH" /usr/local/bin/sali

echo "sali has been set up successfully."
