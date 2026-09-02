mod double_pendulum;

use std::collections::HashMap;

use engine::{
    glam::DVec2,
    handler::WindowHandler,
    jade::{
        ecs::{
            components::{renderable::RenderInfo, transform::Transform},
            system::scheduler::Stage,
        },
        scene::{Scene, manager::ManagedScene},
    },
    util::{
        assets::{ManagedResource, assetpool::AssetPool},
        settings::window::WindowDescriptor,
    },
    window::Window,
};

use crate::double_pendulum::{DoublePendulum, double_pendulum_system};

struct Handler;

impl WindowHandler for Handler
{
    fn textures() -> impl IntoIterator<Item = (&'static str, ManagedResource<&'static [u8]>)>
    where
        Self: Sized,
    {
        []
    }

    fn sounds() -> impl IntoIterator<Item = (&'static str, ManagedResource<&'static [u8]>)>
    where
        Self: Sized,
    {
        []
    }

    fn scenes(&mut self, dims: (f32, f32), _assetpool: &mut AssetPool) -> HashMap<&'static str, ManagedScene>
    {
        [(
            "double_pendulum",
            ManagedScene::eager(
                Scene::new(dims)
                    .with_entity(|x| {
                        x.with_component(Transform::new(DVec2::new(0.0, -200.0), DVec2::default()))
                            .with_component(RenderInfo::default())
                            .with_component(DoublePendulum::default())
                    })
                    // .with_entity(|x| {
                    //     x.with_component(Transform::new(DVec2::new(0.0, -200.0), DVec2::default()))
                    //         .with_component(RenderInfo::default())
                    //         .with_component(DoublePendulum::default().with_theta1(89.99_f64.to_radians()))
                    // })
                    .with_system(Stage::Update, double_pendulum_system),
            ),
        )]
        .into()
    }

    fn initial_scene() -> &'static str { "double_pendulum" }
}

fn main()
{
    env_logger::init();
    Window::run(Handler, &WindowDescriptor::default().with_title("Primrose"));
}
