use winey::window::Window;
use winey::{WindowEvent, WineyWindowImplementation};
use winey::keyboard::{get_key_name, get_key_state};

fn main() {
    let window = Window::new("Hello World", 500, 500);

    window.show();

    window.run(|event, control_flow| match event {
        WindowEvent::CloseRequested => {
            std::process::exit(0);
        }

        WindowEvent::KeyEvent(code) => {
            println!("Name: {} State: {:?}",get_key_name(code),get_key_state(code))
        }

        _ => {}
    })
}
