use glam::Vec2;

use crate::jade::ecs::{
    component::{Component, ComponentContext},
    object::Object,
    transform::Anchor,
};

#[derive(Clone, Copy, Debug)]
pub struct CameraLock
{
    anchor: Anchor,
    offset: (f32, f32),
}

impl Default for CameraLock
{
    fn default() -> Self
    {
        Self {
            anchor: Anchor::Center,
            offset: Default::default(),
        }
    }
}

impl Component for CameraLock
{
    fn tick(&mut self, parent: &mut Object, ctx: &mut ComponentContext, _dt: f64)
    {
        let pos = Anchor::default().to_anchor(self.anchor, parent.transform.pos, parent.transform.size);
        ctx.camera.position = pos.as_vec2() + Vec2::from(self.offset)
    }
}
