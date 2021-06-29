use nannou::prelude::*;
use nannou::noise::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    logo: ByteDance,
    noise: Perlin,
}

struct ByteDance {
    b: [Point2; 4],
    y: [Point2; 4],
    t: [Point2; 4],
    e: [Point2; 4],
    bw: Vec2,
    yw: Vec2,
    tw: Vec2,
    ew: Vec2,
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
            bw: Vec2::new(65.0, 300.0),
            y: [
                pt2(x - 85.0, y - 180.0),
                pt2(x - 85.0, y + 40.0),
                pt2(x - 20.0, y + 22.0),
                pt2(x - 20.0, y - 162.0),
            ],
            yw: Vec2::new(65.0, 180.0),
            t: [
                pt2(x + 85.0, y - 135.0),
                pt2(x + 85.0, y + 85.0),
                pt2(x + 20.0, y + 67.0),
                pt2(x + 20.0, y - 117.0),
            ],
            tw: Vec2::new(65.0, 180.0),
            e: [
                pt2(x + 190.0, y - 153.0),
                pt2(x + 190.0, y + 153.0),
                pt2(x + 125.0, y + 168.0),
                pt2(x + 125.0, y - 168.0),
            ],
            ew: Vec2::new(65.0, 336.0),
        }
    }

    pub fn resize(&mut self, f: f32) {
        for i in self.b.iter_mut() {
            *i *= f;
        }
        for i in self.y.iter_mut() {
            *i *= f;
        }
        for i in self.t.iter_mut() {
            *i *= f;
        }
        for i in self.e.iter_mut() {
            *i *= f;
        }

        self.bw *= f;
        self.yw *= f;
        self.tw *= f;
        self.ew *= f;
    }

    pub fn draw(&self, draw: &Draw) {
        // 左数第一个
        draw.quad()
            .w_h(self.bw.x, self.bw.y)
            .points(self.b[0], self.b[1], self.b[2], self.b[3])
            .color(rgb8(41, 79, 146));
        // 右数第三个
        draw.quad()
            .w_h(self.yw.x, self.yw.y)
            .points(self.y[0], self.y[1], self.y[2], self.y[3])
            .color(rgb8(65, 113, 170));
        // 右数第二个
        draw.quad()
            .w_h(self.tw.x, self.tw.y)
            .points(self.t[0], self.t[1], self.t[2], self.t[3])
            .color(rgb8(29, 165, 173));
        draw.quad()
            .w_h(self.ew.x, self.ew.y)
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

    // for i in 0..50 {
    //     let i = i- 25;
    //     let mut b = ByteDance::new(0.0 + i as f32 * 190.0, 0.0);
    //     b.resize(0.5);
    //     logos.push(b);
    let mut logo = ByteDance::new(0.0, 0.0);
    logo.resize(1.5);
    // }

    let noise = Perlin::new();

    Model { logo, noise }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let noise = model.noise.get([model.logo.bw.x as f64, model.logo.bw.y as f64]) as f32 * 10.0;
    println!("noise: {}", noise);
    model.logo.bw.x += 2.0 * noise;
    model.logo.b[0].x += noise;
    model.logo.b[1].x += noise;
    model.logo.b[2].x -= noise;
    model.logo.b[3].x -= noise;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    // draw.rect().x_y(0.0, 0.0).w_h(100.0, 100.0).color(PLUM);
    model.logo.draw(&draw);

    draw.to_frame(app, &frame).unwrap();
}
