use tcod::colors::*;
use tcod::map::FovAlgorithm;

pub const SCREEN_WIDTH: i32 = 80;
/// Height of the screen
pub const SCREEN_HEIGHT: i32 = 60;
/// Maximum frames per second
pub const FPS_MAX: i32 = 60;
pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 45;

pub const ROOM_MAX_SIZE: i32 = 10;
pub const ROOM_MIN_SIZE: i32 = 6;
pub const MAX_ROOMS: i32 = 30;
pub const MAX_ROOM_MONSTERS: i32 = 3;

pub const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic; // default FOV algorithm
pub const FOV_LIGHT_WALLS: bool = true; // light walls or not
pub const TORCH_RADIUS: i32 = 6;


// colors 
pub const WALL_COLOR: Color = Color { r: 101, g: 67, b: 42 };
pub const GROUND_COLOR: Color = Color {
    r: 128,
    g: 128,
    b: 128,
};
pub const LIT_WALL_COLOR: Color = Color {
    r: 130,
    g: 110,
    b: 50,
};
pub const LIT_GROUND_COLOR: Color = Color {
    r: 225,
    g: 225,
    b: 225,
};