extern crate tcod;
use tcod::colors::*;
use tcod::console::*;
mod app;
mod player;

///Some standard constants that we will use throughout the game
/// Width of the screen.
const SCREEN_WIDTH: i32 = 80;
/// Height of the screen
const SCREEN_HEIGHT: i32 = 60;
/// Maximum frames per second
const FPS_MAX: i32 = 60;

fn main() {
    // starting our app
    let mut app = app::Tcod::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    // starting our player 
    let mut player = player::Player::new(SCREEN_WIDTH/2, SCREEN_HEIGHT/2);


    tcod::system::set_fps(FPS_MAX as i32);

    // this is our game loop, which will be refactored later
    while !app.root.window_closed() {
        app.root.set_default_foreground(WHITE);
        app.root.clear();
        app.root.put_char(player.x, player.y, '@', BackgroundFlag::None);
        app.root.flush();
        let status_exit = app.handle_keys(&mut player);
        if status_exit == true {
            break
        }

    }
}
