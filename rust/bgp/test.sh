#!/usr/bin/env bash
set -euxo pipefail

# Check if connecting port 179 (BGP) succeeds
target/debug/bgp &
PID=$!
nc -vz -w 1 localhost 179
kill ${PID}
