use std::{cell::RefCell, rc::Rc, sync::Arc};

use wgpu::{
    Device, DeviceDescriptor, Features, Instance, Limits, PowerPreference, Queue, RequestAdapterOptions, Surface,
    SurfaceConfiguration, TextureUsages,
};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window as NativeWindow, WindowAttributes, WindowId},
};

use crate::{
    clock::Clock,
    jade::{
        audio::SoundHandler,
        ecs::{
            components::{basic_controller::PlayerController, camera::camera_lock::CameraLock},
            object::Object,
            transform::{Anchor, Transform},
        },
        input::InputState,
        scene::{ComponentContextIn, Scene},
    },
    renderer::Renderer,
    util::assets::{
        self,
        assetpool::AssetPool,
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
    scene: Scene,
    input: Rc<RefCell<InputState>>,
    clock: Clock,
    sound_handler: SoundHandler,
    asset_pool: AssetPool,
}

pub struct Window
{
    state: Option<RunningState>,
}

impl Window
{
    const DEFAULT_DIMS: (u32, u32) = (1024, 768);

    fn new() -> Self { Self { state: None } }

    pub fn run()
    {
        let event_loop = EventLoop::new().expect("Failed to create event loop");
        event_loop.run_app(&mut Self::new()).expect("Event loop failed");
    }

    fn on_start(state: &mut RunningState)
    {
        let texture = state.asset_pool.get_texture("grass").unwrap();
        state.scene.add(
            Object::new(
                "grass",
                Transform::with_anchor((0.0, 0.0), (200.0, 200.0), Anchor::Center),
            )
            .with_texture(texture.clone())
            .with_z_index(1)
            .with_component(PlayerController { speed: 100.0 })
            .with_component(CameraLock::default()),
        );

        state.scene.add(
            Object::new(
                "grass2",
                Transform {
                    pos: (200.0, 200.0),
                    size: (100.0, 100.0),
                },
            )
            .with_texture(texture),
        );
    }

    fn draw(state: &mut RunningState)
    {
        // scene init
        if !state.started
        {
            {
                let input = state.input.borrow();
                state.scene.start(&mut ComponentContextIn {
                    input: &input,
                    assetpool: &state.asset_pool,
                    sound: &mut state.sound_handler,
                });
            }

            Self::on_start(state);
            state.started = true;
        }

        // main tick
        let dt = state.clock.tick();

        let output = match state.surface.get_current_texture()
        {
            Ok(texture) => texture,
            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) =>
            {
                state.surface.configure(&state.device, &state.config);
                return;
            }
            Err(e) =>
            {
                log::warn!("Dropped frame: {:?}", e);
                return;
            }
        };

        {
            let input = state.input.borrow();
            state.scene.tick(
                &mut ComponentContextIn {
                    input: &input,
                    assetpool: &state.asset_pool,
                    sound: &mut state.sound_handler,
                },
                dt,
            );
        }

        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        state.renderer.draw(
            state.scene.objects(),
            &state.device,
            &state.queue,
            &view,
            &state.scene.camera,
        );

        output.present();
        state.input.borrow_mut().flush();

        state.window.request_redraw(); // loop
    }
}

impl ApplicationHandler for Window
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop)
    {
        if self.state.is_some()
        {
            return;
        }

        let window = Arc::new(
            event_loop
                .create_window(
                    WindowAttributes::default()
                        .with_title("Primrose")
                        .with_inner_size(LogicalSize::new(Self::DEFAULT_DIMS.0, Self::DEFAULT_DIMS.1)),
                )
                .expect("Failed to create window"),
        );

        let size = window.inner_size();

        let instance = Instance::default();
        let surface = instance
            .create_surface(window.clone())
            .expect("Failed to create surface");

        let adapter = pollster::block_on(instance.request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .expect("Failed to find viable adapter");

        let (device, queue) = pollster::block_on(adapter.request_device(
            &DeviceDescriptor {
                label: None,
                required_features: Features::empty(),
                required_limits: Limits::default(),
            },
            None,
        ))
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
        };

        surface.configure(&device, &config);

        let renderer = Renderer::new(&device, surface_format);
        let asset_pool = AssetPool::preloaded(
            assets::TEXTURES,
            assets::SOUNDS,
            &device,
            &queue,
            &renderer.texture_bind_group_layout,
        )
        .expect("Failed to init assetpool");

        let sound_handler = SoundHandler::new().expect("Failed to init sound handler");
        let clock = Clock::new();

        self.state = Some(RunningState {
            window,
            surface,
            device,
            queue,
            config,
            renderer,
            scene: Scene::new((size.width as f32, size.height as f32)),
            input: Rc::new(RefCell::new(InputState::new())),
            clock,
            sound_handler,
            asset_pool,
            started: false,
        })
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent)
    {
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
                    .scene
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
            WindowEvent::RedrawRequested => Self::draw(state),

            _ =>
            {}
        }
    }
}
