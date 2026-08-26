use std::{cell::RefCell, rc::Rc, sync::Arc};

use log::info;
use wgpu::{
    CurrentSurfaceTexture, Device, DeviceDescriptor, ExperimentalFeatures, Features, Instance, Limits, MemoryHints,
    PowerPreference, Queue, RequestAdapterOptions, Surface, SurfaceColorSpace, SurfaceConfiguration, TextureUsages,
};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Fullscreen, Window as NativeWindow, WindowAttributes, WindowId},
};

use crate::{
    clock::Clock, handler::WindowHandler, jade::{
        audio::SoundHandler, ecs::{components::renderable::Renderable, system::Stage}, input::InputState, scene::manager::SceneManager,
    }, renderer::Renderer, util::{
        assets::assetpool::AssetPool,
        settings::window::{FullscreenOptions, WindowDescriptor},
    },
};

pub struct RunningState
{
    window: Arc<NativeWindow>,
    surface: Surface<'static>,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    started: bool,

    // user level
    renderer: Renderer,
    scene_manager: SceneManager,
    input: Rc<RefCell<InputState>>,
    clock: Clock,
    sound_handler: SoundHandler,
    asset_pool: AssetPool,
}

impl RunningState
{
    fn draw(&mut self)
    {
        let scene = self.scene_manager.current_scene_mut();
        scene.run_stage(Stage::PreUpdate);

        // // scene init
        // if !self.started
        // {
        //     {
        //         let input = self.input.borrow();
        //         scene.start(&mut ComponentContextIn {
        //             input: &input,
        //             assetpool: &self.asset_pool,
        //             sound: &mut self.sound_handler,
        //         });
        //     }

        //     self.started = true;
        // }

        // main tick
        let _dt = self.clock.tick();
        scene.run_stage(Stage::Update);

        let surface_frame = self.surface.get_current_texture();
        let output = match surface_frame
        {
            CurrentSurfaceTexture::Success(texture) => texture,

            CurrentSurfaceTexture::Suboptimal(texture) =>
            {
                self.surface.configure(&self.device, &self.config);
                texture
            }

            CurrentSurfaceTexture::Lost | CurrentSurfaceTexture::Outdated =>
            {
                self.surface.configure(&self.device, &self.config);
                return;
            }

            status =>
            {
                log::warn!("Dropped frame: {:?}", status);
                return;
            }
        };

        scene.run_stage(Stage::PostUpdate);

        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        self.renderer
            .draw(scene.world.query::<&Renderable>().iter(), &self.device, &self.queue, &view, &scene.camera);

        scene.run_stage(Stage::PreRender);

        self.queue.present(output);
        self.input.borrow_mut().flush();

        self.window.request_redraw(); // loop
    }
}

pub struct Window<H: WindowHandler>
{
    handler: H,
    state: Option<RunningState>,
    descriptor: WindowDescriptor,
}

impl<H: WindowHandler> Window<H>
{
    fn new(handler: H, descriptor: &WindowDescriptor) -> Self
    {
        Self {
            handler,
            state: None,
            descriptor: *descriptor,
        }
    }

    pub fn run(handler: H, descriptor: &WindowDescriptor)
    {
        let event_loop = EventLoop::new().expect("Failed to create event loop");
        event_loop
            .run_app(&mut Self::new(handler, descriptor))
            .expect("Event loop failed");
    }

    fn window_key_hooks(state: &mut RunningState, descriptor: WindowDescriptor)
    {
        let input = state.input.borrow();

        // window hooks
        if let Some(fs_options) = descriptor.fullscreen_options
        {
            if input.is_key_down(fs_options.toggle_key)
            {
                match state.window.fullscreen()
                {
                    Some(_) => state.window.set_fullscreen(None),
                    None => state.window.set_fullscreen(Some(Fullscreen::Borderless(None))),
                }
            }
            else if input.is_key_down(FullscreenOptions::DEFAULT_ESCAPE_KEY)
            {
                state.window.set_fullscreen(None);
            }
        }
    }
}

impl<H: WindowHandler> ApplicationHandler for Window<H>
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop)
    {
        if self.state.is_some()
        {
            return;
        }

        info!("Configuring window ...");

        let window = Arc::new(
            event_loop
                .create_window(
                    WindowAttributes::default()
                        .with_title(self.descriptor.title)
                        .with_inner_size(LogicalSize::new(self.descriptor.dims.0, self.descriptor.dims.1))
                        .with_fullscreen(self.descriptor.get_fullscreen()),
                )
                .expect("Failed to create window"),
        );

        info!("Succesfully created window");

        let size = window.inner_size();

        let instance = Instance::default();
        let surface = instance
            .create_surface(window.clone())
            .expect("Failed to create surface");

        let adapter = pollster::block_on(instance.request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
            apply_limit_buckets: false,
        }))
        .expect("Failed to find viable adapter");

        let (device, queue) = pollster::block_on(adapter.request_device(&DeviceDescriptor {
            label: None,
            required_features: Features::empty(),
            required_limits: Limits::default(),
            experimental_features: ExperimentalFeatures::disabled(),
            memory_hints: MemoryHints::Performance,
            trace: wgpu::Trace::Off,
        }))
        .expect("Failed to create device");

        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities.formats[0];

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
            color_space: SurfaceColorSpace::Srgb,
        };

        surface.configure(&device, &config);

        info!("Successfully configured GPU connection and surface");

        let renderer = Renderer::new(&device, surface_format);
        info!("Successfully init renderer");

        let mut asset_pool = AssetPool::preloaded(
            H::textures(),
            H::sounds(),
            device.clone(),
            queue.clone(),
            renderer.texture_bind_group_layout.clone(),
        )
        .expect("Failed to init assetpool");

        let sound_handler = SoundHandler::new().expect("Failed to init sound handler");
        let clock = Clock::new();

        let scene_manager = SceneManager::preloaded(
            self.handler.scenes(
                (self.descriptor.dims.0 as f32, self.descriptor.dims.1 as f32),
                &mut asset_pool,
            ),
            H::initial_scene(),
        )
        .expect("Failed to init scene manager");

        self.state = Some(RunningState {
            window,
            surface,
            device,
            queue,
            config,
            renderer,
            scene_manager,
            input: Rc::new(RefCell::new(InputState::new())),
            clock,
            sound_handler,
            asset_pool,
            started: false,
        })
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent)
    {
        let descriptor = self.descriptor;
        let Some(state) = &mut self.state
        else
        {
            return;
        };

        match event
        {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) =>
            {
                state.config.width = size.width;
                state.config.height = size.height;

                state.surface.configure(&state.device, &state.config);
                state
                    .scene_manager
                    .current_scene_mut()
                    .camera
                    .update_viewport((size.width as f32, size.height as f32));
            }
            WindowEvent::KeyboardInput { event, .. } => state.input.borrow_mut().handle_key_event(event),
            WindowEvent::CursorMoved { position, .. } => state.input.borrow_mut().handle_cursor_event(position),
            WindowEvent::MouseInput {
                state: button_state,
                button,
                ..
            } => state.input.borrow_mut().handle_mouse_event(button_state, button),
            WindowEvent::RedrawRequested =>
            {
                // per tick
                Self::window_key_hooks(state, descriptor);

                state.draw();
            }

            _ =>
            {}
        }
    }
}
