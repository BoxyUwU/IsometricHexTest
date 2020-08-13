use crate::{
    shipyard::{
        *,
    },
    tetra::{
        InputContext,
        graphics::{
            Camera,
            Color,
            Rectangle,
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
        HexTileData,
        HexPathNode,
        Map,
    },
    components::{
        Agent,
        Transform,
        Spawner,
    },
    entity_creator,
};

use vermarine_lib::{
    rendering::{
        Drawables,
        Sprite,
        draw_buffer::{
            DrawCommand,
            DrawBuffer,
        },
    },
    hexmap::{
        HexMap,
        Axial,
        FractionalAxial,
    },
};

pub fn move_agents(map: UniqueView<Map>, agents: View<Agent>, mut transforms: ViewMut<Transform>) {
    for (_, transform) in (&agents, &mut transforms).iter() {
        if let Some(path) = map.get_path(transform.position.to_hex()) {
            if let Some(pos) = path.get(1) {
                transform.position = pos.to_axial();
            }
        }
        
    }
}

pub fn draw_entities(
    map: UniqueView<Map>,
    mut draw_buffer: UniqueViewMut<DrawBuffer>, 
    transforms: View<Transform>,
    sprites: View<Sprite>,
) {
    draw_buffer.new_command_pool(false);

    for (transform, sprite) in (&transforms, &sprites).iter() {
        let depth = 
            if let Some(hex) = map.terrain.get_tile(transform.position.to_hex()) {
                hex.wall_height
            } else {
                0
            };

        let offset_y = depth as f32 * map.terrain.hex_depth_step * -1.;

        let mut draw_cmd = sprite.0;
        let position = map.terrain.axial_to_pixel(transform.position);
        draw_cmd.position.x += position.0;
        draw_cmd.position.y += position.1 + offset_y;

        draw_buffer.draw(draw_cmd);
    }

    draw_buffer.end_command_pool();
}

pub fn spawn_agents(mut all_storages: AllStoragesViewMut) {
    let spawns = all_storages.run(|positions: View<Transform>, mut spawners: ViewMut<Spawner>| {
        let mut spawns = vec![];

        for (pos, spawner) in (&positions, &mut spawners).iter() {
            spawner.counter -= 1;

            if spawner.counter == 0 {
                spawner.counter = spawner.period;

                spawns.push(pos.position);
            }
        }

        spawns
    });
    
    for position in spawns {
        entity_creator::create_agent(position, &mut all_storages);
    }
}

pub fn draw_agent_paths(
    drawables: NonSendSync<UniqueViewMut<Drawables>>, 
    mut draw_buffer: UniqueViewMut<DrawBuffer>, 
    map: UniqueView<Map>, 
    agents: View<Agent>,
    transforms: View<Transform>,
) {
    let arrow_sheet = drawables.alias[textures::ARROW_SHEET];
    for (_, transform) in (&agents, &transforms).iter() {
        if let Some(path) = map.get_path(transform.position.to_hex()) {
            for step in path {
                draw_arrow(&mut draw_buffer, arrow_sheet, &map, step.to_axial());
            }
        }
    }
}

pub fn draw_arrow(draw_buffer: &mut DrawBuffer, arrow_sheet: u64, map: &Map, tile: Axial) {
    let (x, y) = map.terrain.axial_to_pixel(tile);

    let (terrain_tile, flow_tile) = 
        if let (Some(terrain_tile), Some(flow_tile)) = (
            map.terrain.get_tile(tile.to_hex()),
            map.dijkstra.get_tile(tile.to_hex()),
        ) {
            (terrain_tile, flow_tile)
        } else {
            return;
        };

    draw_buffer.draw(
        DrawCommand::new(arrow_sheet)
            .position(Vec3::new(
                x, y, terrain_tile.wall_height as f32 * map.terrain.hex_depth_step 
            ))
            .draw_iso(true)
            .clip(
                match flow_tile {
                    HexPathNode::TopLeft => Rectangle::row(0., 0., 36., 36.).next().unwrap(),
                    HexPathNode::TopRight => Rectangle::row(0., 0., 36., 36.).nth(1).unwrap(),
                    HexPathNode::BottomRight => Rectangle::row(0., 0., 36., 36.).nth(2).unwrap(), 
                    HexPathNode::BottomLeft => Rectangle::row(0., 0., 36., 36.).nth(3).unwrap(), 
                    HexPathNode::Right => Rectangle::row(0., 0., 36., 36.).nth(4).unwrap(), 
                    HexPathNode::Left => Rectangle::row(0., 0., 36., 36.).nth(5).unwrap(),
                    _ => return,
                }
            )
    );
}

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

pub fn update_hex_map(input_ctx: UniqueView<InputContext>, mut map: UniqueViewMut<Map>, camera: UniqueView<Camera>) {
    let axial = 
        if let Some(hex) = map.terrain.pixel_to_hex(camera.mouse_position(&input_ctx)) {
            hex
        } else {
            return;
        };
    
    let tile = map.terrain.get_tile_mut(axial.to_hex()).unwrap();

    let mut modified = false;

    if input::is_mouse_button_pressed(&input_ctx, MouseButton::Left) {
        if tile.ground_height > tile.wall_height && tile.ground_height > 0 {
            tile.ground_height -= 1;
            modified = true;
        }
        else if tile.wall_height > tile.ground_height && tile.wall_height > 0 {
            tile.wall_height -= 1;
            modified = true;
        }
        else if tile.wall_height == tile.ground_height && tile.wall_height > 0 {
            tile.wall_height -= 1;
            tile.ground_height -= 1;
            modified = true;
        }
    } else if input::is_mouse_button_pressed(&input_ctx, MouseButton::Right) {
        if tile.ground_height > tile.wall_height {
            tile.wall_height = tile.ground_height + 1;
            modified = true;
        }
        else if tile.wall_height >= tile.ground_height && tile.wall_height < MAX_BRICK_HEIGHT {
            tile.wall_height += 1;
            modified = true;
        }
    }

    if modified {
        let tile = map.terrain.get_tile_mut(axial.to_hex()).unwrap();
        
        let height = tile.wall_height;
        if height > map.terrain.tallest {
            map.terrain.tallest = height;
        }
        
        let goal_hex = Axial::new(10, 5).to_hex();
        let map = &mut *map;
        crate::map::update_dijkstra_hexmap(&map.terrain, &mut map.dijkstra, vec![goal_hex + Axial::new(0, 1), goal_hex + Axial::new(1, 1)]);
    }
}

pub fn draw_hex_map(
    input_ctx: UniqueView<InputContext>, 
    drawables: NonSendSync<UniqueViewMut<Drawables>>, 
    mut draw_buffer: UniqueViewMut<DrawBuffer>, 
    mut map: UniqueViewMut<Map>, 
    camera: UniqueView<Camera>
) {
    draw_buffer.new_command_pool(true);
    let command_pool = draw_buffer.get_command_pool();

    let mouse_pos = camera.mouse_position(&input_ctx);
    let selected_hex = map.terrain.pixel_to_hex(mouse_pos);

    let FractionalAxial { q, r }  = map.terrain.pixel_to_hex_raw(camera.position, 0.);

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

    for height in 0..=map.terrain.tallest {
        let mut wall_buffer: Vec<DrawCommand> = Vec::with_capacity(1024);
        let mut wall_brick_buffer: Vec<DrawCommand> = Vec::with_capacity(1024);
        let mut top_buffer: Vec<DrawCommand> = Vec::with_capacity(1024);
        let mut top_brick_buffer: Vec<DrawCommand> = Vec::with_capacity(1024);
        for r in startr..=endr {
            for q in startq..=endq {
                let axial = Axial::new(q as i32, r as i32);

                let tile = if let Some(tile) = map.terrain.get_tile(axial.to_hex()) {
                    tile
                } else {
                    continue;
                };

                if tile.wall_height < height {
                    continue;
                }

                let (draw_x, draw_y) = {
                    let offset_x = (map.terrain.hex_width / 2.0) * r as f32;
                    let mut x = map.terrain.hex_width * q as f32;
                    x += offset_x;
                    (
                        x + map.terrain.position.x,
                        (r as f32 * map.terrain.hex_vert_step) + map.terrain.position.y
                    )
                };
                
                if height <= tile.ground_height {
                    draw_hex_walls(&map.terrain, &mut wall_buffer, draw_x, draw_y, height, wall_tex);
                }
                if height > tile.ground_height && height <= tile.wall_height {
                    draw_hex_bricks(&map.terrain, &mut wall_brick_buffer, draw_x, draw_y, height, brick_tex);
                }

                let color = if let Some(axial) = selected_hex {
                    if q == axial.q && r == axial.r {
                        Color::RED
                    } else {
                        Color::WHITE
                    }
                } else {
                    Color::WHITE
                };

                if height == tile.ground_height && height == tile.wall_height {
                    draw_hex_top(&map.terrain, &mut top_buffer, draw_x, draw_y, tile.ground_height, top_tex, color);
                }
                if height == tile.wall_height && height != tile.ground_height {
                    draw_hex_brick_top(&map.terrain, &mut top_brick_buffer, draw_x, draw_y, tile.wall_height, brick_floor_tex, color);
                }
            }
        }
        command_pool.commands.extend(&wall_buffer);
        command_pool.commands.extend(&wall_brick_buffer);
        command_pool.commands.extend(&top_buffer);
        command_pool.commands.extend(&top_brick_buffer);
    }
    
    // Draw dots at hex centers
    if DRAW_DOTS {
        let marker_tex = drawables.alias[textures::MARKER];
        for r_tile in startr..=endr {
            for q_tile in startq..=endq {
                let axial = Axial::new(q_tile as i32, r_tile as i32);
                let (x, y) = map.terrain.axial_to_pixel(axial);
                
                let tile = if let Some(tile) = map.terrain.get_tile(axial.to_hex()) {
                    tile
                } else {
                    continue;
                };
    
                draw_buffer.draw(
                    DrawCommand::new(marker_tex)
                        .position(Vec3::new(
                            x, y, tile.wall_height as f32 * map.terrain.hex_depth_step 
                        ))
                        .draw_iso(true)
                );
            }
        }
    }

    // Draw dijkstra map
    if DRAW_FLOW {
        let arrow_sheet = drawables.alias[textures::ARROW_SHEET];
        for r_tile in startr..=endr {
            for q_tile in startq..=endq {
                let axial = Axial::new(q_tile as i32, r_tile as i32);
                draw_arrow(&mut draw_buffer, arrow_sheet, &map, axial);
            }
        }
    }

    draw_buffer.end_command_pool();
}

pub fn draw_hex_top(map: &HexMap<HexTileData>, draw_buffer: &mut Vec<DrawCommand>, x: f32, y: f32, height: u8, texture: u64, color: Color) {
    let mut draw_command = create_draw_cmd(x, y, height as f32 * map.hex_depth_step, textures::COLOR_TINT[height as usize], texture);
    if color != Color::WHITE {
        draw_command = draw_command.color(color);
    }
    draw_buffer.push(draw_command);
}

pub fn draw_hex_brick_top(map: &HexMap<HexTileData>, draw_buffer: &mut Vec<DrawCommand>, x: f32, y: f32, height: u8, texture: u64, color: Color) {
    let mut draw_command = create_draw_cmd(x, y, height as f32 * map.hex_depth_step, textures::COLOR_TINT[height as usize], texture);
    if color != Color::WHITE {
        draw_command = draw_command.color(color);
    }
    draw_buffer.push(draw_command);
}

pub fn draw_hex_walls(map: &HexMap<HexTileData>, draw_buffer: &mut Vec<DrawCommand>, x: f32, y: f32, height: u8, wall_tex: u64) {
    let start_height = height as f32 * map.hex_depth_step - map.wall_vert_offset;
    draw_buffer.push(
        create_draw_cmd(x, y, start_height, textures::COLOR_TINT[height as usize], wall_tex)
    );
}

pub fn draw_hex_bricks(map: &HexMap<HexTileData>, draw_buffer: &mut Vec<DrawCommand>, x: f32, y: f32, height: u8, brick_tex: u64) {
    let start_height = height as f32 * map.hex_depth_step - map.wall_vert_step;
    draw_buffer.push(
        create_draw_cmd(x, y, start_height, textures::COLOR_TINT[height as usize], brick_tex)
    );
}

pub fn create_draw_cmd(x: f32, y: f32, height: f32, tint: f32, texture: u64) -> DrawCommand {
    DrawCommand::new(texture)
        .position(Vec3::new(x, y, height))
        .draw_iso(true)
        .color(Color::rgba(tint, tint, tint, 1.0))
}