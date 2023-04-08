use winey::window::Window;
use winey::WineyWindowImplementation;

fn main() {
    let window = Window::new("Hello World", 500, 500);
    window.show();

    window.run(|| {

    })
}