use std::any::Any;

use glam::Vec2;

use crate::jade::ecs::{
    component::{Component, ComponentContext},
    components::default_any_impl,
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
        let (x, y) = Anchor::default().to_anchor(self.anchor, parent.transform.pos, parent.transform.size);
        let (offset_x, offset_y) = self.offset;

        ctx.camera.position = Vec2::new(x as f32 + offset_x, y as f32 + offset_y)
    }

    default_any_impl!();
}
