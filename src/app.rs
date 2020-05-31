use tcod::console::*;
use tcod::input::*;

use crate::player::Player;
/// This struct houses our game runner
pub struct Tcod {
    pub root: Root,
}

impl Tcod {
    pub fn new(width: i32, height: i32) -> Tcod {
        let root = Root::initializer()
            .font("assets/consolas10x10_gs_tc.png", FontLayout::Tcod)
            .font_type(FontType::Greyscale)
            .size(width, height)
            .title("Game")
            .init();

        Tcod { root }
    }
    pub fn handle_keys(&mut self, player: &mut Player) -> bool {
        let key: Key = self.root.wait_for_keypress(true);
        match key.code {
            KeyCode::Enter => {
                if key.alt == true {
                    let fullscreen = self.root.is_fullscreen();
                    self.root.set_fullscreen(!fullscreen)
                }
            },
            KeyCode::Escape => return true,
            KeyCode::Up => player.y -= 1,
            KeyCode::Down => player.y += 1,
            KeyCode::Left => player.x -= 1,
            KeyCode::Right => player.x += 1,

            _ => {}
        }
        false
    }
}
