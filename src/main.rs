use winit::event;

fn main() {
    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    let mut app = winit_ui::App::new("Hello, world!", 1280, 720, true, false);

    app.on_draw(|canvas| {
        canvas.draw_rect_f(100, 100, 200, 150, &winit_ui::color::Color::new(255, 0, 0, 255));
        canvas.draw_rect(400, 100, 200, 150, &winit_ui::color::Color::new(0, 255, 0, 255));
        canvas.draw_line(100, 300, 300, 500, &winit_ui::color::Color::new(0, 0, 255, 255));
        canvas.draw_circle_f(600, 400, 75, &winit_ui::color::Color::new(255, 255, 0, 255));
    });

    event_loop.run_app(&mut app).unwrap();
}
