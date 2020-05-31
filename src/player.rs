/// this struct houses our player, and contains important information such as position.
pub struct Player {
    pub x: i32,
    pub y: i32,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Player { x, y }
    }
    pub fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    
}
