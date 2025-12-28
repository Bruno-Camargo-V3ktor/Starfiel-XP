use softbuffer;
use std::{num::NonZeroU32, sync::Arc};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    window::{Window, WindowAttributes},
};

use crate::star::Star;

#[derive(Default)]
pub struct App {
    pub window: Option<Arc<Window>>,
    pub context: Option<softbuffer::Context<Arc<Window>>>,
    pub surface: Option<softbuffer::Surface<Arc<Window>, Arc<Window>>>,

    pub stars: Vec<Star>,
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

            let count = 1000;
            let initial_spread = 1000.0;

            for _ in 0..count {
                self.stars
                    .push(Star::new_rng(initial_spread, initial_spread));
            }
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
                    buffer.fill(0xFF202020);

                    /*
                        let width_u32 = width.get();
                        let height_u32 = height.get();

                        let rect_x = 200;
                        let rect_y = 200;
                        let rect_w = 100;
                        let rect_h = 100;
                        let rect_color: u32 = 0xFFFF1100;

                        for y in rect_y..(rect_y + rect_h) {
                            for x in rect_x..(rect_x + rect_w) {
                                if (x < 0 || x >= width_u32) || (y < 0 || y >= height_u32) {
                                    continue;
                                }

                                let index = (y * width_u32 + x) as usize;

                                buffer[index] = rect_color;
                            }
                        }
                    */

                    let speed = 10.0;
                    let spread = 500.0;
                    let max_depth = 2000.0;

                    let half_width = width.get() as f32 / 2.0;
                    let half_height = height.get() as f32 / 2.0;

                    for star in &mut self.stars {
                        star.update(speed, spread, spread);

                        let scale_factor = width.get() as f32;

                        let sx = (star.x / star.z) * scale_factor + half_width;
                        let sy = (star.y / star.z) * scale_factor + half_height;

                        let proximity = 1.0 - (star.z / max_depth);
                        let star_size = proximity * star.size;
                        let radius = (star_size / 2.0).max(1.0) as i32;

                        for y in (sy as i32 - radius)..(sy as i32 + radius) {
                            for x in (sx as i32 - radius)..(sx as i32 + radius) {
                                let is_inside_screen = x >= 0
                                    && x < width.get() as i32
                                    && y >= 0
                                    && y < height.get() as i32;

                                if is_inside_screen {
                                    let index = (y as u32 * width.get() + x as u32) as usize;
                                    buffer[index] = 0xFFFFFFFF;
                                }
                            }
                        }
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
