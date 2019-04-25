# Chip 8 Emulator written in rust

![Preview](https://raw.githubusercontent.com/nicoan/chip8emu/master/preview.gif)

The objective of this project is to get some understanding in how emulators work and an use it as an oportunity to learn and play with Rust lang 

## Build

`cargo build`

## Run

`cargo run <path_to_game>`

*NOTE: You need to be root in order to run termion front end, root privileges are needed to detect KEY_UP and KEY_DOWN events in a terminal environment*

## Todo

- Add help argument
- Add SDL Frontend
- Add keyboard layout to readme
