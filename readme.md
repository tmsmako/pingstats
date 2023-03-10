[![Rust](https://github.com/tmsmako/pingstats/actions/workflows/rust.yml/badge.svg)](https://github.com/tmsmako/pingstats/actions/workflows/rust.yml)

## Description
a simple console app written in rust that creates a histogram of RTT values (round-trip time) from linux "ping" command output and renders to an SVG image

## Notes
- build with `cargo run --release` to take advantage of optimizations (especially when processing large files) 
