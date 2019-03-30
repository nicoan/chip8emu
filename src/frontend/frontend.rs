pub trait Frontend {
    fn initialize(&mut self);
    fn draw(&mut self, screen: [[u8; 8]; 32]);
    fn check_pressed_keys(&mut self) -> u16;
}