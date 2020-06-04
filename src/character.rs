use tcod::colors::*;
use tcod::console::*;
use crate::map::*;

/// this struct houses our characters, and contains important information such as position.
pub struct Character {
    pub x: i32,
    pub y: i32,
    pub literal: char,
    pub color: Color,
    pub block: bool,
    pub alive: bool,

}
impl Character {
    pub fn new(x: i32, y: i32, literal: char, color: Color) -> Self {
        Character { x, y, literal, block:false, alive: true, color }
    }

    pub fn set_position(&mut self, dx: i32, dy: i32, map: &Map) {
        // wall logic
        if !map[(self.x+dx) as usize][(self.y+dy) as usize].blocked{
            self.x += dx;
            self.y += dy;
        }
    }
    /// characters are able to draw themselves when given a mutable ref to an object that implements console
    pub fn draw(&self, screen: &mut dyn Console) {
        screen.set_default_foreground(self.color);
        screen.put_char(self.x, self.y, self.literal, BackgroundFlag::None);
    }
    
    pub fn get_pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

/// This enum houses player actions
#[derive(PartialEq)]
pub enum PlayerActions {
    TookTurn,
    DidNothing,
    Exit,
}