use rand::Rng;
use std::thread;
use std::time::Duration;
use crossterm::terminal::{self, ClearType};
use std::io;

fn get_console_size() -> Result<(u16, u16), io::Error> {
    terminal::enable_raw_mode()?;
    let size = terminal::size()?;
    terminal::disable_raw_mode()?;
    Ok(size)
}

fn main() {
    println!("Hello, world!");
    terminal::enable_raw_mode().unwrap();
    let (width, height) = get_console_size().unwrap();
    terminal::disable_raw_mode().unwrap();
    let mut board = Board::random(width as i32, height as i32);
    let mut count: i32 = 0;
    loop {
        board = Board::next_board(&board);
        println!("NEXT GENERATION {}", count);
        board.print();
        thread::sleep(Duration::from_secs(1));
        count += 1;
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Cell {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Board {
    cells: Vec<Cell>,
    x: i32,
    y: i32,
}

impl Board {
    fn random(rows: i32, cols: i32) -> Board {
        let mut rng = rand::thread_rng();
        let mut cells: Vec<Cell> = Vec::new();

        for x in 0..cols {
            for y in 0..rows {
                let cell = Cell {x: x, y: y};
                let add_cell: bool = rng.gen();
                if add_cell {
                    cells.push(cell);
                }
            }
        }

        Board {cells: cells, x: rows, y: cols}
    }

    fn next_board(board: &Board) -> Board {
        let mut cells: Vec<Cell> = Vec::new();

        println!("{}, {}", board.x, board.y);
        for x in 0..board.x {
            for y in 0..board.y {
                let cell = Cell {x: x, y: y};
                let add_cell: bool = cell.in_next_gen(board);
                if add_cell {
                    cells.push(cell);
                }
            }
        }

        Board {cells: cells, x: board.x, y: board.y}
    }

    fn neighbours(&self, cell: &Cell) -> Vec<Cell> {
        self.cells.iter().filter(|c| {
            let diff_x = (c.x - cell.x).abs();
            let diff_y = (c.y - cell.y).abs();
            !(diff_x == 0 && diff_y == 0) && diff_x <= 1 && diff_y <= 1
        }).map(|c| c.clone()).collect()
    }

    fn print(&self) {
        print!("\x1B[2J\x1B[1;1H");
        println!("");

        let mut array: Vec<Vec<bool>> = vec![vec![false; self.y as usize]; self.x as usize];

        for cell in &self.cells {
            array[cell.y as usize][cell.x as usize] = true;
        }

        for row in &array {
            for col in row {
                if *col {
                    print!("*");
                } else {
                    print!(" ");
                }
            }
            println!("");
        }
    }
}

impl Cell {
    fn underpopulated(&self, board: &Board) -> bool {
        board.neighbours(self).len() < 2
    }

    fn overpopulated(&self, board: &Board) -> bool {
        board.neighbours(self).len() > 3
    }

    fn stays_alive(&self, board: &Board) -> bool {
        vec![2,3].contains(&(board.neighbours(self).len() as i32))
    }

    fn reproduce(&self, board: &Board) -> bool {
        board.neighbours(self).len() == 3
    }

    fn in_next_gen(&self, board: &Board) -> bool {
        !self.underpopulated(board) && !self.overpopulated(board) &&
            (self.stays_alive(board) || self.reproduce(board))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighbours_of_cell() {
        let cell  = Cell { x: 1, y: 2 };
        let cells = vec![Cell{x: 4, y: 9}, Cell{x: 1, y: 1}, Cell {x: 1, y: 2}];
        let board = Board { cells: cells, x: 10, y: 10 };
        let expected_cells = vec![Cell{x: 1, y: 1}];

        assert_eq!(expected_cells, board.neighbours(&cell));
    }

    #[test]
    fn is_underpopulated() {
        let cell  = Cell { x: 1, y: 2 };
        let cells = vec![Cell{x: 1, y: 1}, Cell {x: 1, y: 2}];
        let board = Board { cells: cells, x: 10, y: 10 };

        assert_eq!(true, cell.underpopulated(&board));
    }

    #[test]
    fn is_overpopulated() {
        let cell  = Cell { x: 1, y: 2 };
        let cells = vec![
            Cell{x: 1, y: 1},
            Cell{x: 1, y: 2},
            Cell{x: 1, y: 3},
            Cell{x: 2, y: 2},
            Cell{x: 0, y: 2},
        ];
        let board = Board { cells: cells, x: 10, y: 10 };

        assert_eq!(true, cell.overpopulated(&board));
    }

    #[test]
    fn stays_alive() {
        let cell  = Cell { x: 1, y: 2 };
        let cells = vec![
            Cell{x: 1, y: 2},
            Cell{x: 1, y: 1},
            Cell{x: 1, y: 3},
            Cell{x: 2, y: 2},
        ];
        let board = Board { cells: cells, x: 10, y: 10 };

        assert_eq!(true, cell.stays_alive(&board));
    }

    #[test]
    fn is_reborn() {
        let cell  = Cell { x: 1, y: 2 };
        let cells = vec![
            Cell{x: 1, y: 2},
            Cell{x: 1, y: 1},
            Cell{x: 1, y: 3},
            Cell{x: 2, y: 2},
        ];
        let board = Board { cells: cells, x: 10, y: 10 };

        assert_eq!(true, cell.reproduce(&board));
    }
}
