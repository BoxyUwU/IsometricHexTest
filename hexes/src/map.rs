use vermarine_lib::hexmap::{
    TileData,
};

#[derive(Clone, Copy, Debug)]
pub struct HexTileData {
    pub ground_height: u8,
    pub wall_height: u8,

    
}

impl HexTileData {
    pub fn new(height: u8) -> HexTileData {
        HexTileData {
            ground_height: height,
            wall_height: height,
        }
    }
}

impl TileData for HexTileData {
    fn height(&self) -> u8 {
        self.wall_height
    }
}