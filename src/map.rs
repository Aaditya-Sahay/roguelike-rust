use crate::room::Room;
use crate::character::Character;
use std::cmp;
use rand::Rng;

use crate::constants::*;
/// Will be our tile, this has clone and copy traits
#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub blocked: bool,
    pub opaque: bool,
    pub explored: bool,
}

impl Tile {
    /// generates an empty space
    pub fn empty() -> Self {
        Tile {
            blocked: false,
            explored: false,
            opaque: false,
        }
    }
    /// generates a tile which emulates a wall
    pub fn wall() -> Self {
        Tile {
            blocked: true,
            explored: false,
            opaque: true,
        }
    }
}

/// Represents a Map type which is essentially just a 2d vector of tiles
pub type Map = Vec<Vec<Tile>>;

pub struct Mapping {}

/// implementation of some associated methods that lets us make maps easily
impl Mapping {
    pub fn generate_random_map(player: &mut Character) -> Map {
        let mut map = Mapping::empty_walled(MAP_WIDTH, MAP_HEIGHT);

        let mut rooms:Vec<Room> = vec![];

        let mut gen = rand::thread_rng();

        for _ in 0..MAX_ROOMS {
            // random width and height
            let w = gen.gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
            let h = gen.gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
            // random position without going out of the boundaries of the map
            let x = gen.gen_range(0, MAP_WIDTH - w);
            let y = gen.gen_range(0, MAP_HEIGHT - h);

            let new_room = Room::new(x, y, w, h);

            let failed = rooms.iter().any(|room| new_room.intersects_with(room));

            if !failed {
                Mapping::create_room(&new_room, &mut map);
            }

            let (new_x, new_y) = new_room.center();

            if rooms.is_empty() {

                player.x = new_x;
                player.y = new_y;
            }else {
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();
                if rand::random() {
                    Mapping::create_tunnel_horizontal(prev_x, new_x, prev_y, &mut map);
                    Mapping::create_tunnel_vertical(prev_y, new_y, new_x, &mut map);
                } else {
                    Mapping::create_tunnel_vertical(prev_y, new_y, prev_x, &mut map);
                    Mapping::create_tunnel_horizontal(prev_x, new_x, new_y, &mut map);
                }
            }

            rooms.push(new_room);


        }
        map
    }
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

    pub fn create_tunnel_vertical(y1: i32, y2: i32, x: i32, map: &mut Map) {
        for y in cmp::min(y1, y2)..(cmp::max(y1, y2) + 1) {
            map[x as usize][y as usize] = Tile::empty();
        }
    }
}
