#!/bin/bash

# Exit immediately if any command fails
set -e

# Assuming you're running this from the workspace root
# Replace these paths with your actual crate paths
LIBRARY_PATH="./boxy-cli"
BINARY_PATH="../binary-testing"

echo "Building library crate..."
cd "$LIBRARY_PATH"
cargo build

echo "Running binary crate..."
cd "$BINARY_PATH"
cargo run