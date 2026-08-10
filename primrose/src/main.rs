use std::iter;

use engine::{
    glam::DVec2, handler::WindowHandler, jade::{
        ecs::{
            components::{basic_controller::PlayerController, camera::camera_lock::CameraLock},
            object::Object,
            transform::{Anchor, Transform},
        },
        scene::{Scene, manager::ManagedScene},
    }, util::{assets::assetpool::AssetPool, settings::window::WindowDescriptor}, window::Window,
};

struct Handler;

impl WindowHandler for Handler
{
    fn textures() -> &'static [(&'static str, &'static [u8])]
    where
        Self: Sized,
    {
        &[("grass", include_bytes!("../assets/images/grass.png"))]
    }

    fn sounds() -> &'static [(&'static str, &'static [u8])]
    where
        Self: Sized,
    {
        &[]
    }

    fn scenes(
        &mut self,
        dims: (f32, f32),
        assetpool: &AssetPool,
    ) -> impl IntoIterator<Item = (&'static str, ManagedScene)>
    {
        let texture = assetpool.get_texture("grass").unwrap();
        iter::once_with(move || (
            "base",
            ManagedScene::eager(
                Scene::new(dims)
                    .with_object(
                        Object::new(
                            "grass",
                            Transform::with_anchor(DVec2::ZERO, DVec2::splat(200.0), Anchor::Center),
                        )
                        .with_texture(texture.clone())
                        .with_z_index(1)
                        .with_component(PlayerController { speed: 200.0 })
                        .with_component(CameraLock::default()),
                    )
                    .with_object(
                        Object::new(
                            "grass2",
                            Transform {
                                pos: (200.0, 200.0).into(),
                                size: (100.0, 100.0).into(),
                            },
                        )
                        .with_texture(texture),
                    ),
            ),
        ))
    }

    fn initial_scene() -> &'static str { "base" }
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
