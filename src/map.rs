
use stdweb::console;
use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::error::*;
use stdweb::web::event::ResizeEvent;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, window, CanvasRenderingContext2d};

use std::sync::Mutex;
use crate::screen::Screen;

const MAP_SIZE: usize = 1000;
pub const TILE_SIZE: f64 = 13.0;

lazy_static! {
    static ref MAP: Mutex<Map> = Mutex::new(Map::load("#map").expect("failed to load the map"));
}

#[derive(Copy, Clone, Debug)]
pub enum Element {
    Empty,
    Wall,
    Player,
}

pub struct Map {
    // The map need to be written into the heap due to the stack size limitation
    // put by the browsers.
    map: Box<[[Element; MAP_SIZE]; MAP_SIZE]>,
    screen: Screen,

    canvas: CanvasElement,
    context: CanvasRenderingContext2d,
}

impl Map {
    pub fn load(selector: &str) -> Result<Self, Error> {

        let canvas: CanvasElement = document()
            .query_selector(selector)?
            .ok_or_else(|| Error::new(&format!("selector \"{}\" not found", selector)))?
            .try_into()
            .unwrap();



        window().add_event_listener(|_: ResizeEvent| {
            {
                // The lock needs to be released before calling draw();
                let mut map = MAP.lock().expect("failed to lock the map");

                let width = map.canvas.offset_width() as u32;
                let height = map.canvas.offset_height() as u32;

                map.canvas.set_width(width);
                map.canvas.set_height(height);

                map.screen.set_width(width);
                map.screen.set_height(height);
            }

            draw();
        });

        let width = canvas.offset_width() as u32;
        let height = canvas.offset_height() as u32;

        canvas.set_width(width);
        canvas.set_height(height);

        let mut screen = Screen::default();
        screen.set_width(width);
        screen.set_height(height);

        let res = Self {
            map: Box::new([[Element::Empty; MAP_SIZE]; MAP_SIZE]),
            screen: screen,

            context: canvas.get_context().unwrap(),
            canvas: canvas,
        };

        Ok(res)
    }
}

pub fn init() {
    let mut map = MAP.lock().expect("failed to lock the map");

    for x in 0..MAP_SIZE {
        // Put a wall on the top and bottom sides of the map.
        map.map[0][x] = Element::Wall;
        map.map[MAP_SIZE-1][x] = Element::Wall;

        // Put a wall on the left and right sides of the map.
        map.map[x][0] = Element::Wall;
        map.map[x][MAP_SIZE-1] = Element::Wall;
    }

    map.map[10][10] = Element::Player;
}

pub fn draw() {
    let map = MAP.lock().expect("failed to lock the map");

    clear(&map);

    for (x, col) in map.map.iter().enumerate() {
        if !map.screen.is_col_in_screen(x) {
            break
        }

        for (y, tile_content) in col.iter().enumerate() {
            if !map.screen.is_tile_in_screen(x, y) {
                break
            }

            let screen_pos = map.screen.convert_to_screen_position(x, y);
            draw_tile(&map, screen_pos.0, screen_pos.1, tile_content)
        }
    }

    map.context.stroke();
}

pub fn clear(map: &Map) {
    map.context.set_fill_style_color("black");

    let screen_size = map.screen.pixel_size();

    map.context.fill_rect(0.0, 0.0, screen_size.0, screen_size.1);
}

pub fn draw_tile(map: &Map, x: usize, y: usize, elem: &Element) {
    match elem {
        Element::Wall => map.context.set_fill_style_color("red"),
        Element::Empty => map.context.set_fill_style_color("black"),
        Element::Player => map.context.set_fill_style_color("blue"),
    }

    let x_offset = x as f64 * TILE_SIZE;
    let y_offset = y as f64 * TILE_SIZE;

    map.context.fill_rect(x_offset, y_offset,  TILE_SIZE,  TILE_SIZE);
}
