pub enum KeyboardCommand {
    KeypadState(u16),
    SingleKey(u8),
    Quit,
}

pub trait Input {
    fn initialize(&mut self);
    
    fn set_waiting_key(&mut self);

    fn get_keyboard_state(&mut self) -> KeyboardCommand;
}

pub mod sdl_input;

pub mod termion_input;
