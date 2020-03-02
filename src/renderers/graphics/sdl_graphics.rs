extern crate sdl2;
use sdl2::Sdl;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use renderers::graphics::Graphics;

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

        let mut canvas = window
            .into_canvas()
            .build()
            .unwrap();
        
        canvas.set_scale(10.0, 10.0)
            .unwrap();

        SdlGraphics {
            canvas: canvas,
        } 
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
                        let x_coord = (x * 8) + i as i32;
                        let y_coord = y as i32;
                        self.canvas.draw_point(Point::new(x_coord, y_coord)).unwrap();
                    }
                }
            }
        }

        // Present the changes
        self.canvas.present();
    }
}
