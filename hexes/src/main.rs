mod systems;
#[allow(dead_code)]
mod consts;
mod map;

use consts::{
    *,
};

use vermarine_lib::{
    rendering::{
        RenderingWorkloadCreator,
        RenderingWorkloadSystems,
        Drawables,
        draw_buffer::{
            DrawBuffer,
        },
    },
    tetra::{
        self,
        ContextBuilder,
        State,
        Context,
        graphics::{
            Camera,
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

fn main() -> tetra::Result {
    ContextBuilder::new("Hexes", 1280, 720)
        .show_mouse(true)
        .build()?
        .run(Game::new)
}

pub struct Game {
    world: World,
}

impl Game {
    pub fn new(ctx: &mut Context) -> tetra::Result<Self> {
        let mut world = World::new();

        let hex_width = 36.;
        let hex_height = 36.;
        let hex_vert_step = 28.;
        let hex_depth_step = 12.;

        let wall_vert_offset = 12.;
        let wall_vert_step = 12.;

        world.add_unique(map::HexMap::new(
            WIDTH, HEIGHT,
            
            hex_width,
            hex_height,
            hex_vert_step,
            hex_depth_step,

            wall_vert_offset,
            wall_vert_step,

            MAX_FLOOR_HEIGHT,
        ));
        world.add_unique((*ctx.input_context()).clone());
        world.add_unique_non_send_sync(Drawables::new(ctx).unwrap());

        world
            .add_rendering_workload(ctx)
            .with_rendering_systems()
            .with_system(system!(systems::render_hex_map))
            .build();

        Ok(Game {
            world,
        })
    }
}

impl State for Game {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        let input_ctx = ctx.input_context();
        self.world.run(|mut ctx: UniqueViewMut<InputContext>| {
            *ctx = (*input_ctx).clone();
        });

        self.world.run(systems::move_camera);
        self.world.run(systems::update_hex_map);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        self.world.run_workload("Rendering");
        self.world.run(|mut camera: UniqueViewMut<Camera>, mut draw_buff: UniqueViewMut<DrawBuffer>| {
            camera.update();
            draw_buff.transform_mat = camera.as_matrix();
        });
        self.world.run_with_data(DrawBuffer::flush, ctx);

        Ok(())
    }
}