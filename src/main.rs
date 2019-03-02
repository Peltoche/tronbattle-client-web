#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate lazy_static;

mod map;
mod screen;
mod socket;
mod drawer;
mod keyboard;

use stdweb::console;
use stdweb::web::{document, window};
use stdweb::web::html_element::CanvasElement;
use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::WebSocket;

use std::sync::Mutex;

use screen::Screen;
use map::{Map, MAP_SIZE, Element};
use drawer::Drawer;

const CANVAS_SELECTOR: &'static str = "#map";
const CONNECTION: &'static str = "ws://127.0.0.1:8080/socket";


lazy_static! {
    static ref CANVAS: CanvasElement = document()
        .query_selector(CANVAS_SELECTOR).expect("failed to select the canvas")
        .expect("canvas not found")
        .try_into()
        .unwrap();

    static ref SCREEN: Mutex<Screen> = Mutex::new(Screen::init(&CANVAS).expect("failed to load the screen"));
    static ref DRAWER: Drawer = Drawer::new(&CANVAS);
    static ref MAP: Mutex<Map> = Mutex::new(Box::new([[Element::Empty; MAP_SIZE]; MAP_SIZE]));
    static ref SOCKET: Mutex<WebSocket> = Mutex::new(WebSocket::new(CONNECTION).expect("failed to create the socket"));
}

fn main() {
    console!(log, "start");
    stdweb::initialize();

    map::setup();

    screen::register_event_listeners();
    socket::register_event_listeners();
    keyboard::register_event_listeners();

    //draw_loop();
    window().request_animation_frame(|_| draw_loop());

    stdweb::event_loop();
    console!(log, "end");
}

fn draw_loop() {
    {
        let map = MAP.lock().expect("failed to lock the map");
        let screen = SCREEN.lock().expect("failed to lock the screen for drawing");

        DRAWER.draw_map_on_screen(&map, &screen);
    }

    //queue another animate() on the next frame
    window().request_animation_frame(|_| draw_loop());
}

