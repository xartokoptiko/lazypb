#!/bin/bash

# Define paths
CONFIG_DIR="$HOME/.config/lazyp"
DATA_DIR="$CONFIG_DIR/data"
JSON_FILE="$DATA_DIR/projects.json"
BINARY_DIR="target/debug"
BINARY_FILE="lazypb"
DESTINATION_DIR="/bin"
DESTINATION_BINARY="$DESTINATION_DIR/lazypb"
LOG_FILE="$CONFIG_DIR/lazyp.log"

# Function to log messages
log() {
    echo "$(date): $1" >> "$LOG_FILE"
}

# Check if config directory exists, if not, create it
if [ ! -d "$CONFIG_DIR" ]; then
    mkdir -p "$CONFIG_DIR"
    log "Created directory: $CONFIG_DIR"
fi

# Check if data directory exists, if not, create it
if [ ! -d "$DATA_DIR" ]; then
    mkdir -p "$DATA_DIR"
    log "Created directory: $DATA_DIR"
fi

# Check if JSON file exists, if not, create it
if [ ! -f "$JSON_FILE" ]; then
    echo '{"projects": []}' > "$JSON_FILE"
    log "Created file: $JSON_FILE"
fi

# Check if binary exists
if [ -f "$BINARY_DIR/$BINARY_FILE" ]; then
    # Copy binary to destination directory
    cp "$BINARY_DIR/$BINARY_FILE" "$DESTINATION_DIR"
    log "Copied binary to: $DESTINATION_BINARY"
else
    log "Error: Binary not found at $BINARY_DIR/$BINARY_FILE"
fi

# Create lazyp file in /bin directory
# shellcheck disable=SC2016
echo '#!/bin/bash

# Check if the provided arguments match ":lazyp project -s"
if [ "$1" = "project" ] && [ "$2" = "-s" ]; then
    # Get the current working directory
    CURRENT_DIR=$(pwd)

    # Execute the binary from the current working directory
    /bin/lazypb "$@" "$CURRENT_DIR"
else
    # Execute the binary without modifying the arguments
    /bin/lazypb "$@"
fi' > "$DESTINATION_DIR/lazyp"
chmod +x "$DESTINATION_DIR/lazyp"
log "Created script: $DESTINATION_DIR/lazyp"
