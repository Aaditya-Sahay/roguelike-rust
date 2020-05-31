extern crate tcod;
mod app;
mod character;
mod map;
use tcod::colors::*;

///Some standard constants that we will use throughout the game
/// Width of the screen.
const SCREEN_WIDTH: i32 = 80;
/// Height of the screen
const SCREEN_HEIGHT: i32 = 60;
/// Maximum frames per second
const FPS_MAX: i32 = 60;



fn main() {
    // starting our app
    
    // starting our player
    
    let npc = character::Character::new(SCREEN_WIDTH / 3, SCREEN_HEIGHT / 3, '@', YELLOW);
    let npc_two = character::Character::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 4, '@', RED);
    let characters = vec![npc, npc_two];

    let mut app = app::Tcod::new(SCREEN_WIDTH, SCREEN_HEIGHT, characters);

    tcod::system::set_fps(FPS_MAX as i32);

    // initiate the game loop
    app.game_loop()
}
