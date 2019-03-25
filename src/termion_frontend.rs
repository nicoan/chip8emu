extern crate termion;
use frontend::Frontend;
use termion::{cursor, clear};
use termion::raw::IntoRawMode;
use std::io::{Read, Write, stdout, Stdout, Bytes};
use termion::async_stdin;

pub struct TermionFrontend {
    output_stream: termion::raw::RawTerminal<Stdout>,
    input_stream: Bytes<termion::AsyncReader>,
}

impl TermionFrontend {
    pub fn new() -> Self {
        TermionFrontend {
            output_stream: stdout().into_raw_mode().unwrap(),
            input_stream: async_stdin().bytes()
        }
    }
}

impl Frontend for TermionFrontend {
    fn initialize(&mut self) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        // Clear and hide cursor
        write!(stdout, "{}", clear::All).unwrap();
        write!(stdout, "{}", cursor::Hide).unwrap();

        // Draw screen box
        write!(stdout, "{}", clear::All).unwrap();
        // Top row
        write!(stdout, "{}┌", cursor::Goto(1, 1)).unwrap();
        for i in 2..66 {
            write!(stdout, "{}─", cursor::Goto(i, 1)).unwrap();
        }
        write!(stdout, "{}┐", cursor::Goto(66, 1)).unwrap();

        // Vertical rows
        for i in 2..18 {
            write!(stdout, "{}│", cursor::Goto(1, i)).unwrap();
            write!(stdout, "{}│", cursor::Goto(66, i)).unwrap();
        }

        // Bottom row
        write!(stdout, "{}└", cursor::Goto(1, 18)).unwrap();
        for i in 2..66 {
            write!(stdout, "{}─", cursor::Goto(i, 18)).unwrap();
        }
        write!(stdout, "{}┘", cursor::Goto(66, 18)).unwrap();

        stdout.flush().unwrap();
    }

    fn draw(&mut self, screen: [[u8; 8]; 32]) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        const PADDING: u16 = 2;
        for y in (0..16).map(|x| x * 2) {
            for x in 0..8 {
                for i in 0..8 {
                    let top_square: bool = (screen[y as usize][x as usize] << i) & 0x80 == 0x80;
                    let bottom_square: bool = (screen[y + 1 as usize][x as usize] << i) & 0x80 == 0x80;
                    let x_coord = (x * 8) + i + PADDING;
                    let y_coord: u16 = (y / 2) as u16 + PADDING;
                    match (top_square, bottom_square) {
                        (true, true) => write!(stdout, "{}█", cursor::Goto(x_coord, y_coord)).unwrap(),
                        (true, false) => write!(stdout, "{}▀", cursor::Goto(x_coord, y_coord)).unwrap(),
                        (false, true) => write!(stdout, "{}▄", cursor::Goto(x_coord, y_coord)).unwrap(),
                        (false, false) => write!(stdout, "{} ", cursor::Goto(x_coord, y_coord)).unwrap()
                    }
                }
            }
        }
        stdout.flush().unwrap();
    }
}