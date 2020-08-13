mod systems;
mod consts;
mod map;
mod components;
mod entity_creator;

use components::{
    Spawner,
    Transform,
};

use map::{
    Map,
};

use vermarine_lib::{
    rendering::{
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
    hexmap::{
        Axial,
    },
};

fn main() -> tetra::Result {
    ContextBuilder::new("Hexes", 1280, 720)
        .show_mouse(true)
        .resizable(true)
        .timestep(tetra::time::Timestep::Variable)
        .vsync(true)
        .build()?
        .run(Game::new)
}

pub struct Game {
    world: World,
}

impl Game {
    pub fn new(ctx: &mut Context) -> tetra::Result<Self> {
        let world = World::new();

        world.add_unique(Map::new());
        world.add_unique((*ctx.input_context()).clone());
        world.add_unique_non_send_sync(Drawables::new(ctx).unwrap());

        world.run(|mut all_storages| {
            entity_creator::create_base(Axial::new(10, 5), &mut all_storages);
        });

        world.entity_builder()
            .with(Spawner::new(120))
            .with(Transform::new(Axial::new(-5, -7)))
            .build();

        world.add_unique(Camera::with_window_size(ctx));
        world.add_unique(DrawBuffer::new());

        Ok(Game {
            world,
        })
    }
}

impl State for Game {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        let input_ctx = (*ctx.input_context()).clone();
        self.world.run(|mut ctx: UniqueViewMut<InputContext>| {
            *ctx = input_ctx;
        });

        self.world.run(systems::move_camera);
        self.world.run(systems::update_hex_map);
        self.world.run(systems::move_agents);
        self.world.run(systems::spawn_agents);
            
        Ok(())
    }

    fn event(&mut self, _: &mut Context, event: tetra::Event) -> tetra::Result {
        match event {
            tetra::Event::Resized { width, height } => {
                self.world.run(|mut camera: UniqueViewMut<Camera>| {
                    camera.set_viewport_size((width & !1) as f32, (height & !1) as f32);
                    camera.update();
                });
            },
            _ => { },
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.4, 0.6, 0.9));

        self.world.run(systems::draw_hex_map);
        self.world.run(systems::draw_agent_paths);
        self.world.run(systems::draw_entities);

        self.world.run(|mut camera: UniqueViewMut<Camera>, mut draw_buff: UniqueViewMut<DrawBuffer>| {
            camera.position.floor();
            camera.update();
            draw_buff.transform_mat = camera.as_matrix();
        });

        self.world.run_with_data(DrawBuffer::flush, ctx);

        Ok(())
    }
}