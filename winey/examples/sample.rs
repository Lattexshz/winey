use winey::keyboard::{get_key_name, get_key_state};
use winey::window::Window;
use winey::{WindowEvent, WineyWindowImplementation};

fn main() {
    let window = Window::new("Hello World", 500, 500);

    window.set_visible(true);
    window.set_resizable(false);

    window.run(|event, _control_flow| match event {
        WindowEvent::CloseRequested => {
            std::process::exit(0);
        }

        WindowEvent::KeyEvent(code) => {
            println!(
                "Name: {} State: {:?}",
                get_key_name(code),
                get_key_state(code)
            )
        }

        _ => {}
    });
}
