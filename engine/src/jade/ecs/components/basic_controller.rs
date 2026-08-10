use crate::jade::{
    ecs::{
        component::{Component, ComponentContext},
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
            parent.transform.pos.x -= self.speed * dt;
        }
        if input.is_key_held(Key::D)
        {
            parent.transform.pos.x += self.speed * dt;
        }
        if input.is_key_held(Key::W)
        {
            parent.transform.pos.y -= self.speed * dt;
        }
        if input.is_key_held(Key::S)
        {
            parent.transform.pos.y += self.speed * dt;
        }
    }
}
