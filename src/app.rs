use crate::map::*;
use tcod::colors::*;
use tcod::console::*;
use tcod::input::*;

use tcod::map::Map as FovMap;

use crate::character::Character;
use crate::constants::*;

/// This struct houses our game runner
pub struct Tcod {
    pub root: Root,
    pub offscreen: Offscreen,
    pub map: Map,
    pub characters: Vec<Character>,
    pub player: Character,
    pub fov: FovMap,
    recompute_fov: bool,
    _width: i32,
    _height: i32,
}

impl Tcod {
    pub fn new(width: i32, height: i32) -> Tcod {
        let root = Root::initializer()
            .font("assets/consolas10x10_gs_tc.png", FontLayout::Tcod)
            .font_type(FontType::Greyscale)
            .size(width, height)
            .title("Game")
            .init();

        let offscreen = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);
        let fov = FovMap::new(MAP_WIDTH, MAP_HEIGHT);

        let mut player = Character::new(25, 23, '@', RED);
        let mut characters:Vec<Character> = vec![];
        let map = Mapping::generate_random_map(&mut player, &mut characters);

    
        Tcod {
            root,
            offscreen,
            map,
            characters,
            player,
            fov,
            recompute_fov: true,
            _width: width,
            _height: height,
        }
    }

    pub fn game_loop(&mut self) {
        // anything pre-init can happer here 
        self.compute_fov();


        while !self.root.window_closed() {
            self.offscreen.clear();
            self.render();
            self.root.flush();

            // exit out of the game if warranted
            let status_exit = self.handle_keys();
        

            if status_exit == true {
                break;
            }
        }
    }

    pub fn render(&mut self) {
        if self.recompute_fov {
            self.fov.compute_fov(
                self.player.x,
                self.player.y,
                TORCH_RADIUS,
                FOV_LIGHT_WALLS,
                FOV_ALGO,
            );
        }

        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let visible = self.fov.is_in_fov(x, y);
                let wall = self.map[x as usize][y as usize].opaque;

                let color = match (visible, wall) {
                    (false, true) => WALL_COLOR,
                    (true, true) => LIT_WALL_COLOR,
                    (true, false) => LIT_GROUND_COLOR,
                    (false, false) => GROUND_COLOR,
                };
                let explored = &mut self.map[x as usize][y as usize].explored;

                if visible {
                    *explored = true;
                }
                if *explored {
                    // show explored tiles only (any visible tile is explored already)
                    self.offscreen
                        .set_char_background(x, y, color, BackgroundFlag::Set);
                }
             
            }
        }

        self.player.draw(&mut self.offscreen);
        // drawing all characters
        for character in self.characters.iter() {
            character.draw(&mut self.offscreen)
        }

        blit(
            &self.offscreen,
            (0, 0),
            (MAP_WIDTH, MAP_HEIGHT),
            &mut self.root,
            (0, 0),
            1.0,
            1.0,
        );
    }

    pub fn handle_keys(&mut self) -> bool {
        let key: Key = self.root.wait_for_keypress(true);

        self.recompute_fov = true;
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

    fn compute_fov(&mut self) {
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                self.fov.set(
                    x,
                    y,
                    !self.map[x as usize][y as usize].opaque,
                    !self.map[x as usize][y as usize].blocked,
                )
            }
        }
    }
}
