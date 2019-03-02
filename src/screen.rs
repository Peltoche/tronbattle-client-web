
use stdweb::console;
use crate::map::TILE_SIZE;

#[derive(Default)]
pub struct Screen {
    pixel_width:  u32,
    pixel_height:  u32,

    display_tile_x_limits: (i64, i64),
    display_tile_y_limits: (i64, i64),

    position: (u32, u32),
}

impl Screen {
    pub fn set_width(&mut self, width: u32) {
        self.pixel_width = width;

        self.calculate_display_tiles()
    }

    pub fn set_height(&mut self, height: u32) {
        self.pixel_height = height;

        self.calculate_display_tiles()
    }

    pub fn position(&mut self) -> (u32, u32) {
        (self.position.0, self.position.1)
    }

    pub fn set_position(&mut self, x: u32, y: u32) {
        self.position.0 = x;
        self.position.1 = y;

        self.calculate_display_tiles()
    }

    pub fn pixel_size(&self) -> (f64, f64) {
        (self.pixel_width as f64, self.pixel_height as f64)
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
