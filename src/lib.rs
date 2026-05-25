mod canvas;
pub mod color;

pub struct App {
    window: Option<std::sync::Arc<winit::window::Window>>,
    pixels: Option<pixels::Pixels<'static>>,
    title: String,
    width: u32,
    height: u32,
    resizable: bool,
    fullscreen: bool,
    draw_fn: Option<Box<dyn FnMut(&mut canvas::Canvas)>>,
}

impl App {
    pub fn new(title: &str, width: u32, height: u32, resizable: bool, fullscreen: bool) -> Self {
        Self {
            window: None,
            pixels: None,
            title: title.to_string(),
            width,
            height,
            resizable,
            fullscreen,
            draw_fn: None,
        }
    }

    pub fn on_draw(&mut self, f: impl FnMut(&mut canvas::Canvas) + 'static) -> &mut Self {
        self.draw_fn = Some(Box::new(f));
        self
    }

    pub fn run(self) {
        let event_loop = winit::event_loop::EventLoop::new().unwrap();
        let mut handler = AppHandler {
            title: self.title,
            width: self.width,
            height: self.height,
            draw_fn: self.draw_fn,
            window: self.window,
            pixels: self.pixels,
        };
        event_loop.run_app(&mut handler).unwrap();
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
        let pixels = pixels::Pixels::new(self.width, self.height, surface_texture).unwrap();

        self.pixels = Some(pixels);
        self.window = Some(window);
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
                            let mut canvas = canvas::Canvas::new(pixels.frame_mut(), self.width, self.height);
                            draw_fn(&mut canvas);
                            pixels.render().unwrap();
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

struct AppHandler {
    title: String,
    width: u32,
    height: u32,
    draw_fn: Option<Box<dyn FnMut(&mut canvas::Canvas)>>,
    window: Option<std::sync::Arc<winit::window::Window>>,
    pixels: Option<pixels::Pixels<'static>>,
}

impl winit::application::ApplicationHandler for AppHandler {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = std::sync::Arc::new(
            event_loop.create_window(
                winit::window::Window::default_attributes()
                    .with_title(&self.title)
                    .with_inner_size(winit::dpi::LogicalSize::new(self.width, self.height))
            ).unwrap()
        );

        let surface_texture = pixels::SurfaceTexture::new(self.width, self.height, window.clone());
        let pixels = pixels::Pixels::new(self.width, self.height, surface_texture).unwrap();

        self.pixels = Some(pixels);
        self.window = Some(window);
    }

    fn window_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, _id: winit::window::WindowId, event: winit::event::WindowEvent) {
        match event {
            winit::event::WindowEvent::CloseRequested => event_loop.exit(),
            winit::event::WindowEvent::RedrawRequested => {
                if let (Some(pixels), Some(draw_fn)) = (self.pixels.as_mut(), self.draw_fn.as_mut()) {
                    let mut canvas = canvas::Canvas::new(pixels.frame_mut(), self.width, self.height);
                    draw_fn(&mut canvas);
                    pixels.render().unwrap();
                }
            }
            _ => {}
        }
    }
}