pub struct App {
    window: Option<std::sync::Arc<winit::window::Window>>,
    pixels: Option<pixels::Pixels<'static>>,
    title: String,
    width: u32,
    height: u32,
    aspect_ratio: f64,
    keep_aspect_ratio: bool,
    resizable: bool,
    fullscreen: bool,
    draw_fn: Option<Box<dyn FnMut(&mut crate::canvas::Canvas)>>,
}

impl App {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Self {
            window: None,
            pixels: None,
            title: title.to_string(),
            width,
            height,
            aspect_ratio: width as f64 / height as f64,
            keep_aspect_ratio: false,
            resizable: false,
            fullscreen: false,
            draw_fn: None,
        }
    }

    pub fn set_fullscreen(&mut self, fullscreen: bool) -> &mut Self {
        self.fullscreen = fullscreen;
        self
    }

    pub fn set_resizable(&mut self, resizable: bool) -> &mut Self {
        self.resizable = resizable;
        self
    }

    pub fn set_title(&mut self, title: &str) -> &mut Self {
        self.title = title.to_string();
        self
    }

    pub fn keep_aspect_ratio(&mut self, keep: bool) -> &mut Self {
        self.keep_aspect_ratio = keep;
        self
    }

    pub fn on_draw(&mut self, f: impl FnMut(&mut crate::canvas::Canvas) + 'static) -> &mut Self {
        self.draw_fn = Some(Box::new(f));
        self
    }

    pub fn run(mut self) {
        let event_loop = winit::event_loop::EventLoop::new().unwrap();
        event_loop.run_app(&mut self).unwrap();
    }
}

impl winit::application::ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let attributes = winit::window::Window::default_attributes()
            .with_title(self.title.clone())
            .with_resizable(self.resizable)
            .with_inner_size(winit::dpi::PhysicalSize::new(self.width, self.height))
            .with_fullscreen(
                if self.fullscreen {
                    Some(winit::window::Fullscreen::Borderless(None))
                }
                else {
                    None
                }
            );
        
        let window = std::sync::Arc::new(event_loop.create_window(attributes).expect("Failed to create window"));

        let surface_texture = pixels::SurfaceTexture::new(self.width, self.height, window.clone());
        let mut pixels = pixels::Pixels::new(self.width, self.height, surface_texture).unwrap();
        pixels.set_scaling_mode(pixels::ScalingMode::Fill);

        self.pixels = Some(pixels);
        self.window = Some(window);
        self.window.as_ref().unwrap().request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    )
    {
        if let Some(window) = &self.window {
            if window.id() == window_id {
                match event {
                    winit::event::WindowEvent::CloseRequested => {
                        event_loop.exit();
                    },
                    winit::event::WindowEvent::RedrawRequested => {
                        if let (Some(pixels), Some(draw_fn)) = (self.pixels.as_mut(), self.draw_fn.as_mut()) {
                            let mut canvas = crate::canvas::Canvas::new(pixels.frame_mut(), self.width, self.height);
                            draw_fn(&mut canvas);
                            pixels.render().unwrap();
                        }
                        self.window.as_ref().unwrap().request_redraw();
                    }
                    winit::event::WindowEvent::Resized(new_size) => {
                        if new_size.width > 0 && new_size.height > 0 {
                            if self.keep_aspect_ratio {
                                let corrected_height = (new_size.width as f64 / self.aspect_ratio).round() as u32;
                                if corrected_height != new_size.height {
                                    let window = self.window.as_ref().unwrap();
                                    let _ = window.request_inner_size(
                                        winit::dpi::PhysicalSize::new(new_size.width, corrected_height.max(1)),
                                    );
                                }
                            }

                            if let Some(pixels) = self.pixels.as_mut() {
                                pixels.resize_surface(new_size.width, new_size.height).unwrap();
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}