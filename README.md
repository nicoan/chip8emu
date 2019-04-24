# Chip 8 Emulator written in rust

![Preview](https://raw.githubusercontent.com/nicoan/chip8emu/master/preview.gif)

A short project to understand how emulators work and an oportunity to learn and test Rust lang

## Build

`cargo build`

## Run

`cargo run <path_to_game>`

*NOTE: You need to be root to run termion front end, because we need to detect KEY_UP and KEY_DOWN events in a terminal environment*

## Todo

- Add help argument
- Add SDL Frontend
- Add keyboard layout to readme
