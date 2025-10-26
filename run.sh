#!/usr/bin/env bash
# Wrapper script for dogmv to ensure proper environment variables

# Disable GSettings backend to avoid schema errors
export GSETTINGS_BACKEND=memory

# Run the application
exec ./target/release/dogmv "$@"
