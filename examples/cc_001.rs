use nannou::noise::*;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Thing {
    positions: Vec<Vec2>,
    color: Rgb<u8>,
}

impl Thing {
    pub fn new(p: Vec2) -> Self {
        let mut positions = Vec::new();
        positions.push(p);
        Thing {
            positions,
            color: rgb(0, 0, 0),
        }
    }

    pub fn rand_color(&mut self, palette: &[Rgb<u8>]) {
        let i = random_range(0, palette.len());
        let color = palette[i];
        self.color = color;
    }
}

struct Model {
    things: Vec<Thing>,
    noise: Perlin,
}

const N_THINGS: usize = 2000;

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(1024, 1024)
        .view(view)
        .build()
        .unwrap();

    let palette = [
        rgb::<u8>(55, 89, 69),
        rgb::<u8>(91, 135, 103),
        rgb::<u8>(166, 193, 152),
        rgb::<u8>(219, 223, 200),
    ];

    let mut things = Vec::new();
    let noise = Perlin::new();

    for _i in 0..N_THINGS {
        let mut thing = Thing::new(vec2(
            (random_f32() - 0.5) * 1024.0,
            (random_f32() - 0.5) * 1024.0,
        ));

        for _i in 0..10000 {
            let sn = random_f64();
            let last = thing.positions[0];
            let new = last
                + vec2(
                    noise.get([sn * last.x as f64, sn * last.y as f64, 0.0]) as f32 * 3.0,
                    noise.get([sn * last.x as f64, sn * last.y as f64, 1.0]) as f32 * 3.0,
                );
            thing.positions.insert(0, new);
        }

        thing.rand_color(&palette);
        things.push(thing);
    }

    Model { things, noise }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    // let time = app.elapsed_frames() as f32 / 120.0;

    // let sn = 0.01 + time.cos() as f64 * 0.005;

    // for thing in model.things.iter_mut() {
    //     thing.positions.clear();
    //     thing.positions.push(vec2(
    //         (random_f32() - 0.5) * 1024.0,
    //         (random_f32() - 0.5) * 1024.0,
    //     ));

    //     for _k in 0..50 {
    //         let last = thing.positions[0];
    //         let new = last
    //             + vec2(
    //                 model
    //                     .noise
    //                     .get([sn * last.x as f64, sn * last.y as f64, 0.0])
    //                     as f32,
    //                 model
    //                     .noise
    //                     .get([sn * last.x as f64, sn * last.y as f64, 1.0])
    //                     as f32,
    //             );
    //         thing.positions.insert(0, new);
    //     }
    // }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    let _time = app.elapsed_frames() as f32 / 60.0;

    if app.elapsed_frames() == 1 {
        draw.background().color(srgba(218.0, 213.0, 210.0, 0.2));
    }

    // draw.rect()
    //     .w_h(1024.0, 1024.0)
    //     .color(srgba(0.0, 0.0, 0.0, 0.3));

    // let palette = [WHITE, BLUE];
    // let max = palette.len();

    for thing in model.things.iter() {
        // let i = random_range(0, max);
        // let color = palette[i];
        draw.polyline()
            .points(thing.positions.iter().cloned())
            .color(thing.color);
    }

    draw.to_frame(app, &frame).unwrap();
}
