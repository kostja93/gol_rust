use rand::Rng;

fn main() {
    println!("Hello, world!");
    let board = Board::random();
    board.print();
}

#[derive(Debug, PartialEq, Clone)]
struct Cell {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Board {
    cells: Vec<Cell>,
}

impl Board {
    fn random() -> Board {
        let mut rng = rand::thread_rng();
        let row = [0,1,2];
        let col = [0,1,2];
        let mut cells: Vec<Cell> = Vec::new();

        for x in row {
            for y in col {
                let cell = Cell {x: x, y: y};
                let add_cell: bool = rng.gen();
                if add_cell {
                    cells.push(cell);
                }
            }
        }

        Board {cells: cells}
    }

    fn neighbours(&self, cell: &Cell) -> Vec<Cell> {
        self.cells.iter().filter(|c| {
            let diff_x = (c.x - cell.x).abs();
            let diff_y = (c.y - cell.y).abs();
            !(diff_x == 0 && diff_y == 0) && diff_x <= 1 && diff_y <= 1
        }).map(|c| c.clone()).collect()
    }

    fn ranges(&self) -> [Cell; 2] {
        let min_x = self.cells.iter().map(|c| c.x).min().unwrap_or(0);
        let min_y = self.cells.iter().map(|c| c.y).min().unwrap_or(0);
        let max_x = self.cells.iter().map(|c| c.x).max().unwrap_or(0);
        let max_y = self.cells.iter().map(|c| c.y).max().unwrap_or(0);

        [
            Cell{x: min_x, y: min_y},
            Cell{x: max_x, y: max_y}
        ]
    }

    fn print(&self) {
        print!("\x1B[2J\x1B[1;1H");
        println!("");

        let ranges = self.ranges();
        let n = (ranges[1].x - ranges[0].x).abs() as usize;
        let m = (ranges[1].y - ranges[0].y).abs() as usize;

        let mut array: Vec<Vec<bool>> = vec![vec![false; m + 1]; n + 1];

        for cell in &self.cells {
            array[cell.x as usize][cell.y as usize] = true;
        }
        // Print the initialized array
        for row in &array {
            print!(" ");
            for col in row {
                if *col {
                    print!("*");
                } else {
                    print!(".");
                }
            }
            println!("");
        }

        println!("");
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighbours_of_cell() {
        let cell  = Cell { x: 1, y: 2 };
        let cells = vec![Cell{x: 4, y: 9}, Cell{x: 1, y: 1}, Cell {x: 1, y: 2}];
        let board = Board { cells: cells };
        let expected_cells = vec![Cell{x: 1, y: 1}];

        assert_eq!(expected_cells, board.neighbours(&cell));
    }

    #[test]
    fn is_underpopulated() {
        let cell  = Cell { x: 1, y: 2 };
        let cells = vec![Cell{x: 1, y: 1}, Cell {x: 1, y: 2}];
        let board = Board { cells: cells };

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
        let board = Board { cells: cells };

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
        let board = Board { cells: cells };

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
        let board = Board { cells: cells };

        assert_eq!(true, cell.reproduce(&board));
    }

    #[test]
    fn generate_random_board() {
        let board = Board::random();

        dbg!(&board);
        assert_eq!([Cell{x: 0, y: 0}, Cell{x: 2, y: 2}], board.ranges());
    }
}
