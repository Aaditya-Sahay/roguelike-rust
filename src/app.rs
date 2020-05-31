use crate::map::*;
use tcod::colors::*;
use tcod::console::*;
use tcod::input::*;
use rand::Rng;

use crate::character::Character;
use crate::room::Room;

/// height and width of the map
const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 45;
/// Room constants
const ROOM_MAX_SIZE: i32 = 10;
const ROOM_MIN_SIZE: i32 = 6;
const MAX_ROOMS: i32 = 30;

///Color for wall and ground
const WALL_COLOR: Color = Color { r: 0, g: 0, b: 100 };
const GROUND_COLOR: Color = Color {
    r: 50,
    g: 50,
    b: 100,
};

/// This struct houses our game runner
pub struct Tcod {
    pub root: Root,
    pub offscreen: Offscreen,
    pub map: Map,
    pub characters: Vec<Character>,
    pub player: Character,
    _width: i32,
    _height: i32,
}

impl Tcod {
    pub fn new(width: i32, height: i32, characters: Vec<Character>) -> Tcod {
        let root = Root::initializer()
            .font("assets/consolas10x10_gs_tc.png", FontLayout::Tcod)
            .font_type(FontType::Greyscale)
            .size(width, height)
            .title("Game")
            .init();
        let offscreen = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);

        
        let mut player = Character::new(25, 23, '@', WHITE);
        let mut map = Tcod::generate_map(&mut player);

        map[30][22] = Tile::wall();
        map[50][22] = Tile::wall();

        Tcod {
            root,
            offscreen,
            map,
            characters,
            player,
            _width: width,
            _height: height,
        }
    }

    pub fn game_loop(&mut self) {
        while !self.root.window_closed() {
            self.offscreen.clear();

            self.render();

            // combining offscreen with screen
            blit(
                &self.offscreen,
                (0, 0),
                (MAP_WIDTH, MAP_HEIGHT),
                &mut self.root,
                (0, 0),
                1.0,
                1.0,
            );

            self.root.flush();

            // exit out of the game if warranted
            let status_exit = self.handle_keys();
            if status_exit == true {
                break;
            }
        }
    }

    pub fn render(&mut self) {
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let wall = self.map[x as usize][y as usize].opaque;
                if wall {
                    self.offscreen
                        .set_char_background(x, y, WALL_COLOR, BackgroundFlag::Set);
                } else {
                    self.offscreen
                        .set_char_background(x, y, GROUND_COLOR, BackgroundFlag::Set);
                }
            }
        }

        self.player.draw(&mut self.offscreen);
        // drawing all characters
        for character in self.characters.iter() {
            character.draw(&mut self.offscreen)
        }
    }

    pub fn handle_keys(&mut self) -> bool {
        let key: Key = self.root.wait_for_keypress(true);
        match key.code {
            KeyCode::Enter => {
                if key.alt == true {
                    let fullscreen = self.root.is_fullscreen();
                    self.root.set_fullscreen(!fullscreen)
                }
            }
            KeyCode::Escape => return true,
            KeyCode::Up => self.player.set_position(0, -1, &self.map),
            KeyCode::Down => self.player.set_position(0, 1, &self.map),
            KeyCode::Left => self.player.set_position(-1, 0, &self.map),
            KeyCode::Right => self.player.set_position(1, 0, &self.map),

            _ => {}
        }
        false
    }

    /* helper functions will need to refactor them */
    /// generates a map
    fn generate_map(player: &mut Character) -> Map {
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
                // all rooms after the first:
                // connect it to the previous room with a tunnel
            
                // center coordinates of the previous room
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();
            
                // toss a coin (random bool value -- either true or false)
                if rand::random() {
                    // first move horizontally, then vertically
                    Mapping::create_tunnel_horizontal(prev_x, new_x, prev_y, &mut map);
                    Mapping::crete_tunnel_vertical(prev_y, new_y, new_x, &mut map);
                } else {
                    // first move vertically, then horizontally
                    Mapping::crete_tunnel_vertical(prev_y, new_y, prev_x, &mut map);
                    Mapping::create_tunnel_horizontal(prev_x, new_x, new_y, &mut map);
                }
            }

            rooms.push(new_room);


        }
        map
    }
}
