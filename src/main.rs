extern crate tcod;
mod app;
mod character;
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
    let mut app = app::Tcod::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    // starting our player 
    let mut player = character::Character::new(SCREEN_WIDTH/2, SCREEN_HEIGHT/2, '@', WHITE);
    let mut npc = character::Character::new(SCREEN_WIDTH/3, SCREEN_HEIGHT/3, '@', YELLOW);
    let characters = vec![&mut player, &mut npc];


    tcod::system::set_fps(FPS_MAX as i32);

    // initiate the game loop
    app.game_loop(characters)
}
