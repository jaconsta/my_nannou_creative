use std::isize;

use nannou::prelude::*;

const WIDTH: u32 = 600;
const HEIGHT: u32 = WIDTH;
const MIDDLE: u32 = WIDTH / 2;
const BLOCK_SIZE: u32 = 10; // width and height in pixels

pub fn run() {
    nannou::app(model).update(update).run();
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum GridStatus {
    LIFE,
    NOT,
}

impl GridStatus {
    fn color(&self) -> nannou::color::Rgb8 {
        match self {
            GridStatus::LIFE => WHITE,
            GridStatus::NOT => BLACK,
        }
    }
    fn toggle(&self) -> GridStatus {
        match self {
            GridStatus::LIFE => GridStatus::NOT,
            GridStatus::NOT => GridStatus::LIFE,
        }
    }
    fn grid_value(&self) -> u8 {
        match self {
            GridStatus::LIFE => 1,
            GridStatus::NOT => 0,
        }
    }
    fn game_rules(&self, neighbors: &mut [u8; 9]) -> GridStatus {
        let x_y = neighbors[4];
        neighbors[4] = 0;
        let mut total_neighbors = 0;
        for i in neighbors.iter() {
            if i.gt(&0) {
                total_neighbors += 1;
            }
        }

        if x_y == 1 && total_neighbors < 2 {
            // 1. Any live cell with fewer than two live neighbours dies, as if by underpopulation.
            return GridStatus::NOT;
        } else if x_y == 1 && (total_neighbors == 2 || total_neighbors == 3) {
            // 2. Any live cell with two or three live neighbours lives on to the next generation.
            return GridStatus::LIFE;
        } else if x_y == 1 && total_neighbors > 3 {
            // 3. Any live cell with more than three live neighbours dies, as if by overpopulation.
            return GridStatus::NOT;
        } else if x_y == 0 && total_neighbors == 3 {
            // 4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
            return GridStatus::LIFE;
        }
        return self.clone();
    }
}

#[cfg(test)]
mod test_grid_status {
    use super::*;

    #[derive(Clone, Debug, PartialEq)]
    struct NeighborsTest {
        neighbors: [u8; 9],
        name: String,
        expected: GridStatus,
    }
    #[test]
    fn grid_game_rules() {
        let mut neighbohr_blocks = [
            NeighborsTest {
                neighbors: [0, 0, 0, 0, 0, 0, 0, 0, 0],
                name: "all_dead".to_string(),
                expected: GridStatus::NOT,
            },
            NeighborsTest {
                neighbors: [0, 0, 0, 0, 1, 0, 0, 0, 0],
                name: "all_dead_core_alive".to_string(),
                expected: GridStatus::NOT,
            },
            NeighborsTest {
                neighbors: [1, 0, 0, 0, 1, 0, 0, 0, 0],
                name: "core_alive_rule_1".to_string(),
                expected: GridStatus::NOT,
            },
            NeighborsTest {
                neighbors: [1, 1, 0, 0, 1, 0, 0, 0, 0],
                name: "core_alive_rule_2_with_two".to_string(),
                expected: GridStatus::LIFE,
            },
            NeighborsTest {
                neighbors: [1, 1, 0, 0, 1, 0, 0, 0, 0],
                name: "core_alive_rule_2_with_three".to_string(),
                expected: GridStatus::LIFE,
            },
            NeighborsTest {
                neighbors: [1, 1, 1, 0, 1, 0, 1, 0, 0],
                name: "core_alive_rule_3".to_string(),
                expected: GridStatus::NOT,
            },
            NeighborsTest {
                neighbors: [1, 1, 1, 1, 1, 1, 1, 1, 1],
                name: "core_alive_all_alive".to_string(),
                expected: GridStatus::NOT,
            },
            NeighborsTest {
                neighbors: [1, 1, 0, 0, 0, 0, 1, 0, 0],
                name: "core_alive_rule_4".to_string(),
                expected: GridStatus::LIFE,
            },
            NeighborsTest {
                neighbors: [1, 1, 1, 0, 0, 0, 1, 0, 0],
                name: "core_alive_rule_4_fail".to_string(),
                expected: GridStatus::NOT,
            },
        ];

        for n in neighbohr_blocks.iter_mut() {
            let calculation = GridStatus::NOT.game_rules(&mut n.neighbors);
            assert_eq!(calculation, n.expected, "`{0}` `{calculation:?}`", n.name);
        }
    }
}

type Grid = Vec<Vec<GridStatus>>;

#[derive(Debug, Clone)]
struct Model {
    grid: Grid,
    running: bool,
}

impl Model {
    fn new(grid_width: Option<u32>) -> Self {
        let grid_size = grid_width.unwrap_or_else(|| WIDTH) / BLOCK_SIZE;
        let grid = new_square_grid(GridStatus::NOT, grid_size);
        Self {
            grid,
            running: false,
        }
    }
    fn toggle_mouse_grid(&mut self, point: Point2) {
        let x = constraint_to_grid(point.x);
        let y = constraint_to_grid(point.y);
        self.grid[x][y] = self.grid[x][y].toggle();
    }

    // Get the context of neighbors for xx (bb).  then
    // [[aa, ab, ac] , [ba, bb, bc], [ca, cb, cc]]
    // Ideal plan:
    // Returns an hexadecimal to join the elements
    // 0x...aa + 0x...bb0 + 0x...ac00 ...
    // In the end:
    // Returns an binary array of the adjacent cells
    // 1 = life, 0 = dead
    // [aa, ab, ac, ba, bb, bc, ca, cb, cc]
    fn neighbors(&self, x: usize, y: usize) -> [u8; 9] {
        let mut neighbors = [0; 9];
        neighbors.fill(0);
        let size = self.grid.len() as isize;

        for i in 0..3 {
            let point_x: isize = (x + i) as isize - 1;
            let neighbor_i = i * 3;
            for j in 0..3 {
                let point_y: isize = (y + j) as isize - 1;
                let poss: u8;
                let cell_outside_grid =
                    point_x < 0 || point_x >= size || point_y < 0 || point_y >= size;
                if cell_outside_grid {
                    poss = GridStatus::NOT.grid_value();
                } else {
                    poss = self.grid[point_x as usize][point_y as usize].grid_value();
                }
                neighbors[neighbor_i + j] = poss;
            }
        }
        return neighbors;
    }
}

#[cfg(test)]
mod test_model_operations {
    use super::*;

    #[test]
    fn model_get_neighbors() {
        let model_calc = Model {
            grid: vec![
                vec![
                    GridStatus::NOT,
                    GridStatus::NOT,
                    GridStatus::NOT,
                    GridStatus::NOT,
                    GridStatus::LIFE,
                ],
                vec![
                    GridStatus::NOT,
                    GridStatus::LIFE,
                    GridStatus::LIFE,
                    GridStatus::LIFE,
                    GridStatus::NOT,
                ],
                vec![
                    GridStatus::NOT,
                    GridStatus::LIFE,
                    GridStatus::LIFE,
                    GridStatus::LIFE,
                    GridStatus::NOT,
                ],
                vec![
                    GridStatus::NOT,
                    GridStatus::LIFE,
                    GridStatus::LIFE,
                    GridStatus::LIFE,
                    GridStatus::NOT,
                ],
                vec![
                    GridStatus::NOT,
                    GridStatus::NOT,
                    GridStatus::NOT,
                    GridStatus::NOT,
                    GridStatus::LIFE,
                ],
            ],
            running: false,
        };

        println!("First 2, 2");
        let neigh = model_calc.neighbors(2, 2);
        let model_expected = [1, 1, 1, 1, 1, 1, 1, 1, 1];
        assert_eq!(neigh, model_expected);

        println!("\n Second 0, 0");
        let neigh = model_calc.neighbors(0, 0);
        let model_expected = [0, 0, 0, 0, 0, 0, 0, 0, 1];
        assert_eq!(neigh, model_expected);

        println!("\n Third: 4,4");
        let neigh = model_calc.neighbors(4, 4);
        let model_expected = [1, 0, 0, 0, 1, 0, 0, 0, 0];
        assert_eq!(neigh, model_expected);

        println!("\n Fourth: 5, 5");
        let neigh = model_calc.neighbors(5, 5);
        let model_expected = [1, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(neigh, model_expected);
    }
}

fn new_square_grid<T: Clone + std::fmt::Debug>(initial_values: T, size: u32) -> Vec<Vec<T>> {
    let row: Vec<T> = vec![initial_values.clone(); size as usize];
    vec![row; size as usize]
}

fn model(app: &App) -> Model {
    app.new_window()
        .view(view)
        .size(WIDTH, HEIGHT)
        .mouse_pressed(mouse_pressed)
        .build()
        .unwrap();

    Model::new(None)
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if !model.running {
        return;
    }

    let mut new_grid = new_square_grid(GridStatus::NOT, WIDTH / BLOCK_SIZE);
    for x in 0..model.grid.len() {
        for y in 0..model.grid[x].len() {
            let mut neighbors = model.neighbors(x, y);
            new_grid[x][y] = model.grid[x][y].game_rules(&mut neighbors);
        }
    }

    model.grid = new_grid;
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);

    let draw = app.draw();
    let mut background_color = BLACK;
    let mut cell_stroke = BLACK;
    if !model.running {
        background_color = PLUM;
        cell_stroke = PLUM;
    }
    draw.background().color(background_color);

    let axis_in_origin: f32 = MIDDLE as f32;
    let block_size = BLOCK_SIZE as f32;

    for x in 0..model.grid.len() {
        for y in 0..model.grid[x].len() {
            let cell = &model.grid[x][y];
            let the_pos = |xy: f32| (block_size * xy) - axis_in_origin;
            let pos_x = the_pos(x as f32);
            let pos_y = the_pos(y as f32);

            draw.rect()
                .color(cell.color())
                .stroke(cell_stroke)
                .stroke_weight(1.0)
                .x_y(pos_x, pos_y)
                .w(block_size)
                .h(block_size);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

fn mouse_pressed(app: &App, model: &mut Model, button: MouseButton) {
    if MouseButton::Right == button {
        model.running = !model.running;
        return;
    }
    if MouseButton::Left != button {
        return;
    }

    let mouse_point = app.mouse.position();
    model.toggle_mouse_grid(mouse_point);
}

fn constraint_to_grid(point: f32) -> usize {
    ((point + (MIDDLE as f32)) / (BLOCK_SIZE).to_f32().unwrap())
        .floor()
        .abs()
        .to_usize()
        .unwrap()
}
