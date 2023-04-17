use winey::window::Window;
use winey::{CursorIcon, KeyCode, WindowEvent, WindowLevel, WindowType, WineyWindowImplementation};

fn main() {
    let window = Window::new("Hello World", 500, 500);

    window.show();
    window.set_cursor_icon(CursorIcon::Arrow);
    window.set_window_type(WindowType::Utility);
    window.set_window_level(WindowLevel::Normal);

    window.run(|event,control_flow| {
        match event {
            WindowEvent::CloseRequested => {
                control_flow.exit(0);
            }

            _ => {

            }
        }
    })
}
