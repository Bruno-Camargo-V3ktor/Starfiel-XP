use softbuffer;
use std::{num::NonZeroU32, sync::Arc};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    window::{Window, WindowAttributes},
};

#[derive(Default)]
pub struct App {
    pub window: Option<Arc<Window>>,
    pub context: Option<softbuffer::Context<Arc<Window>>>,
    pub surface: Option<softbuffer::Surface<Arc<Window>, Arc<Window>>>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            let window = event_loop
                .create_window(WindowAttributes::default().with_title("My Window"))
                .unwrap();

            let window = Arc::new(window);
            self.window = Some(window.clone());

            let context = softbuffer::Context::new(window.clone()).unwrap();
            let surface = softbuffer::Surface::new(&context, window.clone()).unwrap();

            self.context = Some(context);
            self.surface = Some(surface);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::RedrawRequested => {
                if let (Some(window), Some(surface)) = (&self.window, &mut self.surface) {
                    let (width, height) = {
                        let size = window.inner_size();
                        (
                            NonZeroU32::new(size.width).unwrap_or(NonZeroU32::new(1).unwrap()),
                            NonZeroU32::new(size.height).unwrap_or(NonZeroU32::new(1).unwrap()),
                        )
                    };

                    surface.resize(width, height).unwrap();

                    let mut buffer = surface.buffer_mut().unwrap();
                    let windth_u32 = width.get();

                    for index in 0..buffer.len() {
                        let y = index as u32 / windth_u32;
                        let x = index as u32 % windth_u32;

                        let red = x % 255;
                        let green = y % 255;
                        let blue = (x * y) % 255;

                        let color: u32 = blue | (green << 8) | (red << 16);
                        buffer[index] = color;
                    }

                    buffer.present().unwrap();
                    self.window.as_ref().unwrap().request_redraw();
                }
            }

            _ => {
                println!("{event:?}")
            }
        }
    }
}
