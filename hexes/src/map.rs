use crate::{
    shipyard::{
        *,
    },
    consts::{
        *,
    },
    tetra::{
        math::{
            Vec3,
        },
    },
};

use vermarine_lib::{
    rendering::{
        draw_buffer::{
            DrawBuffer,
            DrawCommand,
        },
    },
};

pub struct Map {
    tiles: Vec<u8>,
}

impl Map {
    pub fn new() -> Self {
        let mut tiles = vec![0; 100 * 100];
        use rand::SeedableRng;
        use rand::Rng;
        let mut rand = rand::rngs::StdRng::seed_from_u64(100);
        for tile in tiles.iter_mut() {
            *tile = rand.gen_range(0, 3);
        }
        Map {
            tiles
        }
    }
}

pub fn render_hex_map(mut draw_buffer: UniqueViewMut<DrawBuffer>, map: UniqueView<Map>) {
    for y in 0..100 {
        for x in 0..100 {
            let (draw_x, draw_y) =
                (
                    if y % 2 == 0 {
                        (x as i32 - 50) as f32 * TILE_WIDTH
                    } else {
                        (x as i32 - 50) as f32 * TILE_WIDTH - (TILE_WIDTH / 2.0)
                    },
                    (y as i32 - 50) as f32 * (TILE_VERT_STEP)
                );
            
            let height = map.tiles[100 * y + x];
            render_hex(&mut draw_buffer, draw_x, draw_y, height);
        }
    }
}


pub fn render_hex(draw_buffer: &mut UniqueViewMut<DrawBuffer>, x: f32, y: f32, height: u8) {
    draw_buffer.draw(create_floor_draw_cmd(x, y, height as f32 * TILE_VERT_STEP, height));

    let start_height = height as f32 * TILE_VERT_STEP - TILE_HEIGHT / 3.0;
    for i in 0..height as usize * 2 {
        draw_buffer.draw(create_wall_draw_cmd(x, y, start_height - i as f32 * TILE_HEIGHT / 3.0));
    }
}

fn create_floor_draw_cmd(x: f32, y: f32, height: f32, texture: u8) -> DrawCommand {
    let texture = 
        if texture == 0 {
            textures::FLOOR_DARK
        } else if texture == 1 {
            textures::FLOOR_DIM
        } else {
            textures::FLOOR
        };

    DrawCommand::new(texture)
        .position(Vec3::new(x, y, height))
        .draw_layer(draw_layers::FLOOR)
        .draw_iso(true)
}

fn create_wall_draw_cmd(x: f32, y: f32, height: f32) -> DrawCommand {
    DrawCommand::new(textures::WALL)
        .position(Vec3::new(x, y, height))
        .draw_layer(draw_layers::WALL)
        .draw_iso(true)
}