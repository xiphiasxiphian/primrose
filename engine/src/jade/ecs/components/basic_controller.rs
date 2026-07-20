use std::any::Any;

use crate::jade::{
    ecs::{
        component::{Component, ComponentContext},
        components::default_any_impl,
        object::Object,
    },
    input::key::Key,
};

pub struct PlayerController
{
    pub speed: f64,
}

impl Component for PlayerController
{
    fn tick(&mut self, parent: &mut Object, ctx: &mut ComponentContext, dt: f64)
    {
        let input = ctx.input;

        if input.is_key_held(Key::A)
        {
            parent.transform.pos.0 -= self.speed * dt;
        }
        if input.is_key_held(Key::D)
        {
            parent.transform.pos.0 += self.speed * dt;
        }
        if input.is_key_held(Key::W)
        {
            parent.transform.pos.1 -= self.speed * dt;
        }
        if input.is_key_held(Key::S)
        {
            parent.transform.pos.1 += self.speed * dt;
        }
    }

    default_any_impl!();
}
