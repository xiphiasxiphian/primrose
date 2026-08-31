use std::{
    cell::RefCell,
    collections::{HashMap, hash_map::Entry},
    rc::Rc,
};


use crate::{
    clock::Clock,
    jade::{
        audio::SoundHandler,
        ecs::{
            components::renderable::RenderInfo, query::Query, system::scheduler::Stage,
            world::World,
        },
        input::InputState,
        scene::Scene,
    },
    util::assets::{ManagedResource, assetpool::AssetPool},
};

pub type ManagedScene = ManagedResource<Scene>;

pub struct GlobalResources
{
    pub dims: (u32, u32),
    pub clock: Clock,
    pub assetpool: AssetPool,
    pub input: Rc<RefCell<InputState>>,
    pub sound_handler: SoundHandler,
}

pub struct SceneManager
{
    scenes: HashMap<&'static str, ManagedScene>,
    current: &'static str,
    global_resources: GlobalResources,
}

impl SceneManager
{
    pub fn preloaded<F>(scenes_generator: F, initial_scene: &'static str, mut globals: GlobalResources) -> Option<Self>
    where
        F: FnOnce((f32, f32), &mut AssetPool) -> HashMap<&'static str, ManagedScene>,
    {
        let scenes = scenes_generator((globals.dims.0 as f32, globals.dims.1 as f32), &mut globals.assetpool);
        if !scenes.contains_key(initial_scene)
        {
            return None;
        }

        let mut manager = Self {
            scenes: scenes,
            current: initial_scene,
            global_resources: globals,
        };
        manager.with_current_scene(|x| Self::init_scene(x));

        Some(manager)
    }

    pub fn add_scene(&mut self, name: &'static str, scene: ManagedScene) -> bool
    {
        match self.scenes.entry(name)
        {
            Entry::Occupied(_) => false,
            Entry::Vacant(e) =>
            {
                e.insert_entry(scene);
                true
            }
        }
    }

    pub fn init_scene(scene: &mut Scene)
    {
        fn tick_clock(_: &mut World, g: &mut GlobalResources) { g.clock.tick(); }
        scene.add_system(Stage::PreUpdate, tick_clock);

        fn clear_render(q: Query<&mut RenderInfo>, _: &mut GlobalResources)
        {
            q.iter().for_each(|x| x.draw_commands.clear());
        }
        scene.add_system(Stage::PreUpdate, clear_render);
    }

    pub fn run_stage(&mut self, stage: Stage)
    {
        let globals = &mut self.global_resources;
        let scene = self
            .scenes
            .get_mut(&self.current)
            .expect("Current scene set to invalid scene. This should be impossible")
            .get();

        scene.scheduler.run_stage(stage, &mut scene.world, globals);
    }

    pub fn with_current_scene<F, T>(&mut self, func: F) -> T
    where
        F: FnOnce(&mut Scene) -> T,
    {
        func(self.current_scene_mut())
    }

    pub fn current_scene(&mut self) -> &Scene
    {
        self.scenes
            .get_mut(&self.current)
            .expect("Current scene set to invalid scene. This should be impossible")
            .get()
    }

    pub fn current_scene_mut(&mut self) -> &mut Scene
    {
        self.scenes
            .get_mut(&self.current)
            .expect("Current scene set to invalid scene. This should be impossible")
            .get()
    }

    pub fn switch(&mut self, target: &'static str) -> Option<&mut Scene>
    {
        let result = self.scenes.get_mut(target)?.get();
        self.current = target;

        if !result.init
        {
            Self::init_scene(result);
        }

        Some(result)
    }

    pub fn switch_with<F>(&mut self, target: &'static str, f: F) -> bool
    where
        F: FnOnce(&mut Scene),
    {
        let Some(new_scene) = self.scenes.get_mut(target)
        else
        {
            return false;
        };

        let result = new_scene.get();
        if !result.init
        {
            Self::init_scene(result);
        }

        f(result);

        true
    }
}
