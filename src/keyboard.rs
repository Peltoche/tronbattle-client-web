
use stdweb::traits::*;
use stdweb::web::window;
use stdweb::web::event::KeyDownEvent;

use crate::SCREEN;

const KEYLEFT: &'static str= "ArrowLeft";
const KEYRIGHT: &'static str= "ArrowRight";
const KEYUP: &'static str= "ArrowUp";
const KEYDOWN: &'static str= "ArrowDown";

pub fn register_event_listeners() {
    window().add_event_listener( move |e: KeyDownEvent| {
        let mut screen = SCREEN.lock().expect("failed to lock the screen for the keyboard");

        let mut pos = screen.position();

        match e.code().as_str() {
            KEYUP => if pos.1 > 1 {pos.1 -= 1},
            KEYDOWN => pos.1 += 1,
            KEYLEFT => if pos.0 > 1 {pos.0 -= 1},
            KEYRIGHT => pos.0 += 1,
            _ => return,
        }

        screen.set_position(pos.0, pos.1);
    });
}
