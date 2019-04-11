pub trait Frontend {
    fn initialize(&mut self);
    fn draw(&mut self, screen: [[u8; 8]; 32]);
    fn wait_for_key(&mut self) -> u8;
    fn get_keyboard_state(&mut self) -> u16;
}