
use stdweb::console;
use stdweb::web::{CanvasRenderingContext2d};
use stdweb::web::html_element::CanvasElement;

use crate::map::{Map, Element};
use crate::screen::Screen;

pub const TILE_SIZE: f64 = 13.0;

pub struct Drawer {
    context: CanvasRenderingContext2d,
}

impl Drawer {
    pub fn new(canvas: &'static CanvasElement) -> Self {
        Self{
            context: canvas.get_context().unwrap(),
        }
    }

    pub fn draw_map_on_screen(&self, map: &Map, screen: &Screen) {
        let screen_size = screen.size();
        self.context.fill_rect(0.0, 0.0, screen_size.0, screen_size.1);

        for (x, col) in map.iter().enumerate() {
            if !screen.is_col_in_screen(x) {
                continue
            }

            for (y, tile_content) in col.iter().enumerate() {
                if !screen.is_tile_in_screen(x, y) {
                    continue
                }

                let screen_pos = screen.convert_to_screen_position(x, y);
                self.draw_tile(screen_pos.0, screen_pos.1, tile_content)
            }
        }


        self.context.stroke();
    }

    fn draw_tile(&self,  x: usize, y: usize, elem: &Element) {
        match elem {
            Element::Empty => return,
            Element::Wall => self.context.set_fill_style_color("red"),
            Element::Player => self.context.set_fill_style_color("blue"),
        }

        let x_offset = x as f64 * TILE_SIZE;
        let y_offset = y as f64 * TILE_SIZE;

        self.context.fill_rect(x_offset, y_offset,  TILE_SIZE,  TILE_SIZE);
        self.context.set_fill_style_color("black")
    }
}

