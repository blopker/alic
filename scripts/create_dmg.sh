#!/usr/bin/env bash

set -e

# Create a DMG for the app
echo "Creating DMG..."
APP_NAME="alic"
create-dmg \
    --volname "$APP_NAME" \
    --window-pos 200 120 \
    --window-size 800 529 \
    --icon-size 130 \
    --text-size 14 \
    --icon "$APP_NAME.app" 260 250 \
    --hide-extension "$APP_NAME.app" \
    --app-drop-link 540 250 \
    --hdiutil-quiet \
    "$APP_NAME.dmg" \
    "build/macos/Build/Products/Release/$APP_NAME.app"

echo "DMG created successfully!"