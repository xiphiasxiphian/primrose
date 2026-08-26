use proc_macros::Component;

use crate::{jade::ecs::components::Component, renderer::{ZIndex, mesh::Mesh, primitive::draw_command::DrawCommand}, util::assets::assetpool::TextureAsset};

#[derive(Component)]
pub struct Renderable
{
    pub texture: Option<TextureAsset>,
    pub mesh: Mesh,
    pub draw_commands: Vec<DrawCommand>,
    pub zindex: ZIndex
}

impl Renderable
{
}
