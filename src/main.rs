use app::App;
use winit::event_loop::{ControlFlow, EventLoop};

mod app;

fn main() {
    let mut app = App::default();

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);

    let _ = event_loop.run_app(&mut app);
}
