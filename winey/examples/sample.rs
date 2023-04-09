use winey::window::Window;
use winey::{WindowEvent, WineyWindowImplementation};
use winey::platform::WindowExtForWindows;

fn main() {
    let window = Window::new("Hello World", 500, 500);

    window.show();

    window.run(|event| {
        match event {
            WindowEvent::CloseRequested => {
                std::process::exit(0);
            }

            _ => {

            }
        }
    })
}
