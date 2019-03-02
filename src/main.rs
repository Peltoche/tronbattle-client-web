#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate lazy_static;

mod map;
mod screen;

use stdweb::console;

fn main() {
    stdweb::initialize();

    map::init();
    map::draw();

    console!(log, "start");
    stdweb::event_loop();
    console!(log, "end");
}
