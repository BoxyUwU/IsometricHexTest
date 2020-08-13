pub mod textures {
    pub const FLOOR: &str = "hex-grass-edges";
    pub const FLOOR_BRICK: &str = "hex-stone-floor";
    pub const WALL: &str = "hex-dirt";
    pub const WALL_BRICK: &str = "hex-stone";
    pub const MARKER: &str = "marker";
    pub const ARROW_SHEET: &str = "arrows";
    pub const BASE: &str = "base";
    pub const ENEMY: &str = "enemy";
}

pub mod draw_layers {
    pub const FLOOR: f32 = 0.0;
    pub const WALL: f32 = 1.0;
}

pub const CAM_SPEED: f32 = 5.0;

pub const MAX_FLOOR_HEIGHT: u8 = 2;
pub const MAX_BRICK_HEIGHT: u8 = 3;

pub const WIDTH: usize = 2;
pub const HEIGHT: usize = 2;

pub const DRAW_DOTS: bool = false;
pub const DRAW_FLOW: bool = false;