use std::ptr;

use nannou::{color, prelude::*};

const WIDTH: u32 = 600;
const HEIGHT: u32 = 600;

pub fn run() {
    nannou::app(model).update(update).run();
}

#[allow(dead_code)]
struct Model {
    // Color u8 scale (0 - 255) but with f32 equivalent (0. - 1.)
    color: color::Rgb,
    step: u8,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(WIDTH, HEIGHT)
        .view(view)
        .build()
        .unwrap();

    Model {
        color: color::Rgb::new(1., 0., 0.),
        step: 0,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    update_with_pointers(model);
}

// Applies same logic of procedural.
// But instead use of pointers to remove redundant logic.
fn update_with_pointers(model: &mut Model) {
    #[allow(unused_assignments)]
    let mut col_a: *mut f32 = ptr::null_mut();
    #[allow(unused_assignments)]
    let mut col_b: *mut f32 = ptr::null_mut();
    let transition_frame: f32 = 0.0025;

    unsafe {
        match model.step {
            0 => {
                col_a = &mut model.color.blue;
                col_b = &mut model.color.red;
            }
            1 => {
                col_a = &mut model.color.green;
                col_b = &mut model.color.blue;
            }
            2 => {
                col_a = &mut model.color.red;
                col_b = &mut model.color.green;
            }
            _ => {
                model.step = 0;
                return;
            }
        }

        *col_a += transition_frame;
        *col_b -= transition_frame;

        if *col_a >= 1. {
            *col_a = 1.;
            model.step += 1;
        }
    }
}

// Procedural version of the color transition.
// According to the current transition phase (step).
// The correct pair of color changes.
//
#[allow(dead_code)]
fn update_explicit(model: &mut Model) {
    let mut blue = model.color.blue;
    let mut green = model.color.green;
    let mut red = model.color.red;
    // 1 / 255 = 0.00392... So better follow a "speed by frame" approach.
    let transition_frame: f32 = 0.0025;

    if model.step == 0 {
        blue += transition_frame;
        red -= transition_frame;

        if blue >= 1. {
            blue = 1.;

            model.step = 1;
        }
    }

    if model.step == 1 {
        green += transition_frame;
        blue -= transition_frame;

        if green >= 1. {
            green = 1.;

            model.step = 2;
        }
    }
    if model.step == 2 {
        red += transition_frame;
        green -= transition_frame;

        if red >= 1. {
            red = 1.;

            model.step = 3;
        }
    }

    if model.step >= 3 {
        model.step = 0;
    }

    model.color = color::Rgb::new(red, green, blue)
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);
    let draw = app.draw();

    draw.background().color(model.color);

    draw.to_frame(app, &frame).unwrap();
}
