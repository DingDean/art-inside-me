use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    logos: Vec<ByteDance>,
}

struct ByteDance {
    b: [Point2; 4],
    y: [Point2; 4],
    t: [Point2; 4],
    e: [Point2; 4],
}

impl ByteDance {
    pub fn new(x: f32, y: f32) -> ByteDance {
        ByteDance {
            b: [
                pt2(x - 125.0, y - 133.0),
                pt2(x - 125.0, y + 133.0),
                pt2(x - 190.0, y + 148.0),
                pt2(x - 190.0, y - 148.0),
            ],
            y: [
                pt2(x - 85.0, y - 180.0),
                pt2(x - 85.0, y + 40.0),
                pt2(x - 20.0, y + 22.0),
                pt2(x - 20.0, y - 162.0),
            ],
            t: [
                pt2(x + 85.0, y - 135.0),
                pt2(x + 85.0, y + 85.0),
                pt2(x + 20.0, y + 67.0),
                pt2(x + 20.0, y - 117.0),
            ],
            e: [
                pt2(x + 190.0, y - 153.0),
                pt2(x + 190.0, y + 153.0),
                pt2(x + 125.0, y + 168.0),
                pt2(x + 125.0, y - 168.0),
            ],
        }
    }

    pub fn draw(&self, draw: &Draw) {
        // 左数第一个
        draw.quad()
            .w_h(65.0, 300.0)
            .points(self.b[0], self.b[1], self.b[2], self.b[3])
            .color(rgb8(41, 79, 146));
        // 右数第三个
        draw.quad()
            .w_h(65.0, 180.0)
            .points(self.y[0], self.y[1], self.y[2], self.y[3])
            .color(rgb8(65, 113, 170));
        // 右数第二个
        draw.quad()
            .w_h(65.0, 180.0)
            .points(self.t[0], self.t[1], self.t[2], self.t[3])
            .color(rgb8(29, 165, 173));
        draw.quad()
            .w_h(65.0, 336.0)
            .points(self.e[0], self.e[1], self.e[2], self.e[3])
            .color(rgb8(123, 188, 185));
    }
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(1024, 1024)
        .view(view)
        .build()
        .unwrap();

    let mut logos = Vec::new();
    logos.push(ByteDance::new(0.0, 0.0));

    Model { logos }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    //
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    // draw.rect().x_y(0.0, 0.0).w_h(100.0, 100.0).color(PLUM);
    for i in model.logos.iter() {
        i.draw(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}
