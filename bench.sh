#!/bin/bash

# 1. Setup: Generate a 100,000 line sample file (~25MB)
echo "Generating test data..."
echo '{"id":"123456789","type":"PushEvent","actor":{"login":"rust-dev"},"repo":{"name":"writeonlycode/octo-flow"},"created_at":"2026-03-12T14:00:00Z"}' > sample.json
for i in {1..16}; do cat sample.json sample.json > temp.json && mv temp.json sample.json; done
# This gives us roughly 65,536 lines.

echo "--- Performance Battle ---"

# 2. Benchmark JQ (The standard but slow way)
echo "Running JQ..."
time jq -c 'select(.type == "PushEvent")' sample.json > /dev/null

# 3. Benchmark GREP (The fast but "dumb" way)
echo -e "\nRunning GREP..."
time grep '"type":"PushEvent"' sample.json > /dev/null

# 4. Benchmark OCTO-FLOW (Your optimized Rust tool)
echo -e "\nRunning OCTO-FLOW..."
cargo build --release > /dev/null
time ./target/release/octo-flow --input sample.json --event PushEvent > /dev/null

# Cleanup
# rm sample.json
