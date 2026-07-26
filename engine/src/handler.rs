use crate::{jade::scene::Scene, util::assets::assetpool::AssetPool};

pub trait WindowHandler: 'static
{
    fn on_start(&mut self, _scene: &mut Scene, _assetpool: &AssetPool) {}

    fn initial_scene(&mut self, dims: (f32, f32), _assetpool: &AssetPool) -> Scene
    {
        Scene::new(dims)
    }
}
