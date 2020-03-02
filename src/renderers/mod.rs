pub mod graphics;
pub mod input;

use renderers::graphics::Graphics;
use renderers::graphics::termion_graphics::TermionGraphics;
use renderers::graphics::sdl_graphics::SdlGraphics;

use renderers::input::{Input};
use renderers::input::sdl_input::SdlInput;
use renderers::input::termion_input::TermionInput;

pub struct Renderer {
    pub graphics: Box<dyn Graphics>,
    pub input: Box<dyn Input>,
}

pub fn get_renders(renderer: String) -> Renderer {
    if renderer == "terminal" {
        return Renderer {
            graphics: Box::new(TermionGraphics::new()),
            input: Box::new(TermionInput::new()),
        }
    }
    let sdl = sdl2::init().unwrap();

    return Renderer {
        graphics: Box::new(SdlGraphics::new(&sdl)),
        input: Box::new(SdlInput::new(&sdl)),
    }
}
