use crate::map::*;
use tcod::colors::*;
use tcod::console::*;
use tcod::input::*;


use crate::character::Character;

/// height and width of the map
const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 45;
/// Room constants


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
        let mut map = Mapping::generate_random_map(&mut player);

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
}
