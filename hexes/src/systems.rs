use crate::{
    shipyard::{
        *,
    },
    tetra::{
        InputContext,
        graphics::{
            Camera,
        },
        input::{
            self,
            Key,
        },
        math::{
            Vec2,
        },
    },
    consts::{
        *,
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