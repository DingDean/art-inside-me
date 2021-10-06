use nannou::noise::*;
use nannou::prelude::*;
use nannou::ui::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Octave {
    /// frequency
    f: f64,
    dimension: (i32, i32),
    noise: Perlin,
}

impl Octave {
    fn new(s: u32, f: f64, d: (i32, i32)) -> Self {
        let noise = Perlin::new().set_seed(s);
        Self {
            f,
            noise,
            dimension: d,
        }
    }

    fn set_seed(&mut self, s: u32) {
        self.noise = Perlin::new().set_seed(s);
    }

    fn seed(&self) -> u32 {
        self.noise.seed()
    }

    fn set_frequency(&mut self, f: f64) {
        self.f = f;
    }

    fn frequency(&self) -> f64 {
        self.f
    }

    fn set_dimension(&mut self, x: i32) {
        self.dimension = (x, x);
    }

    fn get(&self, x: f64, y: f64) -> f64 {
        let fx = -1.0 + x / (self.dimension.0 as f64 / 2.0);
        let fy = -1.0 + y / (self.dimension.1 as f64 / 2.0);
        (self.noise.get([fx * self.f, fy * self.f]) + 1.0) / 2.0
    }
}

widget_ids! {
    struct Ids {
        dimension,
        seed,
        frequency,
        w1,
        w2,
        w3
    }
}

struct Model {
    pub dimension: (i32, i32),
    pub w1: f64,
    pub w2: f64,
    pub w3: f64,
    heightmap: Vec<f64>,
    base_oct: Octave,
    ids: Ids,
    ui: Ui,
}

impl Model {
    fn update_heightmap(&mut self) {
        let x = self.dimension.0 / 2;
        let y = self.dimension.1 / 2;

        let weight = self.w1 + self.w2 + self.w3;
        self.heightmap = {
            let mut output = Vec::new();
            let o1 = &self.base_oct;
            let o2 = Octave::new(15, 0.08, self.dimension);
            let o3 = Octave::new(20, 32.2, self.dimension);
            for i in -x..x {
                for j in -y..y {
                    let x = pt2(i as f32, j as f32);
                    let h = (self.w1 * o1.get(i.into(), j.into())
                        + self.w2 * o2.get(i.into(), j.into())
                        + self.w3 * o3.get(i.into(), j.into()))
                        / weight;
                    // + 0.5 * (noise.get([x.x as f64 * 1.1, x.y as f64 * 3.0]) + 1.0) / 2.0;
                    // let h = noise.get([(i as f64).sin(), j as f64 / haft_y as f64]);
                    // let h = noise.get([i as f64 / haft_y as f64, (j as f64).sin()]);
                    output.push(h);
                }
            }
            output
        };
    }
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::Wait);
    app.new_window()
        .size(1024, 1024)
        .view(view)
        .build()
        .unwrap();

    let heightmap = Vec::new();

    let mut ui = app.new_ui().build().unwrap();
    let ids = Ids::new(ui.widget_id_generator());

    let dimension = (64, 64);
    let base_oct = Octave::new(1, 2.6, dimension);

    Model {
        ui,
        ids,
        base_oct,
        dimension,
        heightmap,
        w1: 1.5,
        w2: 0.5,
        w3: 0.3,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    {
        let ui = &mut model.ui.set_widgets();
        for seed in widget::Slider::new(model.base_oct.seed() as f32, 0.0, 100.0)
            .w_h(200.0, 30.0)
            .label("seed")
            .border(0.0)
            .label_font_size(15)
            .top_left_with_margin(20.0)
            .set(model.ids.seed, ui)
        {
            model.base_oct.set_seed(seed as u32);
        }

        for frequency in widget::Slider::new(model.base_oct.frequency(), 0.0, 100.0)
            .w_h(200.0, 30.0)
            .label("frequency")
            .border(0.0)
            .label_font_size(15)
            .down(20.0)
            .set(model.ids.frequency, ui)
        {
            model.base_oct.set_frequency(frequency);
        }

        for dimension in widget::Slider::new(model.dimension.0 as f64, 64.0, 1024.0)
            .w_h(200.0, 30.0)
            .label("resolution")
            .border(0.0)
            .label_font_size(15)
            .down(20.0)
            .set(model.ids.dimension, ui)
        {
            if (dimension as i32) % 2 == 0 {
                model.dimension = (dimension as i32, dimension as i32);
                model.base_oct.set_dimension(dimension as i32);
            }
        }

        for w1 in widget::Slider::new(model.w1, 0.0, 10.0)
            .w_h(200.0, 30.0)
            .label("w1")
            .border(0.0)
            .label_font_size(15)
            .down(20.0)
            .set(model.ids.w1, ui)
        {
            model.w1 = w1;
        }

        for w2 in widget::Slider::new(model.w2, 0.0, 10.0)
            .w_h(200.0, 30.0)
            .label("w2")
            .border(0.0)
            .label_font_size(15)
            .down(20.0)
            .set(model.ids.w2, ui)
        {
            model.w2 = w2;
        }

        for w3 in widget::Slider::new(model.w3, 0.0, 10.0)
            .w_h(200.0, 30.0)
            .label("w3")
            .border(0.0)
            .label_font_size(15)
            .down(20.0)
            .set(model.ids.w3, ui)
        {
            model.w3 = w3;
        }
    }

    model.update_heightmap();
}

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

            let base_color = if n > 0.9 {
                srgba(46.0 / 256.0, 89.0 / 256.0, 167.0 / 256.0, 1.0)
            } else if n > 0.7 {
                srgba(89.0 / 256.0, 118.0 / 256.0, 186.0 / 256.0, 1.0)
            } else if n > 0.5 {
                srgba(136.0 / 256.0, 171.0 / 256.0, 218.0 / 256.0, 1.0)
            } else {
                srgba(213.0 / 256.0, 227.0 / 256.0, 212.0 / 256.0, 1.0)
            };

            draw.rect().w_h(width, width).x_y(x, y).color(base_color);
        }
    }
    draw.to_frame(app, &frame).unwrap();
    model.ui.draw_to_frame(app, &frame).unwrap();
}
