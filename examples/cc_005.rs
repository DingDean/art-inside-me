/// Star
use nannou::prelude::*;
use std::collections::HashMap;

fn main() {
    nannou::app(model).update(update).view(view).run();
}

struct Model {
    ss: HashMap<WindowId, Things>,
}

impl Model {
    pub fn new(app: &App) -> Self {
        let wa = app
            .new_window()
            .title("鸟巢")
            .size(1024, 1024)
            .build()
            .unwrap();
        let wb = app
            .new_window()
            .title("漩涡")
            .size(1024, 1024)
            .build()
            .unwrap();

        let mut ss = HashMap::new();

        ss.insert(wa, Things::m01());
        ss.insert(wb, Things::m02());

        Self { ss }
    }
}

struct Things {
    pts: Vec<Thing>,
    step: f32,
}

struct Thing {
    a: Point2,
    b: Point2,
}

impl Things {
    /// Bird Nest
    pub fn m01() -> Self {
        let num = 500;
        // let step = 2.0 * PI / (num as f32);
        let half_length = 50.0;
        let radius = 300.0;

        let mut pts = Vec::new();
        for _i in 0..num {
            let r = random_range(0.0, 1.0);
            let mr = 0.5 * r;
            let a = pt2(radius * mr, half_length * r);
            let b = pt2(radius * mr, -half_length * r);

            pts.push(Thing { a, b });
        }

        let step = 2.0 * PI / (num as f32);
        Self { pts, step }
    }

    /// Rotation
    pub fn m02() -> Self {
        let num = 500;
        // let step = 2.0 * PI / (num as f32);
        let half_length = 50.0;
        let radius = 300.0;

        let mut pts = Vec::new();
        for _i in 0..num {
            let r = random_range(0.0, 1.0);
            let mr = 0.5 * r;
            let a = pt2(radius * mr, half_length * r);
            let b = pt2(radius, -half_length * r);

            pts.push(Thing { a, b });
        }

        let step = 2.0 * PI / (num as f32);
        Self { pts, step }
    }
}

impl Things {
    pub fn draw(&self, draw: &Draw) {
        self.pts.iter().enumerate().for_each(|(i, x)| {
            draw.line()
                .rotate(i as f32 * self.step)
                .points(x.a, x.b)
                .color(WHITE)
                .stroke_weight(0.2);
        });
    }
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(1024, 1024)
        .view(view)
        .build()
        .unwrap();

    Model::new(app)
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    //
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    let wi = frame.window_id();
    if let Some(things) = model.ss.get(&wi) {
        things.draw(&draw);
    };
    // things.draw(&draw);

    draw.to_frame(app, &frame).unwrap();
}
