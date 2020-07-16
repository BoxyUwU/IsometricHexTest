use crate::{
    shipyard::{
        *,
    },
    tetra::{
        InputContext,
        graphics::{
            Camera,
            Color,
        },
        input::{
            self,
            Key,
            MouseButton,
        },
        math::{
            Vec2,
            Vec3,
        },
    },
    consts::{
        *,
    },
    map::{
        HexMap,
    },
};

use vermarine_lib::{
    rendering::{
        Drawables,
        draw_buffer::{
            DrawCommand,
            DrawBuffer,
        },
    },
};


pub fn move_camera(mut camera: UniqueViewMut<Camera>, input: UniqueView<InputContext>) {
    let mut movement: Vec2<f32> = Vec2::new(0.0, 0.0);

    for entry in [
        (Key::Up, Vec2::new(0.0, -1.0)),
        (Key::Down, Vec2::new(0.0, 1.0)),
        (Key::Left, Vec2::new(-1.0, 0.0)),
        (Key::Right, Vec2::new(1.0, 0.0)),
    ].iter() {
        if input::is_key_down(&input, entry.0) {
            movement += entry.1;
        }
    }

    if movement != Vec2::new(0.0, 0.0) {
        movement.normalize();
        movement *= CAM_SPEED;
        movement.x = movement.x.floor();
        movement.y = movement.y.floor();        
        camera.position += movement;
    }
}

pub fn update_hex_map(input_ctx: UniqueView<InputContext>, mut map: UniqueViewMut<HexMap>, camera: UniqueView<Camera>) {
    let axial = 
        if let Some(hex) = map.pixel_to_hex(camera.mouse_position(&input_ctx)) {
            hex
        } else {
            return;
        };
    
    let tile = map.get_tile_mut(&axial.to_hex());

    if input::is_mouse_button_pressed(&input_ctx, MouseButton::Left) {
        if tile.ground_height > tile.wall_height && tile.ground_height > 0 {
            tile.ground_height -= 1;
        }
        else if tile.wall_height > tile.ground_height && tile.wall_height > 0 {
            tile.wall_height -= 1;
        }
        else if tile.wall_height == tile.ground_height && tile.wall_height > 0 {
            tile.wall_height -= 1;
            tile.ground_height -= 1;
        }
    } else if input::is_mouse_button_pressed(&input_ctx, MouseButton::Right) {
        if tile.ground_height > tile.wall_height {
            tile.wall_height = tile.ground_height + 1;
        }
        else if tile.wall_height >= tile.ground_height && tile.wall_height < MAX_BRICK_HEIGHT {
            tile.wall_height += 1;
        }
    }

    let height = tile.wall_height;
    if height > map.tallest {
        map.tallest = height;
    }
}

pub fn render_hex_map(
    input_ctx: UniqueView<InputContext>, 
    drawables: NonSendSync<UniqueViewMut<Drawables>>, 
    mut draw_buffer: UniqueViewMut<DrawBuffer>, 
    mut map: UniqueViewMut<HexMap>, 
    camera: UniqueView<Camera>
) {
    draw_buffer.new_command_pool(true);
    let command_pool = draw_buffer.get_command_pool();

    let mouse_pos = camera.mouse_position(&input_ctx);
    let selected_hex = map.pixel_to_hex(mouse_pos);

    use vermarine_lib::hexmap::FractionalAxial;
    let FractionalAxial { q, r }  = map.pixel_to_hex_raw(camera.position, 0.);

    let startq = (q - 40.0) as i32;
    let endq = (q + 40.0) as i32;
    let startr = (r - 20.0) as i32;
    let endr = (r + 20.0) as i32;

    let (top_tex, wall_tex, brick_tex, brick_floor_tex) = 
        (
            drawables.alias[textures::FLOOR], 
            drawables.alias[textures::WALL], 
            drawables.alias[textures::WALL_BRICK], 
            drawables.alias[textures::FLOOR_BRICK]
        );

    for height in 0..=map.tallest {
        let mut wall_buffer: Vec<DrawCommand> = Vec::with_capacity(1024);
        let mut wall_brick_buffer: Vec<DrawCommand> = Vec::with_capacity(1024);
        let mut top_buffer: Vec<DrawCommand> = Vec::with_capacity(1024);
        let mut top_brick_buffer: Vec<DrawCommand> = Vec::with_capacity(1024);
        for r in startr..=endr {
            for q in startq..=endq {
                use vermarine_lib::hexmap::Axial;
                let axial = Axial::new(q as i32, r as i32);

                let tile = if let Some(tile) = map.try_get_tile(&axial.to_hex()) {
                    tile
                } else {
                    continue;
                };

                if tile.wall_height < height {
                    continue;
                }

                let (draw_x, draw_y) = {
                    let offset_x = (map.hex_width / 2.0) * r as f32;
                    let mut x = map.hex_width * q as f32;
                    x += offset_x;
                    (
                        x + map.position.x,
                        ((r as i32) as f32 * (map.hex_vert_step)) + map.position.y
                    )
                };
                
                if height <= tile.ground_height && height != 0 {
                    render_hex_walls(&map, &mut wall_buffer, draw_x, draw_y, height, wall_tex);
                }
                if height > tile.ground_height && height <= tile.wall_height {
                    render_hex_bricks(&map, &mut wall_brick_buffer, draw_x, draw_y, height, brick_tex);
                }

                let color = if let Some(axial) = selected_hex {
                    let color = if q == axial.q && r == axial.r {
                        Color::RED
                    } else {
                        Color::WHITE
                    };
                    color
                } else {
                    Color::WHITE
                };

                if height == tile.ground_height && height == tile.wall_height {
                    render_hex_top(&map, &mut top_buffer, draw_x, draw_y, tile.ground_height, top_tex, color);
                }
                if height == tile.wall_height && height != tile.ground_height {
                    render_hex_brick_top(&map, &mut top_brick_buffer, draw_x, draw_y, tile.wall_height, brick_floor_tex, color);
                }
            }
        }
        command_pool.commands.extend(&wall_buffer);
        command_pool.commands.extend(&wall_brick_buffer);
        command_pool.commands.extend(&top_buffer);
        command_pool.commands.extend(&top_brick_buffer);
    }
    
    // Draw dots at hex centers
    /*let marker_tex = drawables.alias[textures::MARKER];
    for r_tile in startr..=endr {
        for q_tile in startq..=endq {
            use vermarine_lib::hexmap::Axial;
            let axial = Axial::new(q_tile as i32, r_tile as i32);
            let (x, y) = map.axial_to_pixel(axial);
            
            let tile = if let Some(tile) = map.try_get_tile(&axial.to_hex()) {
                tile
            } else {
                continue;
            };

            draw_buffer.draw(
                DrawCommand::new(marker_tex)
                    .position(Vec3::new(
                        x - 2.0, y - 2.0, tile.wall_height as f32 * map.hex_depth_step 
                    ))
                    .draw_iso(true)
            );
        }
    }*/

    draw_buffer.end_command_pool();
}

pub fn render_hex_top(map: &HexMap, draw_buffer: &mut Vec<DrawCommand>, x: f32, y: f32, height: u8, texture: u64, color: Color) {
    let mut draw_command = create_floor_draw_cmd(x, y, height as f32 * map.hex_depth_step, height, texture); 
    if color != Color::WHITE {
        draw_command = draw_command.color(color);
    }
    draw_buffer.push(draw_command);
}

fn create_floor_draw_cmd(x: f32, y: f32, height: f32, color: u8, texture: u64) -> DrawCommand {
    let color = 
        if color == 0 {
            let v = 0.55;
            Color::rgba(v, v, v, 1.0)
        } else if color == 1 {
            let v = 0.8;
            Color::rgba(v, v, v, 1.0)
        } else {
            let v = 0.95;
            Color::rgba(v, v, v, 1.0)
        };

    DrawCommand::new(texture)
        .position(Vec3::new(x, y, height))
        .draw_layer(draw_layers::FLOOR)
        .draw_iso(true)
        .color(color)
}

pub fn render_hex_brick_top(map: &HexMap, draw_buffer: &mut Vec<DrawCommand>, x: f32, y: f32, height: u8, texture: u64, color: Color) {
    let mut draw_command = create_brick_floor_draw_cmd(x, y, height as f32 * map.hex_depth_step, height, texture); 
    if color != Color::WHITE {
        draw_command = draw_command.color(color);
    }
    draw_buffer.push(draw_command);
}

fn create_brick_floor_draw_cmd(x: f32, y: f32, height: f32, color: u8, texture: u64) -> DrawCommand {
    let color = 
        if color == 1 {
            let v = 0.65;
            Color::rgba(v, v, v, 1.0)
        } else if color == 2 {
            let v = 0.8;
            Color::rgba(v, v, v, 1.0)
        } else if color == 3 {
            let v = 0.9;
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

pub fn render_hex_walls(map: &HexMap, draw_buffer: &mut Vec<DrawCommand>, x: f32, y: f32, height: u8, wall_tex: u64) {
    let start_height = height as f32 * map.hex_depth_step - map.wall_vert_offset;
    let color = 
        if height % 2 == 1 {
            1
        } else {
            2
        };
    draw_buffer.push(
        create_wall_draw_cmd(x, y, start_height, color, wall_tex)
    );
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
            let v = 1.0;
            Color::rgba(v, v, v, 1.0)
        };

    DrawCommand::new(texture)
        .position(Vec3::new(x, y, height))
        .draw_layer(draw_layers::WALL)
        .draw_iso(true)
        .color(color)
}

pub fn render_hex_bricks(map: &HexMap, draw_buffer: &mut Vec<DrawCommand>, x: f32, y: f32, height: u8, brick_tex: u64) {
    let start_height = height as f32 * map.hex_depth_step - map.wall_vert_step;
    draw_buffer.push(
        create_wall_brick_draw_cmd(x, y, start_height, height, brick_tex)
    );
}

fn create_wall_brick_draw_cmd(x: f32, y: f32, height: f32, color: u8, texture: u64) -> DrawCommand {
    let color =
        if color == 1 {
            let v = 0.3;
            Color::rgba(v, v, v, 1.0)
        } else if color == 2 {
            let v = 0.55;
            Color::rgba(v, v, v, 1.0)
        } else if color == 3 {
            let v = 0.7;
            Color::rgba(v, v, v, 1.0)
        } else {
            let v = 0.80;
            Color::rgba(v, v, v, 1.0)
        };

    DrawCommand::new(texture)
        .position(Vec3::new(x, y, height))
        .draw_layer(draw_layers::WALL)
        .draw_iso(true)
        .color(color)
}