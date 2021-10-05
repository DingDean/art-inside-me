use nannou::noise::*;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    dimension: (i32, i32),
    noise: Perlin,
    heightmap: Vec<f64>,
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
    let noise = Perlin::new().set_seed(45);
    let half_x = dimension.0 / 2;
    let haft_y = dimension.1 / 2;
    let heightmap = {
        let mut output = Vec::new();
        for i in -half_x..half_x {
            for j in -haft_y..haft_y {
                let x = pt2(i as f32, j as f32);
                let h = (5.5 * (noise.get([x.x as f64 * 0.02, x.y as f64 * 0.02]) + 1.0) / 2.0
                    + 0.5 * (noise.get([x.x as f64 * 0.08, x.y as f64 * 0.08]) + 1.0) / 2.0
                    + 0.1 * (noise.get([x.x as f64 * 2.2, x.y as f64 * 2.2]) + 1.0) / 2.0)
                    / 6.1;
                // + 0.5 * (noise.get([x.x as f64 * 1.1, x.y as f64 * 3.0]) + 1.0) / 2.0;
                // let h = noise.get([(i as f64).sin(), j as f64 / haft_y as f64]);
                // let h = noise.get([i as f64 / haft_y as f64, (j as f64).sin()]);
                println!("{}", h);
                output.push(h);
            }
        }
        output
    };

    Model {
        noise,
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
            // .stroke_weight(2.0)
            // .stroke(stroke_color);
        }
    }

    // draw m x n grids
    // for i in 1..m {
    //     let step = (i as f32) * w / (m as f32);
    //     draw.line()
    //         .start(pt2(-w / 2.0 + step, -h / 2.0))
    //         .end(pt2(-w / 2.0 + step, h / 2.0))
    //         .color(RED);
    // }

    // for i in 1..n {
    //     let step = (i as f32) * w / (n as f32);
    //     draw.line()
    //         .start(pt2(-w / 2.0, -h / 2.0 + step))
    //         .end(pt2(w / 2.0, -h / 2.0 + step))
    //         .color(RED);
    // }

    draw.to_frame(app, &frame).unwrap();
}
