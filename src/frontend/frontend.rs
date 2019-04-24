pub enum KeyboardCommand {
    KeypadState(u16),
    Quit,
}

pub trait Frontend {
    fn initialize(&mut self);
    fn draw(&mut self, screen: [[u8; 8]; 32]);
    fn wait_for_key(&mut self) -> u8;
    // We use the first 16 bits for keypad keys, the other ones
    // are for special commands
    fn get_keyboard_state(&mut self) -> KeyboardCommand;
}

// Special keyboard numbers
// 0x10000 = Quit