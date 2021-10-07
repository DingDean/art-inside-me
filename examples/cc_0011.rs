use nannou::noise::*;
use nannou::prelude::*;

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
    // f: f32,
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
            // f: 1.0,
        }
    }

    fn is_in_b(&self, x: f32, y: f32) -> bool {
        x.ge(&-190.0) && x.le(&-125.0) && y.le(&148.0) && y.ge(&-148.0)
    }

    fn is_in_y(&self, x: f32, y: f32) -> bool {
        x.ge(&-85.0) && x.le(&-20.0) && y.le(&40.0) && y.ge(&-162.0)
    }

    fn is_in_t(&self, x: f32, y: f32) -> bool {
        x.ge(&20.0) && x.le(&85.0) && y.le(&85.0) && y.ge(&-117.0)
    }

    fn is_in_e(&self, x: f32, y: f32) -> bool {
        x.ge(&125.0) && x.le(&190.0) && y.le(&153.0) && y.ge(&-153.0)
    }

    pub fn overlap(&self, x: f32, y: f32) -> bool {
        self.is_in_b(x, y) || self.is_in_t(x, y) || self.is_in_y(x, y) || self.is_in_e(x, y)
    }
}

fn my_rgba(r: u8, g: u8, b: u8, a: f64) -> Srgba {
    srgba(
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0,
        a as f32,
    )
}

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    bytedance: ByteDance,
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::NTimes {
        number_of_updates: 500,
    });
    app.new_window()
        .size(1024, 1024)
        .view(view)
        .build()
        .unwrap();

    Model {
        bytedance: ByteDance::new(0.0, 0.0),
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    //
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    let w = app.window_rect().w();
    let h = app.window_rect().h();

    let wuzhe = my_rgba(221, 107, 123, 1.0); // r
    let bintai = my_rgba(190, 202, 183, 1.0); // g
    let qielan = my_rgba(136, 171, 218, 1.0); // b
    let longzhan = my_rgba(95, 67, 33, 1.0);
    let qiehui = my_rgba(190, 177, 170, 1.0);
    let huangai = my_rgba(164, 95, 68, 1.0);
    let jiliang = my_rgba(235, 237, 223, 1.0);
    let zhizi = my_rgba(250, 192, 61, 1.0);
    let zimojin = my_rgba(188, 131, 107, 1.0);
    let huangliliu = my_rgba(254, 220, 94, 1.0);

    if app.elapsed_frames() == 1 {
        // draw.background().color(bintai);
        // draw.background().color(BLACK);
        draw.background().color(jiliang);
    }

    let mut points = Vec::new();
    let mut x = -512.0;
    let mut y = random_range(-0.5, 0.5) * h as f64;

    let noise = Perlin::new().set_seed(random::<u32>());

    let range = 1.2;
    let frequency = 0.01;
    let line_length = 2048;

    for _ in 0..line_length {
        let n = noise.get([x * frequency, y * frequency]) * range;
        let px = x + n.cos();
        let py = y + n.sin();

        let pt = pt2(px as f32, py as f32);
        let v = 1.0 - (pt - pt2(0.0, 0.0)).length() / 328.0;
        let color = if model.bytedance.overlap(px as f32, py as f32) {
            zhizi
        // draw.background().color(qiehui);
        } else {
            // let (r, g, b, _) = wuzhe.into_components();
            let (r, g, b, _) = huangliliu.into_components();
            srgba(r, g, b, v)
        };

        points.push((pt, color));

        x = px;
        y = py;
    }

    draw.polyline().points_colored(points);

    draw.to_frame(app, &frame).unwrap();
}
