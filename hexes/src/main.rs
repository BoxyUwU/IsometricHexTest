mod systems;
#[allow(dead_code)]
mod consts;
mod map;

use map::{
    render_hex_map,
};

use vermarine_lib::{
    rendering::{
        RenderingWorkloadCreator,
        RenderingWorkloadSystems,
        draw_buffer::{
            DrawBuffer,
        },
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
        input::{
            InputContext,
        },
    },
    shipyard::{
        self,
        *,
    },
};

type Res = ();

fn main() -> tetra::Result {
    ContextBuilder::new("Hexes", 1280, 720)
        .show_mouse(true)
        .build()?
        .run(Game::new, |_| Ok(()))
}

pub struct Game {
    world: World,
}

impl Game {
    pub fn new(ctx: &mut Context) -> tetra::Result<Self> {
        let mut world = World::new();

        world.add_unique(map::Map::new(100, 100));
        world.add_unique((*ctx.input_context()).clone());

        world
            .add_rendering_workload(ctx)
            .with_rendering_systems()
            .with_system(system!(render_hex_map))
            .build();

        Ok(Game {
            world,
        })
    }
}

impl State<Res> for Game {
    fn update(&mut self, ctx: &mut Context, _res: &mut Res) -> tetra::Result<Trans<Res>> {
        let input_ctx = ctx.input_context();
        self.world.run(|mut ctx: UniqueViewMut<InputContext>| {
            *ctx = (*input_ctx).clone();
        });

        self.world.run(systems::move_camera);

        Ok(Trans::None)
    }

    fn draw(&mut self, ctx: &mut Context, _res: &mut Res) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        self.world.run_workload("Rendering");
        self.world.run_with_data(DrawBuffer::flush, ctx);
        
        Ok(())
    }
}