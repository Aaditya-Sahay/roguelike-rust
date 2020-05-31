use tcod::colors::*;
use tcod::console::*;

/// this struct houses our player, and contains important information such as position.
pub struct Character {
    pub x: i32,
    pub y: i32,
    pub literal: char,
    pub color: Color,

}
impl Character {
    pub fn new(x: i32, y: i32, literal: char, color: Color) -> Self {
        Character { x, y, literal, color }
    }
    pub fn set_position(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }
    /// characters are able to draw themselves when given a mutable ref to an object that implements console
    pub fn draw(&self, screen: &mut dyn Console) {
        screen.set_default_foreground(self.color);
        screen.put_char(self.x, self.y, self.literal, BackgroundFlag::None);
    }
    
}
