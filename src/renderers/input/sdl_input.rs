extern crate sdl2;
use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;
use renderers::input::{Input, KeyboardCommand};

pub struct SdlInput {
    waiting_key: bool,
    keyboard_state: u32,
    event_pump: EventPump,
}

impl SdlInput {
    pub fn new(sdl: &Sdl) -> Self {
        SdlInput {
            waiting_key: false,
            keyboard_state: 0x0,
            event_pump: sdl.event_pump().unwrap(),
        }
    }

    fn check_pressed_keys(&mut self) -> KeyboardCommand {
        let events: Vec<Event> = self.event_pump.poll_iter().collect();

        for event in events {
            match event {
                Event::KeyDown { keycode, .. } => {
                    match keycode {
                        Some(Keycode::Num1) => { self.keyboard_state |= 0x1; },
                        Some(Keycode::Num2) => { self.keyboard_state |= 0x2; },
                        Some(Keycode::Num3) => { self.keyboard_state |= 0x4; },
                        Some(Keycode::Num4) => { self.keyboard_state |= 0x8; },
                        Some(Keycode::Q) => { self.keyboard_state |= 0x10; },
                        Some(Keycode::W) => { self.keyboard_state |= 0x20; },
                        Some(Keycode::E) => { self.keyboard_state |= 0x40; },
                        Some(Keycode::R) => { self.keyboard_state |= 0x80; },
                        Some(Keycode::A) => { self.keyboard_state |= 0x100; },
                        Some(Keycode::S) => { self.keyboard_state |= 0x200; },
                        Some(Keycode::D) => { self.keyboard_state |= 0x400; },
                        Some(Keycode::F) => { self.keyboard_state |= 0x800; },
                        Some(Keycode::Z) => { self.keyboard_state |= 0x1000; },
                        Some(Keycode::X) => { self.keyboard_state |= 0x2000; },
                        Some(Keycode::V) => { self.keyboard_state |= 0x4000; },
                        Some(Keycode::C) => { self.keyboard_state |= 0x8000; },
                        Some(Keycode::O) => { return KeyboardCommand::Quit; },
                        _ => {},
                    }
                },
                Event::KeyUp { keycode, .. } => {
                    match keycode {
                        Some(Keycode::Num1) => { self.keyboard_state &= !0x1 },
                        Some(Keycode::Num2) => { self.keyboard_state &= !0x2; },
                        Some(Keycode::Num3) => { self.keyboard_state &= !0x4; },
                        Some(Keycode::Num4) => { self.keyboard_state &= !0x8; },
                        Some(Keycode::Q) => { self.keyboard_state &= !0x10; },
                        Some(Keycode::W) => { self.keyboard_state &= !0x20; },
                        Some(Keycode::E) => { self.keyboard_state &= !0x40; },
                        Some(Keycode::R) => { self.keyboard_state &= !0x80; },
                        Some(Keycode::A) => { self.keyboard_state &= !0x100; },
                        Some(Keycode::S) => { self.keyboard_state &= !0x200; },
                        Some(Keycode::D) => { self.keyboard_state &= !0x400; },
                        Some(Keycode::F) => { self.keyboard_state &= !0x800; },
                        Some(Keycode::Z) => { self.keyboard_state &= !0x1000; },
                        Some(Keycode::X) => { self.keyboard_state &= !0x2000; },
                        Some(Keycode::V) => { self.keyboard_state &= !0x4000; },
                        Some(Keycode::C) => { self.keyboard_state &= !0x8000; },
                        _ => {},
                    }
                },
                _ => {}
            }
        }

        return KeyboardCommand::KeypadState(self.keyboard_state as u16);
    }

    fn get_single_key(&mut self) -> KeyboardCommand {
        let events: Vec<Event> = self.event_pump.poll_iter().collect();
        let mut result = KeyboardCommand::KeypadState(0x0);

        for event in events {
            match event {
                Event::KeyDown { keycode, .. } => {
                    match keycode {
                        Some(Keycode::Num1) => { result = KeyboardCommand::SingleKey(1); },
                        Some(Keycode::Num2) => { result = KeyboardCommand::SingleKey(2); },
                        Some(Keycode::Num3) => { result = KeyboardCommand::SingleKey(3); },
                        Some(Keycode::Num4) => { result = KeyboardCommand::SingleKey(4); },
                        Some(Keycode::Q) => { result = KeyboardCommand::SingleKey(5); },
                        Some(Keycode::W) => { result = KeyboardCommand::SingleKey(6); },
                        Some(Keycode::E) => { result = KeyboardCommand::SingleKey(7); },
                        Some(Keycode::R) => { result = KeyboardCommand::SingleKey(8); },
                        Some(Keycode::A) => { result = KeyboardCommand::SingleKey(9); },
                        Some(Keycode::S) => { result = KeyboardCommand::SingleKey(10); },
                        Some(Keycode::D) => { result = KeyboardCommand::SingleKey(11); },
                        Some(Keycode::F) => { result = KeyboardCommand::SingleKey(12); },
                        Some(Keycode::Z) => { result = KeyboardCommand::SingleKey(13); },
                        Some(Keycode::X) => { result = KeyboardCommand::SingleKey(14); },
                        Some(Keycode::V) => { result = KeyboardCommand::SingleKey(15); },
                        Some(Keycode::C) => { result = KeyboardCommand::SingleKey(16); },
                        Some(Keycode::O) => { result = KeyboardCommand::Quit; },
                        _ => {},
                    }
                },
                _ => {},
            }
        }

        self.waiting_key = false;
        return result;
    }
}

impl Input for SdlInput {
    fn initialize(&mut self) {}

    fn set_waiting_key(&mut self) {
        self.waiting_key = true;
    }

    fn get_keyboard_state(&mut self) -> KeyboardCommand {
        return if self.waiting_key { self.get_single_key() } else { self.check_pressed_keys() };
    }
}