use nannou::noise::*;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    dimension: (i32, i32),
    heightmap: Vec<f64>,
}

struct Octave {
    /// frequency
    f: f64,
    /// Seed for Perlin
    s: u32,
    noise: Perlin,
}

impl Octave {
    fn new(s: u32, f: f64) -> Self {
        let noise = Perlin::new().set_seed(s);
        Self { f, s, noise }
    }

    fn get(&self, x: f64, y: f64) -> f64 {
        (self.noise.get([x * self.f, y * self.f]) + 1.0) / 2.0
    }
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::NTimes {
        number_of_updates: 1,
    });
    app.new_window()
        .size(1024, 1024)
        .view(view)
        .build()
        .unwrap();

    let dimension = (256, 256);
    let half_x = dimension.0 / 2;
    let haft_y = dimension.1 / 2;
    let heightmap = {
        let mut output = Vec::new();
        let o1 = Octave::new(1, 0.01);
        let o2 = Octave::new(15, 0.08);
        let o3 = Octave::new(20, 32.2);
        for i in -half_x..half_x {
            for j in -haft_y..haft_y {
                let x = pt2(i as f32, j as f32);
                let h = (10.5 * o1.get(i.into(), j.into())
                    + 0.5 * o2.get(i.into(), j.into())
                    + 0.3 * o3.get(i.into(), j.into()))
                    / 11.3;
                // + 0.5 * (noise.get([x.x as f64 * 1.1, x.y as f64 * 3.0]) + 1.0) / 2.0;
                // let h = noise.get([(i as f64).sin(), j as f64 / haft_y as f64]);
                // let h = noise.get([i as f64 / haft_y as f64, (j as f64).sin()]);
                output.push(h);
            }
        }
        output
    };

    Model {
        dimension,
        heightmap,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    let window = app.window_rect();
    let w = window.w();
    let h = window.h();

    let m = model.dimension.0;
    let n = model.dimension.1;

    // draw m x n heightmap
    for i in 0..m {
        for j in 0..n {
            let width = w / (m as f32);
            let height = h / (n as f32);
            let x = -w / 2.0 + (i as f32 + 0.5) * width;
            let y = -h / 2.0 + (j as f32 + 0.5) * height;
            let index = (i * m + j) as usize;
            let n = model.heightmap[index] as f64;

            let base_color = srgba(46.0 / 256.0, 89.0 / 256.0, 167.0 / 256.0, 1.0 * n);

            draw.rect().w_h(width, width).x_y(x, y).color(base_color);
        }
    }
    draw.to_frame(app, &frame).unwrap();
}
