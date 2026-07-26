use crate::{jade::scene::Scene, util::assets::assetpool::AssetPool};

pub trait WindowHandler: 'static
{
    fn initial_scene(&mut self, dims: (f32, f32), _assetpool: &AssetPool) -> Scene { Scene::new(dims) }
}
