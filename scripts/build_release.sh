#!/bin/bash
clear #ide
printf "\033c" #terminal
wasm-pack build --target web --no-typescript --release
