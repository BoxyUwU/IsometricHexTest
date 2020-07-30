use vermarine_lib::hexmap::*;

#[derive(Copy, Clone, Debug)]
pub struct Transform {
    pub position: Axial,
}

impl Transform {
    pub fn new(position: Axial) -> Self {
        Transform {
            position,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Agent {
}

impl Agent {
    pub fn new() -> Self {
        Agent {}
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Spawner {
    pub period: u8,
    pub counter: u8,
}

impl Spawner {
    pub fn new(period: u8) -> Self {
        Self {
            period,
            counter: 1,
        }
    }
}