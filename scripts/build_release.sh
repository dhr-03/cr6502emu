#!/bin/bash
clear #ide
printf "\033c" #terminal

export build_timestamp=$(date)

wasm-pack build --target web --no-typescript --release
