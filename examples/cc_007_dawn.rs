/// 凭栏望月
/// 2021-07-13 晚 7:30 下班后，在公司的阳台上看见了美好的落日余晖。
/// 天色渐晚，露出天边的一弯月牙。
/// 以此为念。

/// 生活一个真实的云彩其实和生成一个真实的地形是相似的。
/// 我可以研究一下 Midway Displacement Algorithm 来生成地形
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
    draw.background().color(WHITE);
    draw.rect().x_y(0.0, 0.0).w_h(100.0, 100.0).color(PLUM);

    draw.to_frame(app, &frame).unwrap();
}
