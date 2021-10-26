pub struct Player; // Component player strucuture

pub enum TileType {
    ArenaCorner(TileRotation),
    ArenaWall(TileRotation),
    ArenaTile(TileRotation),
    PlayerHead(TileRotation),
    PlayerBody1(TileRotation),
    PlayerBody2(TileRotation),
    Blank,
} // Enumerations for the different tiles we can have.

pub enum TileRotation {
    North,
    East,
    South,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SoutWest,
} // Enumerations for the total amount of rotations a tile can have.

pub struct Position {
    x: u32, // x pos
    y: u32, // y pos
    z: u32, // Layer based pos
} // Position based structure for the tile

impl Position {
    pub fn new(x: u32, y: u32, z: u32) -> Position {
        Position{x, y, z}
    }
    
    /// Get a reference to the x pos.
    pub fn x(&self) -> &u32 {
        &self.x
    }
    /// Addtive change of the x pos.
    pub fn change_x(&mut self, change: u32) {
        self.x += change;
    }
    
    /// Get a reference to the y pos.
    pub fn y(&self) -> &u32 {
        &self.y
    }
    /// Addtive change of y pos.
    pub fn change_y(&mut self, change: u32) {
        self.y += change;
    }
    
    /// Get a reference to the z pos.
    pub fn z(&self) -> &u32 {
        &self.z
    }
    /// Addtive change of z pos.
    pub fn change_z(&mut self, change: u32) {
        self.z += change;
    }
} // Implemented functions for the positions

pub struct Tile {
    tiletype: TileType,
    pos: Position,
}// Data strucutre for a Tile

impl Tile {
    /// Get a reference to the tile's pos.
    pub fn pos(&self) -> &Position {
        &self.pos
    }

    pub fn x(&self) -> &u32 {
        self.pos.x()
    }
    pub fn change_x(&mut self, change: u32) {
        self.pos.change_x(change)
    }
    
    pub fn y(&self) -> &u32 {
        self.pos.y()
    } 
    pub fn change_y(&mut self, change: u32) {
        self.pos.change_y(change)
    }
    
    pub fn z(&self) -> &u32 {
        self.pos.z()
    }
    pub fn change_z(&mut self, change: u32) {
        self.pos.change_z(change)
    }

    /// Get a reference to the tile's type.
    pub fn tiletype(&self) -> &TileType {
        &self.tiletype
    }
    /// Set the tile's type.
    pub fn set_tiletype(&mut self, tiletype: TileType) {
        self.tiletype = tiletype;
    }
} //Implement functions for a Tile