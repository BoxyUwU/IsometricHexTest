pub mod textures {
    pub const FLOOR: &'static str = "hex-grass";
    pub const FLOOR_DIM: &'static str = "hex-grass-dim";
    pub const FLOOR_DARK: &'static str = "hex-grass-dark";
    pub const WALL: &'static str = "hex-dirt";
}

pub mod draw_layers {
    pub const FLOOR: f32 = 0.0;
    pub const WALL: f32 = -1.0;
}

pub const TILE_WIDTH: f32 = 48.0;
pub const TILE_HEIGHT: f32 = 36.0;
pub const TILE_VERT_STEP: f32 = 24.0;