use engine::{
    handler::WindowHandler,
    jade::{
        ecs::{
            components::{basic_controller::PlayerController, camera::camera_lock::CameraLock},
            object::Object,
            transform::{Anchor, Transform},
        },
        scene::Scene,
    },
    util::{assets::assetpool::AssetPool, settings::window::WindowDescriptor},
    window::Window,
};

struct Handler;

impl WindowHandler for Handler
{
    fn textures() -> &'static [(&'static str, &'static [u8])]
    where Self: Sized
    {
        &[
            ("grass", include_bytes!("../assets/images/grass.png"))
        ]
    }

    fn sounds() -> &'static [(&'static str, &'static [u8])]
    where Self: Sized
    {
        &[]
    }

    fn initial_scene(&mut self, dims: (f32, f32), assetpool: &AssetPool) -> Scene
    {
        let texture = assetpool.get_texture("grass").unwrap();
        Scene::new(dims)
            .with_object(
                Object::new(
                    "grass",
                    Transform::with_anchor((0.0, 0.0), (200.0, 200.0), Anchor::Center),
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
                        pos: (200.0, 200.0),
                        size: (100.0, 100.0),
                    },
                )
                .with_texture(texture),
            )
    }
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
