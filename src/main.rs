extern crate tcod;
extern crate rand;

mod app;
mod character;
mod map;
mod room;
mod constants;


///Some standard constants that we will use throughout the game
/// Width of the screen.
use constants::*;



fn main() {
    // starting our app
    
    // starting our player
    
    // let npc = character::Character::new(SCREEN_WIDTH / 3, SCREEN_HEIGHT / 3, '@', YELLOW);
    // let npc_two = character::Character::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 4, '@', RED);
    let characters = vec![];

    let mut app = app::Tcod::new(SCREEN_WIDTH, SCREEN_HEIGHT, characters);

    // limit max fps count
    tcod::system::set_fps(FPS_MAX);

    // initiate the game loop
    app.game_loop()
}
