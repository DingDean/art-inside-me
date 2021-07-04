/// Fractal 101
///
///
///
use nannou::prelude::*;

struct Model {
    points: Vec<Point2>,
    depth: i32,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model (app: &App) -> Model {
    app.new_window().size(1024, 1024).view(view).build().unwrap();

    let a = pt2(-500.0, 0.0);
    let b = pt2(0.0, 200.0);
    let c = pt2(500.0, 0.0);

    let mut points = Vec::new();
    points.push(a);
    points.push(b);
    points.push(c);
    points.push(a);

    Model {
        points,
        depth: 0,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let frame = app.elapsed_frames();

    if model.depth  < 10 && frame % 60 == 0  {

    let mut it = model.points.iter().peekable();
    let mut new_points = Vec::new();

    loop {
        let cur = it.next();
        let nex = it.peek();

        match (cur, nex) {
            (Some(a), Some(b)) => {
                let a = a.clone();
                let b = *b.clone();

                let left_step = (b - a) / 8.0;
                let up_step = (b-a).perp().normalize() * (3.0 * left_step.length() / 2.0.sqrt());

                let x1 = a + 2.0 * left_step;
                let x2 = x1 + left_step + up_step;
                let x3 = x2 + left_step - up_step;
                new_points.push(a);
                new_points.push(x1);
                new_points.push(x2);
                new_points.push(x3);
            },
            (None, _) => {
                break;
            },
            (Some(a), None) => {
                new_points.push(a.clone());
            }
        }
    }
    model.points = new_points;
    model.depth += 1;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    let it = model.points.iter().skip(1);
    for (a, b) in model.points.iter().zip(it) {
        draw.line().points(a.clone(), b.clone()).color(PLUM);
    }

    draw.to_frame(app, &frame).unwrap();
}
