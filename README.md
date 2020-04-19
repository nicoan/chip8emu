# Chip 8 Emulator

![Preview](https://raw.githubusercontent.com/nicoan/chip8emu/master/preview.gif)

The objective of this project is to get some understanding in how emulators work and use it as an opportunity to learn and play a little with Rust lang. 

## Build

### Dependencies
- `libsdl2-gfx-dev`

### Building
Debug: `cargo build`

Release: `cargo build --release`

## Run

```
cargo run -- [OPTIONS] --game <FILE> 
```

#### Options
 * `-g, --game <FILE>`: Path to the game
 * `-r, --renderer <terminal | sdl>`: Render method to use. Default is SDL.
 * `-h, --help`: Prints help information
 * `-V, --version`: Prints version information

*NOTE: You need to be root in order to run the terminal front end, root privileges are needed to detect KEY_UP and KEY_DOWN events in a terminal environment*

## Play

The emulator has the following keymappings

```
 Chip8 Keypad                PC Keyboard
   +-+-+-+-+                  +-+-+-+-+
   |1|2|3|C|                  |1|2|3|4|
   +-+-+-+-+                  +-+-+-+-+
   |4|5|6|D|                  |Q|W|E|R|
   +-+-+-+-+       =>         +-+-+-+-+
   |7|8|9|E|                  |A|S|D|F|
   +-+-+-+-+                  +-+-+-+-+
   |A|0|B|F|                  |Z|X|C|V|
   +-+-+-+-+                  +-+-+-+-+
```

To exit emulator, press 'o' key.

## Resources
- [Cowgod's Chip-8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
- [How to write an emulator (CHIP-8 interpreter](http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/)
- [Mastering Chip-8](http://mattmik.com/files/chip8/mastering/chip8.html)