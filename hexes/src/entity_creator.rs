use vermarine_lib::{
    shipyard::*,
    hexmap::Axial,
    rendering::{
        Sprite,
        Drawables,
        draw_buffer::{
            DrawCommand,
        },
    },
};

use crate::components::{
    Transform,
    Agent,
    Spawner,
};

use crate::map::Map;

use crate::consts::*;

pub fn create_agent(position: Axial, world: &mut AllStoragesViewMut) {
    let sprite = world.run(|drawables: NonSendSync<UniqueViewMut<Drawables>>| {
        drawables.alias[textures::ENEMY]
    });

    world.entity_builder()
        .with(Transform::new(position))
        .with(Sprite::new(sprite))
        .with(Agent::new())
        .build();
}

pub fn create_base(position: Axial, world: &mut AllStoragesViewMut) {
    let sprite = world.run(|drawables: NonSendSync<UniqueView<Drawables>>| {
        drawables.alias[textures::BASE]
    });
    
    world.entity_builder()
        .with(Transform::new(position))
        .with(Sprite::from_command(
            DrawCommand::new(sprite)
        ))
        .build();

    world.run(|mut map: UniqueViewMut<Map>| {
        let desired_height = 2;

        map.flatten_tile(position.to_hex(), desired_height);
        map.flatten_tile(position.to_hex() + Axial::new(1, 0), desired_height);
        map.flatten_tile(position.to_hex() + Axial::new(2, 0), desired_height);

        map.flatten_tile(position.to_hex() + Axial::new(0, 1), desired_height);
        map.flatten_tile(position.to_hex() + Axial::new(1, 1), desired_height);

        map.flatten_tile(position.to_hex() + Axial::new(-1, 2), desired_height);
        map.flatten_tile(position.to_hex() + Axial::new(0, 2), desired_height);
        map.flatten_tile(position.to_hex() + Axial::new(1, 2), desired_height);

        if map.terrain.tallest < desired_height {
            map.terrain.tallest = desired_height;
        }

        let goals = vec![
            position.to_hex() + Axial::new(0, 1), 
            position.to_hex() + Axial::new(1, 1),
        ];
        map.update_dijkstra(goals);
    });
}

pub fn create_nest(postion: Axial, timer: u8, world: &mut AllStoragesViewMut) {
    let sprite = world.run(|drawables: NonSendSync<UniqueView<Drawables>>| {
        drawables.alias[textures::NEST]
    });

    world.entity_builder()
        .with(Spawner::new(timer))
        .with(Transform::new(postion))
        .with(Sprite::new(sprite))
        .build();
}