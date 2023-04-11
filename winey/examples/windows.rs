use winey::window::Window;
use winey::{KeyCode, WindowEvent, WindowRect, WineyWindowImplementation};
use winey::platform::WindowExtForWindows;

fn main() {
    let window = Window::new("Hello World", 500, 500);

    window.set_window_border_color(255,0,0);
    window.extend_frame_into_client_area(WindowRect {
        left_width: 40,
        right_width: 40,
        top_height: 40,
        bottom_height: 40,
    });

    window.show();

    let mut r = 0;
    let mut g = 255;
    let mut b = 0;

    let mut increment:i16 = 1;

    println!("{}",window.get_title());

    window.run(|event,control_flow| {
        match event {
            WindowEvent::Update => {
                if r == 0 {
                    increment = 1;
                }

                if r == 255 {
                    increment = -1;
                }

                r += increment;
                b += increment;

                window.set_window_caption_color(r as u8,g as u8,b as u8);
                window.set_window_border_color(r as u8,g as u8,b as u8);
            }

            WindowEvent::CloseRequested => {
                control_flow.exit(0);
            }

            _ => {

            }
        }
    })
}
