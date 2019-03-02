
use stdweb::console;
use stdweb::traits::*;
use stdweb::web::error::*;
use stdweb::web::event::ResizeEvent;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::window;

use crate::drawer::TILE_SIZE;
use crate::SCREEN;


pub struct Screen {
    canvas: &'static CanvasElement,

    pixel_width:  u32,
    pixel_height:  u32,

    display_tile_x_limits: (i64, i64),
    display_tile_y_limits: (i64, i64),

    position: (u32, u32),
}

pub fn register_event_listeners() {
    // Redifine the canvas size in case of resize event.
    window().add_event_listener(move |_: ResizeEvent| {
        let mut screen = SCREEN.lock().expect("failed to lock the screen");

        let screen_width = screen.canvas.offset_width() as u32;
        let screen_height = screen.canvas.offset_height() as u32;

        screen.set_width(screen_width);
        screen.set_height(screen_height);
    });
}

impl Screen {
    pub fn init(canvas: &'static CanvasElement) -> Result<Self, Error> {
        let width = canvas.offset_width() as u32;
        let height = canvas.offset_height() as u32;

        canvas.set_width(width);
        canvas.set_height(height);

        let mut screen = Self{
            canvas: canvas,
            pixel_width: width,
            pixel_height: height,

            // Set some default values.
            display_tile_x_limits: (0, 0),
            display_tile_y_limits: (0, 0),
            position: (0, 0),
        };

        // Override the default values with the valid ones.
        screen.calculate_display_tiles();

        Ok(screen)
    }


    pub fn position(&mut self) -> (u32, u32) {
        (self.position.0, self.position.1)
    }

    pub fn set_position(&mut self, x: u32, y: u32) {
        self.position.0 = x;
        self.position.1 = y;

        self.calculate_display_tiles()
    }

    pub fn is_col_in_screen(&self, x: usize) -> bool {
        (x as i64) >self.display_tile_x_limits.0 && (x as i64) < self.display_tile_x_limits.1
    }

    pub fn is_tile_in_screen(&self, x: usize, y: usize) -> bool {
        (x as i64) > self.display_tile_x_limits.0
            && (x as i64) < self.display_tile_x_limits.1
            && (y as i64) > self.display_tile_y_limits.0
            && (y as i64) < self.display_tile_y_limits.1
    }

    pub fn convert_to_screen_position(&self, x: usize, y: usize) -> (usize, usize) {
        (
            ((x as i64) - self.display_tile_x_limits.0) as usize,
            ((y  as i64)- self.display_tile_y_limits.0) as usize,
            )
    }

    pub fn size(&self) -> (f64, f64) {
        (self.pixel_width as f64, self.pixel_height as f64)
    }

    fn set_width(&mut self, width: u32) {
        self.canvas.set_width(width);

        self.pixel_width = width;

        self.calculate_display_tiles()
    }

    fn set_height(&mut self, height: u32) {
        self.canvas.set_height(height);

        self.pixel_height = height;

        self.calculate_display_tiles()
    }

    fn calculate_display_tiles(&mut self) {
        let screen_tile_width = self.pixel_width / TILE_SIZE as u32;
        let screen_tile_height = self.pixel_height / TILE_SIZE as u32;


        self.display_tile_x_limits = (
            self.position.0 as i64 - (screen_tile_width / 2) as i64,
            self.position.0 as i64 + (screen_tile_width / 2) as i64,
            );

        self.display_tile_y_limits = (
            self.position.1 as i64 - (screen_tile_height / 2) as i64,
            self.position.1 as i64 + (screen_tile_height / 2) as i64,
            );
    }

}
