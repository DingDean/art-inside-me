/// 凭栏望月
/// 2021-07-13 晚 7:30 下班后，在公司的阳台上看见了美好的落日余晖。
/// 天色渐晚，露出天边的一弯月牙。
/// 以此为念。

/// 生活一个真实的云彩其实和生成一个真实的地形是相似的。
/// 我可以研究一下 Midway Displacement Algorithm 来生成地形
use nannou::prelude::*;
use nannou::ui::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

widget_ids! {
    struct Ids {
        speed
    }
}

struct Model {
    ui: Ui,
    ids: Ids,
    speed: f32,
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::Wait);

    app.new_window()
        .size(1024, 1024)
        .view(view)
        .build()
        .unwrap();

    let mut ui = app.new_ui().build().unwrap();
    let ids = Ids::new(ui.widget_id_generator());

    Model {
        ui,
        ids,
        speed: 0.0,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    //
    let set_w = &mut model.ui.set_widgets();

    for speed in widget::Slider::new(model.speed, 0.0, 100.0)
        .w_h(200.0, 30.0)
        .label_font_size(15)
        .rgb(0.3, 0.3, 0.3)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
        .top_left_with_margin(20.0)
        .label("Resolution")
        .set(model.ids.speed, set_w)
    {
        model.speed = speed;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    draw.text(&format!("{}", model.speed)).color(RED);

    draw.to_frame(app, &frame).unwrap();

    model.ui.draw_to_frame(app, &frame).unwrap();
}
