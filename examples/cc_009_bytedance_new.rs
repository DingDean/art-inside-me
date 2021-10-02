// use nannou::prelude::*;

// fn main() {
//     nannou::app(model).update(update).run();
// }

// struct Model {}

// /// 一个 Byte 可以是 8 个一字排开的点
// /// 每个值为 1 的点用半圆相连
// struct Byte {
//     num: i8,
// }

// impl Byte {
//     pub fn new() -> Self {
//         Self { num: 0 }
//     }

//     fn dance(&self, draw: &Draw) {
//         draw.polyline
//     }
// }

// fn model(app: &App) -> Model {
//     app.new_window()
//         .size(1024, 1024)
//         .view(view)
//         .build()
//         .unwrap();

//     Model {}
// }

// fn update(_app: &App, _model: &mut Model, _update: Update) {
//     //
// }

// fn view(app: &App, _model: &Model, frame: Frame) {
//     let draw = app.draw();
//     draw.background().color(WHITE);
//     draw.rect().x_y(0.0, 0.0).w_h(100.0, 100.0).color(PLUM);

//     let byte = Byte::new();

//     byte.dance(&draw);

//     draw.to_frame(app, &frame).unwrap();
// }
use nannou::prelude::*;

pub fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    draw.to_frame(app, &frame).unwrap();
}
