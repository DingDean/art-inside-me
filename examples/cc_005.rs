/// Star
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {}

fn model(app: &App) -> Model {
    app.new_window()
        .size(1024, 1024)
        .view(view)
        .build()
        .unwrap();

    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    //
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    // draw.rect().x_y(0.0, 0.0).w_h(100.0, 100.0).color(PLUM);

    let num = 200;
    let step = 2.0 * PI / (num as f32);
    let half_length = 50.0;
    let radius = 300.0;

    for i in 0..num {
        let r = random_range(0.0, 1.0);
        let pt01 = pt2(radius, half_length * r);
        let pt02 = pt2(radius, -half_length * r);

        draw.line()
            .rotate(i as f32 * step)
            .points(pt01, pt02)
            .color(WHITE)
            .stroke_weight(0.2);
    }
    draw.ellipse()
        .radius(256.0)
        .x_y(0.0, 0.0)
        .no_fill()
        .stroke_weight(0.3)
        .stroke_color(WHITE);
    draw.to_frame(app, &frame).unwrap();
}
