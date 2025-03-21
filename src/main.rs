use std::{fmt::Display, thread::sleep, time::Duration};

use color_print::cwrite;

const GRID_SIZE: usize = 30;
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
                let move_weights = (5, 5, 5, 5, 5);
                new[row][col].drunkards -= current.drunkards * (1.0 - 1.0 / move_weights.4 as f32);
                new[(row + GRID_SIZE - 1) % GRID_SIZE][col].drunkards +=
                    current.drunkards / move_weights.0 as f32;
                new[(row + 1) % GRID_SIZE][col].drunkards +=
                    current.drunkards / move_weights.1 as f32;
                new[row][(col + GRID_SIZE - 1) % GRID_SIZE].drunkards +=
                    current.drunkards / move_weights.2 as f32;
                new[row][(col + 1) % GRID_SIZE].drunkards +=
                    current.drunkards / move_weights.3 as f32;
            }
        }
        let mut acc = 0.0;

        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                // if I find a police
                if self.cells[row][col].police_eff <= 0.0 {
                    continue;
                }
                // check the cells around
            }
        }

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
    grid.add_police(&mut rngen, 0.01, 1.0);
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
