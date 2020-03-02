pub mod termion_graphics;
pub mod sdl_graphics;

pub trait Graphics {
    fn initialize(&mut self);
    fn draw(&mut self, screen: [[u8; 8]; 32]);
}
