use winey::platform::{WindowCorner, WindowExtForWindows};
use winey::window::Window;
use winey::{Cursor, CursorIcon, WindowEvent, WineyWindowImplementation};
use winey::keyboard::*;

// Note Some of the methods used in this sample may not work properly without Windows 11.

fn main() {
    let window = Window::new("Hello World", 500, 500);

    window.set_window_border_color(255, 0, 0);
    window.set_window_corner_radius(WindowCorner::DoNotRound);
    window.set_window_caption_color(0, 0, 0);
    window.set_window_text_color(255, 255, 255);

    let cursor = Cursor {
        icon: CursorIcon::Wait,
        x: 50,
        y: 80,
    };

    window.set_cursor(cursor);
    window.show();

    window.run(|event, control_flow| match event {
        WindowEvent::CloseRequested => {
            control_flow.exit(0);
        }

        WindowEvent::KeyEvent(code) => {
            println!("{}",code);
            if code == 131 {
                control_flow.exit(0);
            }
        }

        _ => {}
    })
}
