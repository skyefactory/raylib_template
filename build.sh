#!/bin/sh
echo "Building..."
cd ../raylib_game
cargo build --target x86_64-pc-windows-gnu
NAME=$(date +%R)
if [ -f ./build/$NAME.exe ]; then
    rm ./build/$NAME.exe
fi
mv ./target/x86_64-pc-windows-gnu/debug/raylib_game.exe ./build/$NAME.exe
