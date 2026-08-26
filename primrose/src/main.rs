mod double_pendulum;

use std::iter;

use engine::{
    glam::DVec2,
    handler::WindowHandler,
    jade::{
        scene::{Scene, manager::ManagedScene},
    },
    util::{
        assets::{ManagedResource, assetpool::AssetPool},
        settings::window::WindowDescriptor,
    },
    window::Window,
};

struct Handler;

impl WindowHandler for Handler
{
    fn textures() -> impl IntoIterator<Item = (&'static str, ManagedResource<&'static [u8]>)>
    where
        Self: Sized,
    {
        [(
            "grass",
            ManagedResource::eager(&include_bytes!("../assets/images/grass.png")[..]),
        )]
    }

    fn sounds() -> impl IntoIterator<Item = (&'static str, ManagedResource<&'static [u8]>)>
    where
        Self: Sized,
    {
        []
    }

    fn scenes(
        &mut self,
        dims: (f32, f32),
        assetpool: &mut AssetPool,
    ) -> impl IntoIterator<Item = (&'static str, ManagedScene)>
    {
        let texture = assetpool.get_texture("grass").unwrap();
        [
            (
                "double_pendulum",
                ManagedScene::eager(
                    Scene::new(dims)
                )
            )
        ]
    }

    fn initial_scene() -> &'static str { "double_pendulum" }
}

fn main()
{
    env_logger::init();
    Window::run(
        Handler,
        &WindowDescriptor {
            title: "Primrose",
            ..Default::default()
        },
    );
}
