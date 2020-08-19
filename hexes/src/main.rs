mod systems;
mod consts;
mod map;
mod components;
mod entity_creator;

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

            entity_creator::create_nest(Axial::new(-5, -7), 120, &mut all_storages);
            entity_creator::create_nest(Axial::new(12, -15), 120, &mut all_storages);
            entity_creator::create_nest(Axial::new(-12, -5), 120, &mut all_storages);
            entity_creator::create_nest(Axial::new(2, -8), 120, &mut all_storages);
            entity_creator::create_nest(Axial::new(-8, 6), 120, &mut all_storages);
            entity_creator::create_nest(Axial::new(-5, -15), 120, &mut all_storages);
            entity_creator::create_nest(Axial::new(11, 14), 120, &mut all_storages);
            entity_creator::create_nest(Axial::new(5, 13), 120, &mut all_storages);
            entity_creator::create_nest(Axial::new(2, 4), 120, &mut all_storages);
            entity_creator::create_nest(Axial::new(14, -3), 120, &mut all_storages);
        });

        let mut camera = Camera::with_window_size(ctx);
        camera.zoom = 1.0;
        world.add_unique(camera);

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

        self.world.run(|mut camera: UniqueViewMut<Camera>, mut draw_buff: UniqueViewMut<DrawBuffer>| {
            camera.position.floor();
            camera.update();
            draw_buff.transform_mat = camera.as_matrix();
        });

        self.world.run_with_data(DrawBuffer::flush, ctx);

        Ok(())
    }
}