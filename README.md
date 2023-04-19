# Winey - Windowing library for Rust
[![Lines of code](https://tokei.rs/b1/github/Lattexshz/Winey)](https://github.com/Lattexshz/Winey)  

# Target
 - Simple and easy to use
 - Highly customizable (see detailed examples [here](https://github.com/Lattexshz/winey/blob/main/winey/examples/windows.rs))

# Supported platforms
 - [x] Windows
 - [ ] MacOS
 - [x] Xlib
 - [ ] Wayland

# Example
```rust
use winey::window::Window;
use winey::{KeyCode, WindowEvent, WineyWindowImplementation};

fn main() {
    let window = Window::new("Hello World", 500, 500);

    window.show();

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
```

# License
Winey is under MIT LICENSE

