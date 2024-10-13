use nannou::prelude::*;

pub fn run() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    // frame.clear(PURPLE);
    let draw = app.draw();
    draw.background().color(PLUM);
    draw.ellipse().color(STEELBLUE);

    draw.rect()
        .color(STEELBLUE)
        .x_y(-250.90, -250.0)
        .w(300.0)
        .h(200.0);

    let points = (0..50).map(|i| {
        let x = i as f32 - 25.0;
        let point = pt2(x, x.sin()) * 20.0;
        (point, STEELBLUE)
    });
    draw.polyline().weight(3.0).points_colored(points);

    // Moving circle
    let sine = app.time.sin();
    let slowersine = (app.time / 2.0).sin();
    let boundary = app.window_rect();
    let x = map_range(sine, -1.0, 1.0, boundary.left(), boundary.right());
    let y = map_range(slowersine, -1.0, 1.0, boundary.bottom(), boundary.top());
    draw.ellipse().color(STEELBLUE).x_y(x, y);

    draw.to_frame(app, &frame).unwrap();
}
