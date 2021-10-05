use nannou::noise::*;
use nannou::prelude::*;
use nannou::ui::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Octave {
    /// frequency
    f: f64,
    noise: Perlin,
}

impl Octave {
    fn new(s: u32, f: f64) -> Self {
        let noise = Perlin::new().set_seed(s);
        Self { f, noise }
    }

    fn set_seed(&mut self, s: u32) {
        self.noise = Perlin::new().set_seed(s);
    }

    fn seed(&self) -> u32 {
        self.noise.seed()
    }

    fn get(&self, x: f64, y: f64) -> f64 {
        (self.noise.get([x * self.f, y * self.f]) + 1.0) / 2.0
    }
}

widget_ids! {
    struct Ids {
        seed
    }
}

struct Model {
    dimension: (i32, i32),
    heightmap: Vec<f64>,
    base_oct: Octave,
    ids: Ids,
    ui: Ui,
}

impl Model {
    fn update_heightmap(&mut self) {
        let x = self.dimension.0 / 2;
        let y = self.dimension.1 / 2;

        self.heightmap = {
            let mut output = Vec::new();
            let o1 = &self.base_oct;
            let o2 = Octave::new(15, 0.08);
            let o3 = Octave::new(20, 32.2);
            for i in -x..x {
                for j in -y..y {
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

    let base_oct = Octave::new(1, 0.01);
    let dimension = (64, 64);

    Model {
        ui,
        ids,
        base_oct,
        dimension,
        heightmap,
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
            .rgb(0.3, 0.3, 0.3)
            .top_left_with_margin(20.0)
            .set(model.ids.seed, ui)
        {
            model.base_oct.set_seed(seed as u32);
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

            let base_color = srgba(46.0 / 256.0, 89.0 / 256.0, 167.0 / 256.0, 1.0 * n);

            draw.rect().w_h(width, width).x_y(x, y).color(base_color);
        }
    }
    draw.to_frame(app, &frame).unwrap();
    model.ui.draw_to_frame(app, &frame).unwrap();
}
