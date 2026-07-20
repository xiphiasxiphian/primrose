use std::mem;

use crate::{
    jade::ecs::{
        component::{Component, ComponentContext},
        transform::Transform,
    },
    renderer::{Renderable, ZIndex, mesh::Mesh},
    util::assets::assetpool::TextureAsset,
};

#[derive(Default)]
pub struct Object
{
    pub name: String,
    pub transform: Transform,
    z_index: ZIndex,
    components: Vec<Box<dyn Component>>,
    started: bool,
    texture: Option<TextureAsset>,
}

impl Object
{
    pub fn new(name: &str, transform: Transform) -> Self
    {
        Self {
            name: name.to_string(),
            transform,
            ..Default::default()
        }
    }

    pub fn with_texture(mut self, texture: TextureAsset) -> Self
    {
        self.texture = Some(texture);
        self
    }

    pub fn with_z_index(mut self, z_index: i32) -> Self
    {
        self.z_index = z_index;
        self
    }

    pub fn with_component<C: Component>(mut self, component: C) -> Self
    {
        self.components.push(Box::new(component));
        self
    }

    pub fn with_components<I>(mut self, components: I) -> Self
    where
        I: IntoIterator<Item = Box<dyn Component>>,
    {
        self.components.extend(components);
        self
    }

    pub fn add_component<C: Component>(&mut self, component: C) -> &mut Self
    {
        self.components.push(Box::new(component));
        self
    }

    pub fn get_component<C: Component>(&self) -> Option<&C>
    {
        self.components.iter().find_map(|x| x.as_any().downcast_ref::<C>())
    }

    pub fn get_component_mut<C: Component>(&mut self) -> Option<&mut C>
    {
        self.components
            .iter_mut()
            .find_map(|x| x.as_any_mut().downcast_mut::<C>())
    }

    pub fn has_component<C: Component>(&self) -> bool { self.get_component::<C>().is_some() }

    pub fn start(&mut self, ctx: &mut ComponentContext)
    {
        if self.started
        {
            return;
        }

        let mut components = mem::take(&mut self.components);
        for component in &mut components
        {
            component.start(self, ctx);
        }

        self.components = components;
        self.started = true;
    }

    pub fn tick(&mut self, ctx: &mut ComponentContext, dt: f64)
    {
        let mut components = mem::take(&mut self.components);
        for component in &mut components
        {
            component.tick(self, ctx, dt);
        }

        self.components = components;
    }
}

impl Renderable for Object
{
    fn texture(&self) -> Option<&TextureAsset> { self.texture.as_ref() }

    fn mesh(&self) -> Mesh
    {
        Mesh::quad(
            self.transform.pos.0 as f32,
            self.transform.pos.1 as f32,
            self.transform.size.0 as f32,
            self.transform.size.1 as f32,
        )
    }

    fn z_index(&self) -> ZIndex { self.z_index }
}
