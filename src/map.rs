use crate::room::Room;
use std::cmp;
/// Will be our tile, this has clone and copy traits
#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub blocked: bool,
    pub opaque: bool,
}

impl Tile {
    /// generates an empty space
    pub fn empty() -> Self {
        Tile {
            blocked: false,
            opaque: false,
        }
    }
    /// generates a tile which emulates a wall
    pub fn wall() -> Self {
        Tile {
            blocked: true,
            opaque: true,
        }
    }
}

/// Represents a Map type which is essentially just a 2d vector of tiles
pub type Map = Vec<Vec<Tile>>;

pub struct Mapping {}

/// implementation of some associated methods that lets us make maps easily
impl Mapping {
    pub fn empty_walled(width: i32, height: i32) -> Map {
        let map = vec![vec![Tile::wall(); height as usize]; width as usize];

        map
    }
    pub fn create_room(room: &Room, map: &mut Map) {
        for x in (room.x1 + 1)..room.x2 {
            for y in (room.y1 + 1)..room.y2 {
                map[x as usize][y as usize] = Tile::empty();
            }
        }
    }
    pub fn create_tunnel_horizontal(x1: i32, x2: i32, y: i32, map: &mut Map) {
        for x in cmp::min(x1, x2)..(cmp::max(x1, x2) + 1) {
            map[x as usize][y as usize] = Tile::empty();
        }
    }

    pub fn crete_tunnel_vertical(y1: i32, y2: i32, x: i32, map: &mut Map) {
        for y in cmp::min(y1, y2)..(cmp::max(y1, y2) + 1) {
            map[x as usize][y as usize] = Tile::empty();
        }
    }
}
