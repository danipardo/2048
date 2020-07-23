use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::render::TextureCreator;
use sdl2::video::Window;
use sdl2::video::WindowContext;
use std::path::Path;
use std::rc::Rc;

use sdl2::rect::Rect;
use sdl2::render::Texture;

use sdl2::pixels::Color;
use sdl2::rect::Point;

use crate::resource_manager::{ResourceManager, FontDetails};
use crate::board::Board;
use sdl2::ttf::{Sdl2TtfContext, Font};
use std::collections::HashMap;
use std::borrow::Borrow;

type TextureManager<'l, T> = ResourceManager<'l, String, Texture<'l>, TextureCreator<T>>;
type FontManager<'l> = ResourceManager<'l, FontDetails, Font<'l, 'static>, Sdl2TtfContext>;

const CELL_SIZE: u16 = 65;
const DEFAULT_BROWN: Color = Color::RGB(119,110,101);
const DEFAULT_WHITE: Color = Color::RGB(249,246,242);
fn color_from_number(n: u8) -> (Color, Color) {
    match (n) {
        2 => return (Color::RGB(238, 228, 218), DEFAULT_BROWN),
        _ => return (Color::RGB(205, 193, 180), DEFAULT_WHITE)
    }
}

pub struct Game<'a> {
    event_pump: Box<sdl2::EventPump>,
    texture_manager: TextureManager<'a, WindowContext>,
    font_manager: FontManager<'a>,
}

impl<'a> Game<'a> {
    pub fn draw(&mut self, canvas: &mut Canvas<Window>, board: &Board) {
        canvas.set_draw_color(Color::RGB(187, 173, 160));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        let font = self.font_manager.load(
            &FontDetails {
                path: "res/fonts/Lato-Regular.ttf".to_owned(),
                size: 35,
            }).unwrap();

        for x in 0..4 {
            for y in 0..4 {
                let cell = board.get_cell(x, y);
                let colors = color_from_number(cell);

                // Draws the cell background
                canvas.set_draw_color(colors.0);
                canvas.fill_rect(Rect::from_center(Point::new(
                    300 + x as i32 * (CELL_SIZE + 10) as i32,
                    200 + y as i32 * (CELL_SIZE + 10) as i32,
                ), CELL_SIZE as u32, CELL_SIZE as u32)).unwrap();

                // Get a surface for the number
                let surface = font.render(cell.to_string().borrow()).blended(colors.1).unwrap();

                let texture_creator = canvas.texture_creator();
                let texture = texture_creator.
                    create_texture_from_surface(&surface).unwrap();

                let source_rect = Rect::new(0, 0, surface.size().0, surface.size().1);

                let dest_rect = Rect::from_center(Point::new(
                    300 + x as i32 * (CELL_SIZE + 10) as i32,
                    200 + y as i32 * (CELL_SIZE + 10) as i32,
                ),
                                                  surface.size().0,
                                                  surface.size().1);


                canvas.copy(&texture, source_rect, dest_rect).unwrap();
            }
        }

        canvas.present();
    }


    pub fn new(event_pump: Box<sdl2::EventPump>,
               texture_creator: &'a mut TextureCreator<WindowContext>,
               font_context: &'a mut Sdl2TtfContext,
    ) -> Self {
        let tm = TextureManager::new(texture_creator);
        let fm = FontManager::new(font_context);

        Game {
            event_pump,
            texture_manager: tm,
            font_manager: fm,

        }
    }

    pub fn update(&mut self, delta: u32) {

        // self.player.update();
    }

    pub fn finished(&mut self) -> bool {
        self.handle_events()
    }
    fn handle_events(&mut self) -> bool {
        let mut r: bool = false;

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => r = true,
                _ => {}
            }
        }

        r
    }
}


