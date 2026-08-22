use crate::{jade::ecs::component::Component, renderer::primitive::draw_command::DrawCommand};

pub struct Canvas
{
    draw_commands: Vec<DrawCommand>,
}

impl Canvas
{
    pub fn commands(&self) -> &[DrawCommand] { &self.draw_commands }
}

impl Component for Canvas {}
