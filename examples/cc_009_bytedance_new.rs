use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {}

struct Byte {}

impl Byte {
    fn dance(&self, draw: &Draw) {
        // TODO
    }
}

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
    draw.background().color(WHITE);
    draw.rect().x_y(0.0, 0.0).w_h(100.0, 100.0).color(PLUM);

    let byte = Byte {};

    byte.dance(&draw);

    draw.to_frame(app, &frame).unwrap();
}
