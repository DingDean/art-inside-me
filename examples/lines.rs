use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    draw.line()
        .start(Vec2::new(0.0, 0.0))
        .end(Vec2::new(100.0, 100.0))
        .color(PLUM);
    // draw.rect().x_y(0.0, 0.0).w_h(100.0, 100.0).color(PLUM);
    draw.to_frame(app, &frame).unwrap();
}
