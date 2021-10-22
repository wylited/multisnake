pub struct Player; // Component player strucuture

pub enum TileRotation {
    North,
    East,
    South,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SoutWest,
}

pub struct Position {
    x: u32, // x pos
    y: u32, // y pos
    z: u32, // Layer based pos
}

impl Position {
    pub fn new(x: u32, y: u32, z: u32) -> Position {
        Position{x, y, z}
    }

    /// Get a reference to the z pos.
    pub fn z(&self) -> &u32 {
        &self.z
    }
    /// Addtive change of z pos.
    pub fn change_z(&mut self, change: u32) {
        self.z += change;
    }

    /// Get a reference to the y pos.
    pub fn y(&self) -> &u32 {
        &self.y
    }
    /// Addtive change of y pos.
    pub fn change_y(&mut self, change: u32) {
        self.y += change;
    }

    /// Get a reference to the x pos.
    pub fn x(&self) -> &u32 {
        &self.x
    }
    /// Addtive change of the x pos.
    pub fn change_x(&mut self, change: u32) {
        self.x += change;
    }
}

pub enum Tile {
    ArenaCorner(TileRotation),
    ArenaWall(TileRotation),
    ArenaTile(TileRotation),
    PlayerHead(TileRotation),
    PlayerBody1(TileRotation),
    PlayerBody2(TileRotation),
    Blank,
}