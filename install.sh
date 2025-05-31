#!/usr/bin/env bash

set -e

REPO_NAME="airpods-cli"
BIN_NAME="airpods-cli"
INSTALL_DIR="$HOME/$REPO_NAME"
BIN_PATH="$INSTALL_DIR/target/release/$BIN_NAME"
ALIAS_NAME="airpods"

echo "📥 Cloning repo to $INSTALL_DIR"
git clone --depth=1 https://github.com/myferr/$REPO_NAME "$INSTALL_DIR"
cd "$INSTALL_DIR"

echo "🔧 Building release binary..."
cargo build --release
clear

echo -n "📝 What is your shell config file? [e.g. .zshrc / .bashrc]: "
read -r CONFIG_FILENAME

if [[ "$CONFIG_FILENAME" == /* ]]; then
  echo "❌ Please enter just the filename, not a full path like /path/to/.zshrc"
  exit 1
fi

SHELL_CONFIG="$HOME/$CONFIG_FILENAME"

if [ ! -f "$SHELL_CONFIG" ]; then
  echo "⚠️  $SHELL_CONFIG not found. Creating it..."
  touch "$SHELL_CONFIG"
fi

if ! grep -Fq "alias $ALIAS_NAME=" "$SHELL_CONFIG"; then
  echo "🔗 Adding alias to $SHELL_CONFIG"
  echo "alias $ALIAS_NAME=\"$BIN_PATH\"" >> "$SHELL_CONFIG"
else
  echo "✅ Alias already exists in $SHELL_CONFIG"
fi

clear

echo "🎉 Installed!"
echo "🔁 Restart your terminal or run: source $SHELL_CONFIG"
echo "🚀 Then use: $ALIAS_NAME"
echo "📖 Try: $ALIAS_NAME --help"
