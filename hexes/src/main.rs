mod systems;
mod consts;

use consts::*;

use systems::{
    render_hex_map
};

use vermarine_lib::{
    rendering::{
        RenderingWorkloadCreator,
        RenderingWorkloadSystems,
        draw_buffer::{
            DrawBuffer,
            DrawCommand,
        },
        Drawables,
        Sprite,
    },
    components::{
        Transform,
    },
    tetra::{
        self,
        ContextBuilder,
        State,
        Context,
        Trans,
        graphics::{
            self,
            Color,
        },
    },
    shipyard::{
        self,
        *,
    },
};

pub struct Res {
    drawables: Drawables,
}

impl Res {
    pub fn new(ctx: &mut Context) -> tetra::Result<Self> {
        Ok(Res {
            drawables: Drawables::new(ctx)?,
        })
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Hexes", 1280, 720)
        .show_mouse(true)
        .build()?
        .run(Game::new, Res::new)
}

pub struct Game {
    world: World,
}

impl Game {
    pub fn new(ctx: &mut Context) -> tetra::Result<Self> {
        let mut world = World::new();

        world
            .add_rendering_workload(ctx)
            .with_rendering_systems()
            .with_system(system!(render_hex_map))
            .build();

        world.run(|
            mut entities: EntitiesViewMut,
            mut sprites: ViewMut<Sprite>,
            mut transforms: ViewMut<Transform>, | {
                entities.add_entity(
                    (
                        &mut sprites, 
                        &mut transforms,
                    ),
                    (
                        Sprite::from_command(
                            DrawCommand::new(textures::FLOOR)
                        ),
                        Transform::new(0.0, 0.0),
                    ),
                );
        });

        Ok(Game {
            world,
        })
    }
}

impl State<Res> for Game {
    fn update(&mut self, _ctx: &mut Context, _resources: &mut Res) -> tetra::Result<Trans<Res>> {        
        Ok(Trans::None)
    }

    fn draw(&mut self, ctx: &mut Context, res: &mut Res) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        self.world.run_workload("Rendering");
        self.world.run_with_data(DrawBuffer::flush, (ctx, &res.drawables));
        
        Ok(())
    }
}