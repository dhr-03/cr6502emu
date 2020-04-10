#!/bin/bash
printf "\033c"
wasm-pack build --target web --no-typescript --dev --out-name $(grep -oP "(?<=name = \")\w+(?=\")" Cargo.toml)_dev
