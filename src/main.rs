use winit::event;

fn main() {
    let mut app = winit_ui::App::new("Hello, world!", 1280, 720);
    app.set_resizable(true);
    app.keep_aspect_ratio(true);

    app.on_draw(|canvas| {
        canvas.draw_rect_f(100, 100, 200, 150, &winit_ui::color::Color::new(255, 0, 0, 255));
        canvas.draw_rect(400, 100, 200, 150, &winit_ui::color::Color::new(0, 255, 0, 255));
        canvas.draw_line(100, 300, 300, 500, &winit_ui::color::Color::new(0, 0, 255, 255));
        canvas.draw_circle_f(600, 400, 75, &winit_ui::color::Color::new(255, 255, 0, 255));
        canvas.draw_text(10, 40, 14.0, "The quick brown fox jumps over the lazy dog", &winit_ui::font::Font::new("IBMPlexMono-Regular.ttf"), &winit_ui::color::Color::from(0xFFFFFF));
    });

    app.run();
}
