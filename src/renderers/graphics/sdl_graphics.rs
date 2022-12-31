extern crate sdl2;
use renderers::graphics::Graphics;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;

pub struct SdlGraphics {
    canvas: Canvas<Window>,
}

impl SdlGraphics {
    pub fn new(sdl: &Sdl) -> Self {
        let video_subsystem = sdl.video().unwrap();
        let window = video_subsystem
            .window("Chip 8 Emulator", 640, 320)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_scale(10.0, 10.0).unwrap();

        SdlGraphics { canvas }
    }

    fn clear_screen(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
    }
}

impl Graphics for SdlGraphics {
    fn initialize(&mut self) {
        self.clear_screen();
        self.canvas.present();
    }

    fn draw(&mut self, screen: [[u8; 8]; 32]) {
        // Clear screen
        self.clear_screen();

        // Sets white colro for drawing the pixels.
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));

        // Draw the screen
        for y in 0..32 {
            for x in 0..8 {
                for i in 0..8 {
                    if ((screen[y as usize][x as usize] << i) & 0x80) == 0x80 {
                        let x_coord = (x * 8) + i;
                        self.canvas.draw_point(Point::new(x_coord, y)).unwrap();
                    }
                }
            }
        }

        // Present the changes
        self.canvas.present();
    }
}
