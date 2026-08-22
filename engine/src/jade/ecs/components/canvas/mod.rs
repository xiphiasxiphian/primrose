use glam::DVec2;

use crate::{jade::ecs::component::Component, renderer::primitive::draw_command::{Color, DrawCommand}};

pub struct Canvas
{
    draw_commands: Vec<DrawCommand>,
}

impl Canvas
{
    pub fn commands(&self) -> &[DrawCommand] { &self.draw_commands }


    // temp for now. gonna make the api a whole lot nicer to use in the future
    pub fn line(&mut self, start: DVec2, end: DVec2, thickness: f64, color: Color)
    {
        self.draw_commands.push(DrawCommand::Line { start, end, thickness, color });
    }

    pub fn circle(&mut self, center: DVec2, radius: f64, color: Color, segments: u32)
    {
        self.draw_commands.push(DrawCommand::Circle { center, radius, color, segments });
    }
}

impl Component for Canvas {}
