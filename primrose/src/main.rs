mod double_pendulum;

use std::iter;

use engine::{
    glam::DVec2,
    handler::WindowHandler,
    jade::{
        ecs::{
            components::{basic_controller::PlayerController, camera::camera_lock::CameraLock},
            object::Object,
            transform::{Anchor, Transform},
        },
        scene::{Scene, manager::ManagedScene},
    },
    util::{
        assets::{ManagedResource, assetpool::AssetPool},
        settings::window::WindowDescriptor,
    },
    window::Window,
};

use crate::double_pendulum::DoublePendulum;

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
            ),
            (
                "double_pendulum",
                ManagedScene::eager(
                    Scene::new(dims)
                        .with_object(
                            Object::new(
                                "double_pendulum",
                                Transform::default(),
                            )
                            .with_component(DoublePendulum::default())
                        )
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
