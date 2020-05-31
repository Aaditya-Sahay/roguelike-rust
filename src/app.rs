use tcod::console::*;
use tcod::input::*;


use crate::character::Character;
/// This struct houses our game runner
pub struct Tcod {
    pub root: Root,
    pub offscreen: Offscreen,
    width: i32, 
    height: i32
}

impl Tcod {
    pub fn new(width: i32, height: i32) -> Tcod {
        let root = Root::initializer()
            .font("assets/consolas10x10_gs_tc.png", FontLayout::Tcod)
            .font_type(FontType::Greyscale)
            .size(width, height)
            .title("Game")
            .init();
        let offscreen = Offscreen::new(width, height);
        Tcod { root, offscreen, width, height }
    }

    pub fn game_loop(&mut self, mut characters:Vec<&mut Character>) {
        while !self.root.window_closed() {
      
            self.offscreen.clear();

            for character in &characters {
                character.draw(&mut self.offscreen)
            }

            // combining offscreen with screen
            blit(&self.offscreen, (0,0), (self.width, self.height), &mut self.root, (0,0), 1.0, 1.0);
            self.root.flush();

            // exit out of the game if warranted
            let status_exit = self.handle_keys(&mut characters[0]);
            if status_exit == true {
                break
            }
    
        }
    }

    pub fn handle_keys(&mut self, player: &mut Character) -> bool {
        let key: Key = self.root.wait_for_keypress(true);
        match key.code {
            KeyCode::Enter => {
                if key.alt == true {
                    let fullscreen = self.root.is_fullscreen();
                    self.root.set_fullscreen(!fullscreen)
                }
            },
            KeyCode::Escape => return true,
            KeyCode::Up => player.set_position(0, -1),
            KeyCode::Down => player.set_position(0, 1),
            KeyCode::Left => player.set_position(-1, 0),
            KeyCode::Right => player.set_position(1, 0),

            _ => {}
        }
        false
    }
}
