# Build and install the binary
install:
    cargo install --path . --force

# Build and install the man page
man:
    #!/usr/bin/env bash
    set -euo pipefail
    cargo clean
    cargo build
    MAN_PAGE_SRC=$(find target -name "git-commitgen.1")
    if [[ -z "$MAN_PAGE_SRC" ]]; then
        echo "Error: Man page not found. A build may have failed."
        exit 1
    fi
    echo "Found latest man page at: $MAN_PAGE_SRC"
    MAN_PAGE_DEST="/usr/local/share/man/man1/git-commitgen.1"

    echo "Installing man page to $MAN_PAGE_DEST"
    sudo mkdir -p "$(dirname "$MAN_PAGE_DEST")"
    sudo cp "$MAN_PAGE_SRC" "$MAN_PAGE_DEST"
    echo "Man page installed."
    echo "Updating man page database"
    sudo mandb


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
