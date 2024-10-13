use std::sync::atomic::{AtomicU32, Ordering};

use nannou::prelude::*;

// Video
// https://www.youtube.com/watch?v=L4u7Zy_b868
//
const WIDTH: u32 = 600;
const HEIGHT: u32 = 600;
// offset is half the window's with/height.
const OFFSET: f32 = 300.;
const SQUARE_WIDTH: u32 = 10; // In pixels

// Coloring
static mut HUE: AtomicU32 = AtomicU32::new(200);

pub fn run() {
    nannou::app(model).update(update).run();
}

type Grid = Vec<Vec<u32>>;
struct Model {
    grid: Grid,
    // pixel width
    w: usize,
}

fn model(app: &App) -> Model {
    // Setup
    let width = WIDTH;
    let height = HEIGHT;
    let w = SQUARE_WIDTH.to_usize().unwrap();

    let cols = width / SQUARE_WIDTH; // "cols = 60"
    let rows = height / SQUARE_WIDTH; // "rows = 60"

    app.new_window()
        .size(width, height)
        .view(view)
        .mouse_pressed(mouse_pressed)
        .mouse_moved(mouse_moved)
        .build()
        .unwrap();

    let grid = make_2d_array(cols.try_into().unwrap(), rows.try_into().unwrap());
    Model { grid, w }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let mut next_generation_grid = make_2d_array(model.grid.len(), model.grid[0].len());
    let grid_limit = model.grid.len();
    for col_i in 0..model.grid.len() {
        let col = model.grid[col_i].clone();
        for row_i in 0..col.len() {
            let row = col[row_i];
            if row >= 1 && row_i > 0 {
                // Sand should look like going down smooth.
                let mut direction: isize = -1;
                if random_range(0., 1.) >= 0.5 {
                    direction = 1;
                }
                let index_a = match_boundaries(col_i.to_isize().unwrap() + direction, grid_limit);
                let lateral_a = model.grid[index_a][row_i - 1];
                let index_b = match_boundaries(col_i.to_isize().unwrap() - direction, grid_limit);
                let lateral_b = model.grid[index_b][row_i - 1];

                if next_generation_grid[col_i][row_i - 1] == 0 {
                    next_generation_grid[col_i][row_i - 1] = row;
                } else if lateral_a == 0 {
                    next_generation_grid[index_a][row_i - 1] = row;
                } else if lateral_b == 0 {
                    next_generation_grid[index_b][row_i - 1] = row;
                } else {
                    next_generation_grid[col_i][row_i] = row;
                }
            } else if row >= 1 {
                next_generation_grid[col_i][row_i] = row;
            }
        }
    }
    model.grid = next_generation_grid;
}

fn match_boundaries(col: isize, limit: usize) -> usize {
    match (col).to_usize().unwrap_or(0) {
        p if p >= limit => limit - 1,
        p => p,
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);

    let draw = app.draw();
    draw.background().color(PLUM);
    // Draw the squares
    for col_i in 0..model.grid.len() {
        let col = model.grid[col_i].clone();
        for row_i in 0..col.len() {
            let row = col[row_i];
            // let fill_color = u32::MAX * row as u32;
            let stroke = WHITE;
            let mut light = 0.5;
            if row == 0 {
                light = 0.;
            }

            let fill_color = hsla(f32::from_u32(row).unwrap() / 3600., 1.0, light, 1.);

            // OFFSET ("300") is half the window's with/height.
            let offset = OFFSET - (model.w as f32 / 2.0);
            draw.rect()
                // .color(rgb_u32(fill_color))
                .color(fill_color)
                .stroke(stroke)
                .stroke_weight(1.0)
                .x_y(
                    (col_i * model.w) as f32 - offset,
                    (row_i * model.w) as f32 - offset,
                )
                .w(model.w as f32)
                .h(model.w as f32);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

fn make_2d_array(cols: usize, rows: usize) -> Vec<Vec<u32>> {
    let mut arr = Vec::with_capacity(cols);
    for _ in 0..cols {
        arr.push(vec![0; rows]);
    }

    arr
}

fn mouse_pressed(app: &App, model: &mut Model, button: MouseButton) {
    match button {
        MouseButton::Left => {}
        _ => return,
    }

    let mouse_position = app.mouse.position();
    update_mouse_grid(model, mouse_position);

    increase_hue();
}

fn mouse_moved(app: &App, model: &mut Model, point: Point2) {
    if app.mouse.buttons.left().is_up() {
        return;
    }
    update_mouse_grid(model, point);

    increase_hue();
}

fn update_mouse_grid(model: &mut Model, point: Point2) {
    let offset = OFFSET;
    let x = constraint_to_grid(point.x, offset);
    let y = constraint_to_grid(point.y, offset);

    let hue: u32 = unsafe { HUE.load(Ordering::Relaxed) };
    model.grid[x][y] = hue.clone();

    unsafe { HUE.store(hue, Ordering::Relaxed) };
}

fn constraint_to_grid(p: f32, offset: f32) -> usize {
    let point = match p {
        p if p > offset => offset - 1.,
        p if p < -offset => -offset + 1.,
        p => p,
    };
    ((point + offset) / (SQUARE_WIDTH).to_f32().unwrap())
        .floor()
        .abs()
        .to_usize()
        .unwrap()
}

fn increase_hue() {
    let hue: u32 = unsafe { HUE.load(Ordering::Relaxed) };

    let mut new_hue = hue + 2;
    if new_hue > 3580 {
        new_hue = 1;
    }
    unsafe { HUE.store(new_hue, Ordering::Relaxed) };
}
