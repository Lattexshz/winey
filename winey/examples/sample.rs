use winey::window::Window;
use winey::{WindowEvent, WineyWindowImplementation};

fn main() {
    let window = Window::new("Hello World", 500, 500);

    window.show();

    window.run(|event, control_flow| match event {
        WindowEvent::CloseRequested => {
            control_flow.exit(0);
        }

        _ => {}
    })
}
