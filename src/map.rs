
/// Will be our tile, this has clone and copy traits
#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub blocked: bool,
    pub opaque: bool,
}

impl Tile { 
    /// generates an empty space
    pub fn empty() -> Self {
        Tile {blocked: false, opaque: false}
    }
    /// generates a tile which emulates a wall
    pub fn wall() -> Self {
        Tile {blocked: true, opaque: true}
    }

}

pub type Map = Vec<Vec<Tile>>;