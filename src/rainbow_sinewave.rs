use nannou::noise::NoiseFn;
use nannou::prelude::*;

pub fn run() {
    nannou::app(model).update(update).run();
}

struct Model {}

fn model(app: &App) -> Model {
    app.new_window().size(600, 600).view(view).build().unwrap();

    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    frame.clear(BLACK);
    let noise = nannou::noise::Perlin::new();

    let draw = app.draw();
    let w = app.window_rect();
    let time = (app.elapsed_frames() / 100) as f64;

    let l1 = noise.get([time, 1.23]) as f32;
    let o1 = noise.get([1.0, time]) as f32;
    let l2 = noise.get([time, -0.23]) as f32;
    let o2 = noise.get([0.45, time]) as f32;

    for x in (w.left() * 10.0) as i32..(w.right() * 10.0) as i32 {
        let t1 = 3.14 * (x - 1) as f32 / 100.;
        let t2 = 3.14 * (x) as f32 / 100.;

        let p1 = ((t1 * 0.038 + l1).sin() + o1) * 100.;
        let p2 = ((t1 * 0.074 + l2 + time as f32).sin() + o2) * 100.;
        let p3 = (t1 * 5.0).sin() * (p2 - p1) / 2.0 + (p1 + p2) / 2.;

        let f1 = ((t2 * 0.038 + l1).sin() + o1) * 100.;
        let f2 = ((t2 * 0.074 + l2 + time as f32).sin() + o2) * 100.;
        let f3 = (t2 * 5.0).sin() * (f2 - f1) / 2.0 + (f1 + f2) / 2.;

        draw.line()
            .start(vec2((x - 1) as f32 / 10.0, p3))
            .end(vec2(x as f32 / 10.0, f3))
            .color(hsla(
                x as f32 / 5000.0 + time as f32,
                1.,
                0.5,
                ((p3 - f3) / 20.0).abs(),
            ))
            .stroke_weight(2.0);
    }
    draw.to_frame(app, &frame).unwrap();
}
