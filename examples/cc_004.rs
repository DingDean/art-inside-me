/// Mandelbrot Set
use nannou::prelude::*;

// const Z: [f32; 2] = [1.0, 1.0];
// const THREDHOLD: f32 = 99.0;
// const ITERATION: i32 = 1000;

fn main() {
    nannou::app(model).update(update).run();
}

struct MandelbrotSet {
    z: [f32; 2],
    thredhold: f32,
    max_ite: i32,
    pts: Vec<Point2>,
}

impl MandelbrotSet {
    pub fn is_in_set(&self, pt: &Point2) {}
}

fn model(app: &App) -> MandelbrotSet {
    app.new_window()
        .size(1024, 1024)
        .view(view)
        .build()
        .unwrap();

    MandelbrotSet {
        z: [1.0, 1.0],
        thredhold: 99.0,
        max_ite: 1000,
        pts: Vec::new(),
    }
}

fn update(_app: &App, _model: &mut MandelbrotSet, _update: Update) {
    //
}

fn view(app: &App, _model: &MandelbrotSet, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    draw.rect().x_y(0.0, 0.0).w_h(100.0, 100.0).color(PLUM);

    draw.to_frame(app, &frame).unwrap();
}
