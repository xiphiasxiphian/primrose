use engine::{util::settings::window::WindowDescriptor, window::Window};

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
