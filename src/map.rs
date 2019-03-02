
use crate::MAP;

pub const MAP_SIZE: usize = 100;


#[derive(Copy, Clone, Debug)]
pub enum Element {
    Empty,
    Wall,
    Player,
}

// The map need to be written into the heap due to the stack size limitation
// put by the browsers.
pub type Map = Box<[[Element; MAP_SIZE]; MAP_SIZE]>;

pub fn setup()  {
    let mut map = MAP.lock().expect("failed to lock the map");

    for x in 0..MAP_SIZE {
        // Put a wall on the top and bottom sides of the map.
        map[0][x] = Element::Wall;
        map[MAP_SIZE-1][x] = Element::Wall;

        // Put a wall on the left and right sides of the map.
        map[x][0] = Element::Wall;
        map[x][MAP_SIZE-1] = Element::Wall;
    }

    map[10][10] = Element::Player;
}
