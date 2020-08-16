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