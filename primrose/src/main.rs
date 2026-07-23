use engine::window::{Window, WindowDescriptor};

fn main()
{
    env_logger::init();
    Window::run(
        &WindowDescriptor {
            title: "Primrose",
            ..Default::default()
        }
    );
}
