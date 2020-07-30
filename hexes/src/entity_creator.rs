use vermarine_lib::{
    shipyard::*,
    hexmap::Axial,
    rendering::{
        Sprite,
        Drawables,
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