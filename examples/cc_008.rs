use nannou::prelude::*;
use nannou::ui::prelude::*;

widget_ids! {
    struct Ids {
        steps
    }
}

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    ui: Ui,
    ids: Ids,
    pts: Vec<Point2>,
    moutain: Mountain,
}

struct Mountain {
    height: f32,
    /// # of times to displace midpoints
    ite: i32,

    // start: Point2,
    // end: Point2,
    pts: Vec<Point2>,
}

impl Mountain {
    fn new(start: Point2, end: Point2, height: f32) -> Self {
        let mut pts = Vec::new();
        pts.push(start);
        pts.push(end);

        Self {
            height,
            pts,
            ite: 0,
        }
    }

    fn displace(&self) -> f32 {
        (random_f32() + 1.0) * self.height
    }

    fn run(&self, step: i32) -> Vec<Point2> {
        let mut temp = self.pts.clone();

        for _i in 0..step {
            let mut it = temp.iter().peekable();

            let mut new_pts = Vec::new();
            loop {
                match (it.next(), it.peek()) {
                    (Some(a), Some(b)) => {
                        let mid = (*a + **b) / 2.0;
                        let displacement = self.displace();

                        let pt = pt2(mid.x, displacement);
                        new_pts.push(a.clone());
                        new_pts.push(pt);
                    }
                    (Some(a), None) => {
                        new_pts.push(a.clone());
                    }
                    _ => {
                        break;
                    }
                }
            }
            temp = new_pts;
        }
        temp
    }
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::Wait);
    app.new_window()
        .size(1024, 1024)
        .view(view)
        .build()
        .unwrap();

    let mut ui = app.new_ui().build().unwrap();
    let ids = Ids::new(ui.widget_id_generator());

    let start = pt2(-500.0, 0.0);
    let end = pt2(500.0, 0.0);
    let height = 300.0;

    let moutain = Mountain::new(start, end, height);
    let pts = moutain.run(2);

    Model {
        ui,
        ids,
        moutain,
        pts,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let ui = &mut model.ui.set_widgets();
    for step in widget::Slider::new(model.moutain.ite as f32, 0.0, 10.0)
        .w_h(200.0, 30.0)
        .label("# of Iterations")
        .border(0.0)
        .label_font_size(15)
        .rgb(0.3, 0.3, 0.3)
        .top_left_with_margin(20.0)
        .set(model.ids.steps, ui)
    {
        model.moutain.ite = step as i32;
    }

    model.pts = model.moutain.run(model.moutain.ite);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    draw.polyline().points(model.pts.clone()).color(PLUM);

    draw.to_frame(app, &frame).unwrap();

    model.ui.draw_to_frame(app, &frame).unwrap();
}
