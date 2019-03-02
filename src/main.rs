#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate lazy_static;

mod map;
mod screen;
mod socket;

use stdweb::console;


fn main() {
    console!(log, "start");
    stdweb::initialize();

    map::init();
    map::draw_loop();

    socket::init();


    stdweb::event_loop();
    console!(log, "end");
}
