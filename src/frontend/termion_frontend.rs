extern crate termion;
use frontend::frontend::Frontend;
use termion::{cursor, clear};
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, Stdout};
use std::{thread};
use linux_raw_input_rs::{InputReader, get_input_devices};
use linux_raw_input_rs::keys::Keys;
use linux_raw_input_rs::input::EventType;
use std::sync::{Arc, Mutex};

pub struct TermionFrontend {
    output_stream: termion::raw::RawTerminal<Stdout>,
    keyboard_state: Arc<Mutex<u16>>,
}

impl TermionFrontend {
    pub fn new() -> Self {
        TermionFrontend {
            output_stream: stdout().into_raw_mode().unwrap(),
            keyboard_state: Arc::new(Mutex::new(0x0)),
        }
    }
}

fn check_pressed_keys(keyboard_state: Arc<Mutex<u16>>) {
    let device_path : String = get_input_devices().iter().nth(0).expect("Problem with iterator").to_string();
    let mut input_stream = InputReader::new(device_path);
    loop {
        let input = input_stream.current_state();
        let mut kb_state = keyboard_state.lock().unwrap();
        if input.is_key_event(){
            match input.event_type() {
                EventType::Push => {
                    match input.get_key() {
                        Keys::KEY_1 => { *kb_state |= 0x1 },
                        Keys::KEY_2 => { *kb_state |= 0x2; },
                        Keys::KEY_3 => { *kb_state |= 0x4; },
                        Keys::KEY_4 => { *kb_state |= 0x8; },
                        Keys::KEY_Q => { *kb_state |= 0x10; },
                        Keys::KEY_W => { *kb_state |= 0x20; },
                        Keys::KEY_E => { *kb_state |= 0x40; },
                        Keys::KEY_R => { *kb_state |= 0x80; },
                        Keys::KEY_A => { *kb_state |= 0x100; },
                        Keys::KEY_S => { *kb_state |= 0x200; },
                        Keys::KEY_D => { *kb_state |= 0x400; },
                        Keys::KEY_F => { *kb_state |= 0x800; },
                        Keys::KEY_Z => { *kb_state |= 0x1000; },
                        Keys::KEY_X => { *kb_state |= 0x2000; },
                        Keys::KEY_V => { *kb_state |= 0x4000; },
                        Keys::KEY_C => { *kb_state |= 0x8000; },
                        _ => {},
                    }
                },
                EventType::Release => {
                    match input.get_key() {
                        Keys::KEY_1 => { *kb_state &= !0x1 },
                        Keys::KEY_2 => { *kb_state &= !0x2; },
                        Keys::KEY_3 => { *kb_state &= !0x4; },
                        Keys::KEY_4 => { *kb_state &= !0x8; },
                        Keys::KEY_Q => { *kb_state &= !0x10; },
                        Keys::KEY_W => { *kb_state &= !0x20; },
                        Keys::KEY_E => { *kb_state &= !0x40; },
                        Keys::KEY_R => { *kb_state &= !0x80; },
                        Keys::KEY_A => { *kb_state &= !0x100; },
                        Keys::KEY_S => { *kb_state &= !0x200; },
                        Keys::KEY_D => { *kb_state &= !0x400; },
                        Keys::KEY_F => { *kb_state &= !0x800; },
                        Keys::KEY_Z => { *kb_state &= !0x1000; },
                        Keys::KEY_X => { *kb_state &= !0x2000; },
                        Keys::KEY_V => { *kb_state &= !0x4000; },
                        Keys::KEY_C => { *kb_state &= !0x8000; },
                        _ => {},
                    }
                },
                _ => {}
            }
        }
        drop(kb_state);
    }
}

impl Frontend for TermionFrontend {
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

        let kb_state = self.keyboard_state.clone();
        thread::spawn(move || { check_pressed_keys(kb_state) });
    }

    fn draw(&mut self, screen: [[u8; 8]; 32]) {
        const PADDING: u16 = 2;
        for y in (0..16).map(|x| x * 2) {
            for x in 0..8 {
                for i in 0..8 {
                    let top_square: bool = (screen[y as usize][x as usize] << i) & 0x80 == 0x80;
                    let bottom_square: bool = (screen[y + 1 as usize][x as usize] << i) & 0x80 == 0x80;
                    let x_coord = (x * 8) + i + PADDING;
                    let y_coord: u16 = (y / 2) as u16 + PADDING;
                    match (top_square, bottom_square) {
                        (true, true) => write!(self.output_stream, "{}█", cursor::Goto(x_coord, y_coord)).unwrap(),
                        (true, false) => write!(self.output_stream, "{}▀", cursor::Goto(x_coord, y_coord)).unwrap(),
                        (false, true) => write!(self.output_stream, "{}▄", cursor::Goto(x_coord, y_coord)).unwrap(),
                        (false, false) => write!(self.output_stream, "{} ", cursor::Goto(x_coord, y_coord)).unwrap()
                    }
                }
            }
        }
        self.output_stream.flush().unwrap();
    }

    fn wait_for_key(&mut self) -> u8 {
        /*let stdin = stdin();
        println!("asdas");
        for c in stdin.keys() {
            println!("asdas");
            match c.unwrap() {
                Key::Char('1') => return 0,
                Key::Char('2') => return 1,
                Key::Char('3') => return 2,
                Key::Char('4') => return 3,
                Key::Char('q') => return 4,
                Key::Char('w') => return 5,
                Key::Char('e') => return 6,
                Key::Char('r') => return 7,
                Key::Char('a') => return 8,
                Key::Char('s') => return 9,
                Key::Char('d') => return 10,
                Key::Char('f') => return 11,
                Key::Char('z') => return 12,
                Key::Char('x') => return 13,
                Key::Char('v') => return 14,
                Key::Char('c') => return 15,
                _ => return self.wait_for_key()
            }
        }*/
        return 0x0;
    }

    fn get_keyboard_state(&mut self) -> u16 {
        let keyboard_state = self.keyboard_state.lock().unwrap();
        drop(*keyboard_state);
        return *keyboard_state;
    }
}