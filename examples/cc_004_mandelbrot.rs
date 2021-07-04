/// Mandelbrot Set
use nannou::prelude::*;

// const Z: [f32; 2] = [1.0, 1.0];
// const THREDHOLD: f32 = 99.0;
// const ITERATION: i32 = 1000;

fn main() {
    nannou::app(model).update(update).run();
}

fn complex_square(m: &Point2) -> Point2 {
    complex_product(m, m)
}

fn complex_product(m: &Point2, n: &Point2) -> Point2 {
    let x = m.x * n.x - m.y * n.y;
    let y = m.x * n.y + m.y * n.x;

    pt2(x, y)
}

struct Model {
    vertices: Vec<Vertex>
}

struct Vertex {
    position: Point2,
    color: Hsv,
}

struct MandelbrotSet {
    z: Point2,
    thredhold: f32,
    max_ite: i32,
}


impl MandelbrotSet {
    pub fn is_in_set(&self, pt: &Point2) -> i32 {
        let mut z = self.z.clone();
        for i in 0..self.max_ite {
            z = 0.8 * complex_square(&z) + pt.clone();
            if z.length() > self.thredhold {
                return i;
            }
        }
        self.max_ite
    }
}

fn model(app: &App) -> Model {
    let half_width: i32 = 512;
    let half_height: i32 = 512;

    let width = 2 * half_width;
    let height = 2 * half_height;

    let look_at = pt2(0.8, 1.5);
    let x_range = 3.0;
    let y_range = 3.0;

    let x_factor = x_range / width as f32;
    let y_factor = y_range / height as f32;

    let x_base = look_at[0] - x_range / 2.0;
    let y_base = look_at[1] - y_range / 2.0;

    app.new_window()
        .size(width as u32, height as u32)
        .view(view)
        .build()
        .unwrap();

    let mandelbrot = MandelbrotSet {
        z: pt2( 0.0, 0.0 ),
        thredhold: 2.0,
        max_ite: 100,
    };

    let mut vertices = Vec::new();

    let hbase = 3.0 * mandelbrot.max_ite as f32;

    for i in -half_width..half_width {
        for j in -half_height..half_height {
            let pt = pt2(i as f32 * x_factor + x_base, j as f32 * y_factor + y_base);
            let  depth = mandelbrot.is_in_set(&pt);
            let value = if depth < mandelbrot.max_ite {
                1.0
            } else {
                0.0
            };
            let hue = (depth as f32)/hbase;
            let sat = (hbase- depth as f32) / hbase;
            // let color = hsv(255.0 * (depth as f32) / mandelbrot.max_ite as f32, 255.0, value);
            // let color = hsv(hue, 255.0, value);
            let color = hsv(hue, sat, value);
            vertices.push(Vertex {position: pt2(i as f32, j as f32), color});
        }
    }

    Model {
        vertices
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    //
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    for i in model.vertices.iter() {
        draw.ellipse().radius(0.5).x_y(i.position.x, i.position.y).color(i.color);
    }

    draw.to_frame(app, &frame).unwrap();
}
