#!/bin/bash
clear ;ide
printf "\033c" ;terminal
wasm-pack build --target web --no-typescript --dev --out-name $(grep -oP "(?<=name = \")\w+(?=\")" Cargo.toml)_dev
