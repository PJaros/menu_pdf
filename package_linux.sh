#!/bin/bash

# Package script for Linux

# uncomment to debug
#set -x

DIR="$(dirname "$0")"
LINUX_TARGET="$DIR/target/release"
DEPLOY="$LINUX_TARGET/package"

mkdir -p "$DEPLOY"
cp "$LINUX_TARGET/menu_pdf" "$DEPLOY"
cp -r "$DIR/res" "$DIR/test_res" "$DEPLOY"

# Further ideas: https://stackoverflow.com/a/31081673/406423
