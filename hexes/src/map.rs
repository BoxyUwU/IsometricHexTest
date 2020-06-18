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

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let mut tiles = vec![0; width * height];
        use rand::SeedableRng;
        use rand::Rng;
        let mut rand = rand::rngs::StdRng::seed_from_u64(100);
        for tile in tiles.iter_mut() {
            *tile = rand.gen_range(0, 6);
        }

        Map {
            tiles,
            width,
            height,
            position: Vec2::new(-(width as f32) / 2.0, -(height as f32) / 2.0,)
        }
    }
}

pub fn render_hex_map(drawables: NonSendSync<UniqueViewMut<Drawables>>, mut draw_buffer: UniqueViewMut<DrawBuffer>, map: UniqueView<Map>, camera: UniqueView<Camera>) {
    let camera_pos: Vec2<f32> = camera.position / Vec2::new(FLOOR_WIDTH, FLOOR_VERT_STEP) - map.position;

    let startx = (camera_pos.x - 20.0).max(0.0).min(map.width as f32 - 1.0) as usize;
    let endx = (camera_pos.x + 20.0).max(0.0).min(map.width as f32 - 1.0) as usize;
    let starty = (camera_pos.y - 20.0).max(0.0).min(map.height as f32 - 1.0) as usize;
    let endy = (camera_pos.y + 20.0).max(0.0).min(map.height as f32 - 1.0) as usize;

    for y in starty..=endy {
        for x in startx..=endx {
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
            let height = map.tiles[map.width * y + x];
            render_hex(&drawables, &mut draw_buffer, draw_x, draw_y, height);
        }
    }
}


pub fn render_hex(drawables: &NonSendSync<UniqueViewMut<Drawables>>, draw_buffer: &mut UniqueViewMut<DrawBuffer>, x: f32, y: f32, height: u8) {
    draw_buffer.draw(create_floor_draw_cmd(drawables, x, y, height as f32 * FLOOR_DEPTH_STEP, height / 2));

    let height = height as f32;
    let start_height = height * FLOOR_DEPTH_STEP - WALL_VERT_OFFSET;
    let walls_per_step = FLOOR_DEPTH_STEP / WALL_VERT_STEP;
    let walls_needed = walls_per_step * height;
    for i in 0..walls_needed as usize {
        let texture = 
            if (walls_needed as usize - i) % 2 == 1 {
                1
            } else {
                2
            };
        
        draw_buffer.draw(create_wall_draw_cmd(drawables, x, y, start_height - (i as f32 * WALL_VERT_STEP), texture));
    }
}

fn create_floor_draw_cmd(drawables: &NonSendSync<UniqueViewMut<Drawables>>, x: f32, y: f32, height: f32, texture: u8) -> DrawCommand {
    let color = 
        if texture == 0 {
            let v = 0.5;
            Color::rgba(v, v, v, 1.0)
        } else if texture == 1 {
            let v = 0.7;
            Color::rgba(v, v, v, 1.0)
        } else {
            let v = 0.9;
            Color::rgba(v, v, v, 1.0)
        };

    DrawCommand::new(drawables.alias[textures::FLOOR])
        .position(Vec3::new(x, y, height))
        .draw_layer(draw_layers::FLOOR)
        .draw_iso(true)
        .color(color)
}

fn create_wall_draw_cmd(drawables: &NonSendSync<UniqueViewMut<Drawables>>, x: f32, y: f32, height: f32, texture: u8) -> DrawCommand {
    let color =
        if texture == 1 {
            let v = 0.5;
            Color::rgba(v, v, v, 1.0)
        } else if texture == 2{
            let v = 0.7;
            Color::rgba(v, v, v, 1.0)
        } else {
            panic!();
        };

    DrawCommand::new(drawables.alias[textures::WALL])
        .position(Vec3::new(x, y, height))
        .draw_layer(draw_layers::WALL)
        .draw_iso(true)
        .color(color)
}