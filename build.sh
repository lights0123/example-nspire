#!/usr/bin/env sh
set -e
NAME=$(cargo read-manifest | jq -r '.name')
TARGET=armv5te-nspire-eabi
cargo xbuild --target "$TARGET.json" 2>&1
genzehn --input "target/$TARGET/debug/$NAME" --output "target/$TARGET/debug/$NAME.zehn" --name "hello_world"
make-prg "target/$TARGET/debug/$NAME.zehn" hello_world.tns
