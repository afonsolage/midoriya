extern crate amethyst;

use amethyst::assets::Loader;
use amethyst::core::transform::GlobalTransform;
use amethyst::input::{is_close_requested, is_key};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, DisplayConfig, DrawFlat, Event, Material, MaterialDefaults, MeshHandle, Pipeline,
    PosTex, RenderBundle, Stage, VirtualKeyCode,
};

struct Example;

fn init_camera(world: &mut World) {
    world.create_entity().with(Camera::standard_2d()).build();
}

fn gen_vertices() -> Vec<PosTex> {
    const LEFT: f32 = 0.25;
    const RIGHT: f32 = 0.75;
    const BOTTOM: f32 = 0.25;
    const UP: f32 = 0.75;

    vec![
        PosTex {
            position: [LEFT, BOTTOM, 0.],
            tex_coord: [0.0, 0.0],
        },
        PosTex {
            position: [RIGHT, BOTTOM, 0.],
            tex_coord: [1.0, 0.0],
        },
        PosTex {
            position: [LEFT, UP, 0.],
            tex_coord: [1.0, 1.0],
        },
        PosTex {
            position: [RIGHT, UP, 0.],
            tex_coord: [1.0, 1.0],
        },
        PosTex {
            position: [LEFT, UP, 0.],
            tex_coord: [0.0, 1.0],
        },
        PosTex {
            position: [RIGHT, BOTTOM, 0.],
            tex_coord: [0.0, 0.0],
        },
    ]
}

fn gen_colors() -> [f32; 4] {
    [0.25, 0.25, 0.0, 1.]
}

fn create_mesh(world: &mut World) -> MeshHandle {
    let loader = world.read_resource::<Loader>();
    let vertices = gen_vertices();

    loader.load_from_data(vertices.into(), (), &world.read_resource())
}

fn create_material(world: &mut World) -> Material {
    let mat_defaults = world.read_resource::<MaterialDefaults>();
    let loader = world.read_resource::<Loader>();

    let albedo = loader.load_from_data(gen_colors().into(), (), &world.read_resource());

    Material {
        albedo,
        ..mat_defaults.0.clone()
    }
}

fn init_square(world: &mut World) {
    let mesh = create_mesh(world);
    let mat = create_material(world);

    world
        .create_entity()
        .with(mesh.clone())
        .with(mat.clone())
        .with(GlobalTransform::default())
        .build();
}

impl<'a, 'b> State<GameData<'a, 'b>> for Example {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;
        init_camera(world);
        init_square(world);
    }

    fn handle_event(&mut self, _: StateData<GameData>, event: Event) -> Trans<GameData<'a, 'b>> {
        if is_close_requested(&event) || is_key(&event, VirtualKeyCode::Escape) {
            Trans::Quit
        } else {
            Trans::None
        }
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        data.data.update(&data.world);
        Trans::None
    }
}

fn main() -> amethyst::Result<()> {
    let path = "resources/display_config.ron";

    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.00196, 0.23726, 0.21765, 1.0], 1.0)
            .with_pass(DrawFlat::<PosTex>::new()),
    );

    let game_data = GameDataBuilder::default().with_bundle(RenderBundle::new(pipe, Some(config)))?;
    let mut game = Application::<GameData>::new("./", Example, game_data)?;

    game.run();

    Ok(())
}
