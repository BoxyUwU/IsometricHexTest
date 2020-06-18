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
            Vec2,
        },
        graphics::{
            Camera,
            Color,
        },
    },
};

use vermarine_lib::{
    rendering::{
        draw_buffer::{
            DrawBuffer,
            DrawCommand,
        },
        Drawables,
    },
};

pub struct Map {
    tiles: Vec<u8>,
    width: usize,
    height: usize,
    position: Vec2<f32>,
}

use rand::SeedableRng;
use rand::Rng;

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let mut tiles: Vec<u8> = vec![0; width * height];
        let mut rand = rand::rngs::StdRng::seed_from_u64(100);

        for tile in tiles.iter_mut() {
            *tile = rand.gen_range(0, 3);
        }

        Map {
            tiles,
            width,
            height,
            position: Vec2::new(-(width as f32) / 2.0, -(height as f32) / 2.0),
        }
    }
}

pub fn render_hex_map(drawables: NonSendSync<UniqueViewMut<Drawables>>, mut draw_buffer: UniqueViewMut<DrawBuffer>, map: UniqueView<Map>, camera: UniqueView<Camera>) {
    draw_buffer.new_command_pool(true);
    
    let camera_pos: Vec2<f32> = camera.position / Vec2::new(FLOOR_WIDTH, FLOOR_VERT_STEP) - map.position;

    let startx = (camera_pos.x - 20.0).max(0.0).min(map.width as f32 - 1.0) as usize;
    let endx = (camera_pos.x + 20.0).max(0.0).min(map.width as f32 - 1.0) as usize;
    let starty = (camera_pos.y - 20.0).max(0.0).min(map.height as f32 - 1.0) as usize;
    let endy = (camera_pos.y + 20.0).max(0.0).min(map.height as f32 - 1.0) as usize;

    let (top_tex, wall_tex) = (drawables.alias[textures::FLOOR], drawables.alias[textures::WALL]);
    for y in starty..endy {
        for i in 0..2 {
            for x in startx..endx {
                let (draw_x, draw_y) =
                (
                    if y % 2 == 0 {
                        (x as i32) as f32 * FLOOR_WIDTH
                    } else {
                        (x as i32) as f32 * FLOOR_WIDTH - (FLOOR_WIDTH / 2.0)
                    },
                    (y as i32) as f32 * (FLOOR_VERT_STEP)
                );

                let (draw_x, draw_y) =
                    (
                        draw_x + map.position.x * FLOOR_WIDTH,
                        draw_y + map.position.y * FLOOR_VERT_STEP,
                    );
                let tile_height = map.tiles[map.width * y + x];

                if i == 0 {
                    render_hex_walls(&mut draw_buffer, draw_x, draw_y, tile_height, wall_tex);
                }
                if i == 1 {
                    render_hex_top(&mut draw_buffer, draw_x, draw_y, tile_height, top_tex);
                }
            }
        }
    }

    draw_buffer.end_command_pool();
}

pub fn render_hex_top(draw_buffer: &mut UniqueViewMut<DrawBuffer>, x: f32, y: f32, height: u8, texture: u64) {
    draw_buffer.draw(create_floor_draw_cmd(x, y, height as f32 * FLOOR_DEPTH_STEP, height, texture));
}

pub fn render_hex_walls(draw_buffer: &mut UniqueViewMut<DrawBuffer>, x: f32, y: f32, height: u8, texture: u64) {
    let height = height as f32;
    let start_height = height * FLOOR_DEPTH_STEP - WALL_VERT_OFFSET;
    let walls_per_step = FLOOR_DEPTH_STEP / WALL_VERT_STEP;
    let walls_needed = walls_per_step * height;
    for i in 0..walls_needed as usize {
        let color = 
            if (walls_needed as usize - i) % 2 == 1 {
                1
            } else {
                2
            };
        
        draw_buffer.draw(create_wall_draw_cmd(x, y, start_height - (i as f32 * WALL_VERT_STEP), color, texture));
    }
}

fn create_floor_draw_cmd(x: f32, y: f32, height: f32, color: u8, texture: u64) -> DrawCommand {
    let color = 
        if color == 0 {
            let v = 0.6;
            Color::rgba(v, v, v, 1.0)
        } else if color == 1 {
            let v = 0.8;
            Color::rgba(v, v, v, 1.0)
        } else {
            let v = 1.0;
            Color::rgba(v, v, v, 1.0)
        };

    DrawCommand::new(texture)
        .position(Vec3::new(x, y, height))
        .draw_layer(draw_layers::FLOOR)
        .draw_iso(true)
        .color(color)
}

fn create_wall_draw_cmd(x: f32, y: f32, height: f32, color: u8, texture: u64) -> DrawCommand {
    let color =
        if color == 1 {
            let v = 0.5;
            Color::rgba(v, v, v, 1.0)
        } else if color == 2{
            let v = 0.7;
            Color::rgba(v, v, v, 1.0)
        } else {
            panic!();
        };

    DrawCommand::new(texture)
        .position(Vec3::new(x, y, height))
        .draw_layer(draw_layers::WALL)
        .draw_iso(true)
        .color(color)
}