/// Line
/// 我们可以如何利用线条？
///
/// 单个线条有哪些性质?
///
/// 1. 长度
/// 2. 粗细
/// 3. 圆角弧度
/// 4. 颜色
/// 5. 方向
///
/// 两个线条有哪些性质
///
/// 1. 角度
use nannou::prelude::*;

struct Model {

} 

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    app.new_window().size(1024, 1024).view(view);

    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    
}

fn view(_app: &App, _model: &Model, _frame: Frame) {
    
}
