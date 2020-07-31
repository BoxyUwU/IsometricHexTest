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
    tetra::{
        math::{
            Vec3,
        },
    },
};

use crate::components::{
    Transform,
    Agent,
};
use crate::consts::*;

pub fn create_agent(position: Axial, world: &mut AllStoragesViewMut) {
    let sprite = world.run(|drawables: NonSendSync<UniqueViewMut<Drawables>>| {
        drawables.alias[textures::MARKER]
    });

    world.entity_builder()
        .with(Transform::new(position))
        .with(Sprite::new(sprite))
        .with(Agent::new())
        .build();
}

pub fn create_base(position: Axial, world: &mut AllStoragesViewMut) {
    let sprite = world.run(|drawables: NonSendSync<UniqueViewMut<Drawables>>| {
        drawables.alias[textures::BASE]
    });
    
    world.entity_builder()
        .with(Transform::new(position))
        .with(Sprite::from_command(
            DrawCommand::new(sprite)
            .position(Vec3::new(0., -12., 0.))
        ))
        .build();
}