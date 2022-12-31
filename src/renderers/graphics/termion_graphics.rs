extern crate termion;
use renderers::graphics::Graphics;
use std::io::{stdout, Stdout, Write};
use termion::raw::IntoRawMode;
use termion::{clear, cursor};

pub struct TermionGraphics {
    output_stream: termion::raw::RawTerminal<Stdout>,
}

impl TermionGraphics {
    pub fn new() -> Self {
        TermionGraphics {
            output_stream: stdout().into_raw_mode().unwrap(),
        }
    }
}

impl Graphics for TermionGraphics {
    fn initialize(&mut self) {
        // Clear and hide cursor
        write!(self.output_stream, "{}", clear::All).unwrap();
        write!(self.output_stream, "{}", cursor::Hide).unwrap();

        // Draw screen box
        write!(self.output_stream, "{}", clear::All).unwrap();
        // Top row
        write!(self.output_stream, "{}┌", cursor::Goto(1, 1)).unwrap();
        for i in 2..66 {
            write!(self.output_stream, "{}─", cursor::Goto(i, 1)).unwrap();
        }
        write!(self.output_stream, "{}┐", cursor::Goto(66, 1)).unwrap();

        // Vertical rows
        for i in 2..18 {
            write!(self.output_stream, "{}│", cursor::Goto(1, i)).unwrap();
            write!(self.output_stream, "{}│", cursor::Goto(66, i)).unwrap();
        }

        // Bottom row
        write!(self.output_stream, "{}└", cursor::Goto(1, 18)).unwrap();
        for i in 2..66 {
            write!(self.output_stream, "{}─", cursor::Goto(i, 18)).unwrap();
        }
        write!(self.output_stream, "{}┘", cursor::Goto(66, 18)).unwrap();
    }

    fn draw(&mut self, screen: [[u8; 8]; 32]) {
        const PADDING: u16 = 2;
        for y in (0..16).map(|x| x * 2) {
            for x in 0..8 {
                for i in 0..8 {
                    let top_square: bool = (screen[y][x as usize] << i) & 0x80 == 0x80;
                    let bottom_square: bool = (screen[y + 1_usize][x as usize] << i) & 0x80 == 0x80;
                    let x_coord = (x * 8) + i + PADDING;
                    let y_coord: u16 = (y / 2) as u16 + PADDING;
                    match (top_square, bottom_square) {
                        (true, true) => {
                            write!(self.output_stream, "{}█", cursor::Goto(x_coord, y_coord))
                                .unwrap()
                        }
                        (true, false) => {
                            write!(self.output_stream, "{}▀", cursor::Goto(x_coord, y_coord))
                                .unwrap()
                        }
                        (false, true) => {
                            write!(self.output_stream, "{}▄", cursor::Goto(x_coord, y_coord))
                                .unwrap()
                        }
                        (false, false) => {
                            write!(self.output_stream, "{} ", cursor::Goto(x_coord, y_coord))
                                .unwrap()
                        }
                    }
                }
            }
        }
        self.output_stream.flush().unwrap();
    }
}
