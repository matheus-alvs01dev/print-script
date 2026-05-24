#!/bin/bash
set -euo pipefail

SAVE_DIR="${HOME}/Pictures/Screenshots"
mkdir -p "$SAVE_DIR"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
SAVE_PATH="${SAVE_DIR}/screenshot_${TIMESTAMP}.png"

GEOM=$(slurp 2>/dev/null || true)
if [ -z "$GEOM" ]; then
  notify-send "Screenshot" "Cancelled"
  exit 1
fi

grim -g "$GEOM" -t png "$SAVE_PATH"
satty --filename "$SAVE_PATH" --fullscreen --initial-tool crop -o "$SAVE_PATH"

if [ -f "$SAVE_PATH" ]; then
  wl-copy < "$SAVE_PATH"
  notify-send "Screenshot" "Saved to $SAVE_PATH"
fi
