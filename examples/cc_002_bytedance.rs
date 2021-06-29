use nannou::noise::*;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    logos: Vec<ByteDance>,
    noise: Perlin,
}

struct ByteDance {
    ct: Vec2,
    b: [Point2; 4],
    y: [Point2; 4],
    t: [Point2; 4],
    e: [Point2; 4],
    bw: Vec2,
    yw: Vec2,
    tw: Vec2,
    ew: Vec2,
    f: f32,
}

impl ByteDance {
    pub fn new(x: f32, y: f32) -> ByteDance {
        ByteDance {
            ct: Vec2::new(x, y),
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
            f: 1.0,
        }
    }

    pub fn resize(&mut self, f: f32) {
        self.f = 
        for i in self.b.iter_mut() {
            *i -= self.ct;
            *i *= f;
            *i += self.ct;
        }
        for i in self.y.iter_mut() {
            *i -= self.ct;
            *i *= f;
            *i += self.ct;
        }
        for i in self.t.iter_mut() {
            *i -= self.ct;
            *i *= f;
            *i += self.ct;
        }
        for i in self.e.iter_mut() {
            *i -= self.ct;
            *i *= f;
            *i += self.ct;
        }

        self.bw *= f;
        self.yw *= f;
        self.tw *= f;
        self.ew *= f;
    }

    // pub fn breath(&mut self, sn: f64, f: f32, noise: &Perlin) {
    //     self.breath_b(sn, f, noise);
    //     self.breath_y(sn, f, noise);
    //     self.breath_t(sn, f, noise);
    //     self.breath_e(sn, f, noise);
    // }

    // fn breath_b(&mut self, sn: f64, f: f32, noise: &Perlin) {
    //     let noise = noise.get([5.9 as f64, 6.0 as f64, 1.0]) as f32 * f * sn as f32;
    //     self.bw.x += 2.0 * noise;
    //     self.b[0].x += noise;
    //     self.b[1].x += noise;
    //     self.b[2].x -= noise;
    //     self.b[3].x -= noise;
    // }
    // fn breath_y(&mut self, sn: f64, f: f32, noise: &Perlin) {
    //     let noise = noise.get([self.y[0].x as f64, self.y[0].y as f64, 1.0]) as f32 * f * sn as f32;
    //     self.yw.x += 2.0 * noise;
    //     self.y[0].x += noise;
    //     self.y[1].x += noise;
    //     self.y[2].x -= noise;
    //     self.y[3].x -= noise;
    // }
    // fn breath_t(&mut self, sn: f64, f: f32, noise: &Perlin) {
    //     let noise = noise.get([sn * self.t[0].x as f64, sn * self.t[0].y as f64, 1.0]) as f32 * f;
    //     self.tw.x += 2.0 * noise;
    //     self.t[0].x += noise;
    //     self.t[1].x += noise;
    //     self.t[2].x -= noise;
    //     self.t[3].x -= noise;
    // }
    // fn breath_e(&mut self, sn: f64, f: f32, noise: &Perlin) {
    //     let noise = noise.get([sn * self.e[0].x as f64, sn * self.e[0].y as f64, 1.0]) as f32 * f;
    //     self.ew.x += 2.0 * noise;
    //     self.e[0].x += noise;
    //     self.e[1].x += noise;
    //     self.e[2].x -= noise;
    //     self.e[3].x -= noise;
    // }

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

    let mut logos = Vec::new();
    for i in 0..25 {
        let i = i - 12;
        for j in 0..25 {
            let j = j - 12;
            let mut b = ByteDance::new(0.0 + i as f32 * 40.0, 0.0 + j as f32 * 40.0);
            b.resize(0.1);
            logos.push(b);
        }
    }

    let noise = Perlin::new();

    Model { logos, noise }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let elapsed = app.elapsed_frames() as f32 / 120.0;
    // if elapsed < 1.0 {
    //     // do nothing
    // } else {
    let sn = (elapsed.cos() + 1.0) / 100.0;
    for i in model.logos.iter_mut() {
        i.resize(sn);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    // draw.rect().x_y(0.0, 0.0).w_h(100.0, 100.0).color(PLUM);
    // model.logo.draw(&draw);
    for i in model.logos.iter() {
        i.draw(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}
