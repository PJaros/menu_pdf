#!/bin/bash

# Package script for x86_64 windows

# uncomment to debug
#set -x

DIR="$(dirname "$0")"
X86_TARGET="$DIR/target/x86_64-pc-windows-gnu"
DEPLOY="$X86_TARGET/release/package"

mkdir -p "$DEPLOY"
cp "$X86_TARGET/release/menu_pdf.exe" "$DEPLOY"
cp -r "$DIR/res" "$DIR/test_res" "$DEPLOY"

# Further ideas: https://stackoverflow.com/a/31081673/406423
