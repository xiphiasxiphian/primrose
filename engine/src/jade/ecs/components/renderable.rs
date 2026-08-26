use proc_macros::Component;

use crate::{jade::ecs::components::{transform::Transform}, renderer::{Renderable, ZIndex, mesh::Mesh, primitive::draw_command::DrawCommand}, util::assets::assetpool::TextureAsset};

#[derive(Component, Default)]
pub struct RenderInfo
{
    pub texture: Option<TextureAsset>,
    pub draw_commands: Vec<DrawCommand>,
    pub zindex: ZIndex
}

impl RenderInfo
{
}

impl<'a> Renderable for (&'a Transform, &'a RenderInfo)
{
    fn texture(&self) -> Option<&TextureAsset> {
        self.1.texture.as_ref()
    }

    fn mesh(&self) -> Mesh {
        self.0.mesh()
    }

    fn draw_commands(&self) -> &[DrawCommand] {
        &self.1.draw_commands
    }

    fn z_index(&self) -> ZIndex {
        self.1.zindex
    }
}
