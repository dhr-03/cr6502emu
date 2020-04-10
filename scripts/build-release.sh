#!/bin/bash
printf "\033c"
wasm-pack build --target web --no-typescript --release
