# Build and install the `git-commitgen` CLI for local development
install:
    cargo install --path . --force

# Run all code quality checks
check:
    cargo fmt --check
    cargo clippy -- -D warnings

# Generate a timestamped repomix file
repomix subcommand="":
    #!/usr/bin/env bash
    set -euo pipefail
    if [[ "{{subcommand}}" == "" ]] then
        npx repomix@latest . --style xml \
            -o repomix-git-commitgen-$(date +%Y%m%d-%H%M%S).xml \
            --ignore ""
    fi
