use linux_raw_input_rs::input::{EventType, Input};
use linux_raw_input_rs::keys::Keys;
use linux_raw_input_rs::{get_input_devices, InputReader};
use renderers::input::{Input as KeyInput, KeyboardCommand};
use std::sync::{Arc, Mutex};
use std::thread;

fn set_keyboard_state(input: Input, keyboard_state: u32) -> u32 {
    let mut kb_state = keyboard_state;
    match input.event_type() {
        EventType::Push => match input.get_key() {
            Keys::KEY_1 => kb_state |= 0x1,
            Keys::KEY_2 => {
                kb_state |= 0x2;
            }
            Keys::KEY_3 => {
                kb_state |= 0x4;
            }
            Keys::KEY_4 => {
                kb_state |= 0x8;
            }
            Keys::KEY_Q => {
                kb_state |= 0x10;
            }
            Keys::KEY_W => {
                kb_state |= 0x20;
            }
            Keys::KEY_E => {
                kb_state |= 0x40;
            }
            Keys::KEY_R => {
                kb_state |= 0x80;
            }
            Keys::KEY_A => {
                kb_state |= 0x100;
            }
            Keys::KEY_S => {
                kb_state |= 0x200;
            }
            Keys::KEY_D => {
                kb_state |= 0x400;
            }
            Keys::KEY_F => {
                kb_state |= 0x800;
            }
            Keys::KEY_Z => {
                kb_state |= 0x1000;
            }
            Keys::KEY_X => {
                kb_state |= 0x2000;
            }
            Keys::KEY_V => {
                kb_state |= 0x4000;
            }
            Keys::KEY_C => {
                kb_state |= 0x8000;
            }
            Keys::KEY_O => {
                kb_state |= 0x10000;
            }
            _ => {}
        },
        EventType::Release => match input.get_key() {
            Keys::KEY_1 => kb_state &= !0x1,
            Keys::KEY_2 => {
                kb_state &= !0x2;
            }
            Keys::KEY_3 => {
                kb_state &= !0x4;
            }
            Keys::KEY_4 => {
                kb_state &= !0x8;
            }
            Keys::KEY_Q => {
                kb_state &= !0x10;
            }
            Keys::KEY_W => {
                kb_state &= !0x20;
            }
            Keys::KEY_E => {
                kb_state &= !0x40;
            }
            Keys::KEY_R => {
                kb_state &= !0x80;
            }
            Keys::KEY_A => {
                kb_state &= !0x100;
            }
            Keys::KEY_S => {
                kb_state &= !0x200;
            }
            Keys::KEY_D => {
                kb_state &= !0x400;
            }
            Keys::KEY_F => {
                kb_state &= !0x800;
            }
            Keys::KEY_Z => {
                kb_state &= !0x1000;
            }
            Keys::KEY_X => {
                kb_state &= !0x2000;
            }
            Keys::KEY_V => {
                kb_state &= !0x4000;
            }
            Keys::KEY_C => {
                kb_state &= !0x8000;
            }
            _ => {}
        },
        _ => {}
    }

    kb_state
}

fn check_pressed_keys(keyboard_state: Arc<Mutex<u32>>) {
    let device_path: String = get_input_devices()
        .get(0)
        .expect("There was an error initializing the keyboard.")
        .to_string();
    let mut input_stream = InputReader::new(device_path);

    loop {
        let input = input_stream.current_state();
        let mut kb_state = keyboard_state.lock().unwrap();
        if input.is_key_event() {
            *kb_state = set_keyboard_state(input, *kb_state);
        }
        drop(kb_state);
    }
}

fn wait_for_key() -> KeyboardCommand {
    let device_path: String = get_input_devices()
        .get(0)
        .expect("There was an error initializing the keyboard.")
        .to_string();
    let mut input_stream = InputReader::new(device_path);

    loop {
        let input = input_stream.current_state();
        if input.is_key_event() {
            match input.event_type() {
                EventType::Push => match input.get_key() {
                    Keys::KEY_1 => {
                        return KeyboardCommand::SingleKey(1);
                    }
                    Keys::KEY_2 => {
                        return KeyboardCommand::SingleKey(2);
                    }
                    Keys::KEY_3 => {
                        return KeyboardCommand::SingleKey(3);
                    }
                    Keys::KEY_4 => {
                        return KeyboardCommand::SingleKey(4);
                    }
                    Keys::KEY_Q => {
                        return KeyboardCommand::SingleKey(5);
                    }
                    Keys::KEY_W => {
                        return KeyboardCommand::SingleKey(6);
                    }
                    Keys::KEY_E => {
                        return KeyboardCommand::SingleKey(7);
                    }
                    Keys::KEY_R => {
                        return KeyboardCommand::SingleKey(8);
                    }
                    Keys::KEY_A => {
                        return KeyboardCommand::SingleKey(9);
                    }
                    Keys::KEY_S => {
                        return KeyboardCommand::SingleKey(10);
                    }
                    Keys::KEY_D => {
                        return KeyboardCommand::SingleKey(11);
                    }
                    Keys::KEY_F => {
                        return KeyboardCommand::SingleKey(12);
                    }
                    Keys::KEY_Z => {
                        return KeyboardCommand::SingleKey(13);
                    }
                    Keys::KEY_X => {
                        return KeyboardCommand::SingleKey(14);
                    }
                    Keys::KEY_V => {
                        return KeyboardCommand::SingleKey(15);
                    }
                    Keys::KEY_C => {
                        return KeyboardCommand::SingleKey(16);
                    }
                    Keys::KEY_O => {
                        return KeyboardCommand::Quit;
                    }
                    _ => return KeyboardCommand::KeypadState(0x0),
                },
                _ => return KeyboardCommand::KeypadState(0x0),
            }
        }
    }
}

pub struct TermionInput {
    keyboard_state: Arc<Mutex<u32>>,
    waiting_key: bool,
}

impl TermionInput {
    pub fn new() -> Self {
        TermionInput {
            keyboard_state: Arc::new(Mutex::new(0x0)),
            waiting_key: false,
        }
    }

    fn wait_for_key(&mut self) -> KeyboardCommand {
        let wait = thread::spawn(wait_for_key);
        return wait.join().unwrap();
    }
}

impl KeyInput for TermionInput {
    fn initialize(&mut self) {
        let kb_state = self.keyboard_state.clone();
        thread::spawn(move || check_pressed_keys(kb_state));
    }

    fn set_waiting_key(&mut self) {
        self.waiting_key = true;
    }

    fn get_keyboard_state(&mut self) -> KeyboardCommand {
        if self.waiting_key {
            self.wait_for_key()
        } else {
            let keyboard_state = self.keyboard_state.lock().unwrap();

            #[allow(clippy::comparison_chain)]
            if *keyboard_state < 0x10000 {
                KeyboardCommand::KeypadState(*keyboard_state as u16)
            } else if *keyboard_state == 0x10000 {
                KeyboardCommand::Quit
            } else {
                KeyboardCommand::KeypadState(0x0)
            }
        }
    }
}
