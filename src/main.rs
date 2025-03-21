use std::{fmt::Display, thread::sleep, time::Duration};

use color_print::cwrite;

const GRID_SIZE: usize = 61;
#[derive(Clone, Copy)]
struct Cell {
    drunkards: f32,
    police_eff: f32,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.police_eff > 0.0 {
            cwrite!(f, "<bg:#0000aa>{:<4.1}</bg:#0000aa>", self.police_eff)?
        } else if self.drunkards > 1.0 / (GRID_SIZE * GRID_SIZE) as f32 {
            cwrite!(f, "<#770000>{:<4.0}</#770000>", self.drunkards * 10000.0)?
        } else {
            cwrite!(f, "{:<4.0}", self.drunkards * 10000.0)?
        }
        Ok(())
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            drunkards: 0.0,
            police_eff: 0.0,
        }
    }
}

struct Grid {
    cells: [[Cell; GRID_SIZE]; GRID_SIZE],
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.cells {
            for col in row {
                cwrite!(f, "{}", col)?
            }
            cwrite!(f, "\n")?
        }
        Ok(())
    }
}

impl Default for Grid {
    fn default() -> Self {
        Grid {
            cells: [[Cell::default(); GRID_SIZE]; GRID_SIZE],
        }
    }
}
impl Grid {
    fn add_police<T: rand::Rng>(&mut self, rng: &mut T, probability: f64, efficiency: f32) {
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                if rng.random::<f64>() < probability {
                    self.cells[row][col].police_eff = efficiency;
                }
            }
        }
    }

    fn add_drunkards(&mut self, x_coordinate: usize, y_coordinate: usize, amount: f32) {
        self.cells[y_coordinate][x_coordinate].drunkards = amount
    }

    fn do_iteration(&mut self) -> f64 {
        // I don't like, but otherwise it's annoying AF
        let mut new = self.cells.clone();
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                let current = self.cells[row][col];
                let move_weights =
                    match (row > 0, row < GRID_SIZE - 1, col > 0, col < GRID_SIZE - 1) {
                        (true, true, true, true) => (5, 5, 5, 5, 5),
                        (true, true, true, false) => (4, 4, 4, 0, 4),
                        (true, true, false, true) => (4, 4, 0, 4, 4),
                        (true, false, true, true) => (4, 0, 4, 4, 4),
                        (false, true, true, true) => (0, 4, 4, 4, 4),
                        (true, true, false, false) => (3, 3, 0, 0, 3),
                        (true, false, true, false) => (3, 0, 3, 0, 3),
                        (true, false, false, true) => (3, 0, 0, 3, 3),
                        (false, true, true, false) => (0, 3, 3, 0, 3),
                        (false, true, false, true) => (0, 3, 0, 3, 3),
                        (false, false, true, true) => (0, 0, 3, 3, 3),
                        _ => unreachable!("grid too small"),
                    };
                new[row][col].drunkards -= current.drunkards * (1.0 - 1.0 / move_weights.4 as f32);
                if move_weights.0 > 0 {
                    new[row - 1][col].drunkards += current.drunkards / move_weights.0 as f32
                }
                if move_weights.1 > 0 {
                    new[row + 1][col].drunkards += current.drunkards / move_weights.1 as f32
                }
                if move_weights.2 > 0 {
                    new[row][col - 1].drunkards += current.drunkards / move_weights.2 as f32
                }
                if move_weights.3 > 0 {
                    new[row][col + 1].drunkards += current.drunkards / move_weights.3 as f32
                }
            }
        }
        let mut acc = 0.0;

        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                new[row][col].drunkards *= 1.0 - new[row][col].police_eff;
                acc += new[row][col].drunkards;
            }
        }
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                new[row][col].drunkards /= acc;
            }
        }
        self.cells = new;
        acc as f64
    }
}

fn main() {
    let mut rngen = rand::rng();

    let mut grid = Grid::default();
    let mut scaling = 1.0;
    let mut iter = 0;
    grid.add_police(&mut rngen, 0.1, 0.3);
    grid.add_drunkards(GRID_SIZE / 2, GRID_SIZE / 2, 1.0);
    loop {
        println!("{}", grid);
        sleep(Duration::from_millis(100));
        for _ in 0..50 {
            let scale = grid.do_iteration();
            scaling *= scale;
            iter += 1;
        }
        println!("at step {}, scaled {} in total", iter, 1.0 / scaling)
    }
}
