use winey::platform::{WindowCorner, WindowExtForWindows};
use winey::window::Window;
use winey::{WindowEvent, WineyWindowImplementation};

fn main() {
    let window = Window::new("Hello World", 500, 500);
    window.set_window_corner_radius(WindowCorner::DoNotRound);
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
