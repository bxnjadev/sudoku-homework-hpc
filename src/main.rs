const N: usize = 9;

#[derive(Copy, Clone)]
struct Cell {
    x: usize,
    y: usize,
}

fn find_empty_cells(board: &[[u8; N]; N]) -> Vec<Cell> {
    let mut cells = Vec::new();

    for i in 0..N {
        for j in 0..N {
            if board[i][j] == 0 {
                cells.push(Cell { x: i, y: j });
            }
        }
    }
    cells
}

fn solve(board: &mut [[u8; N]; N],
         cells_empty: &mut Vec<Cell>,
         index : usize) -> bool {

    if  index == cells_empty.len() {
        return true;
    }

    let cell = cells_empty[index];

    for i in 1..=N {
        if(is_allowed(board, &cell, i as u8)) {
            board[cell.x][cell.y] = i as u8;
            if (solve(board, cells_empty, index + 1)) {
                return true;
            }
            board[cell.x][cell.y] = 0;
        }
    }

    false
}

fn main() {
    let board: [[u8; N]; N] = [
        [1, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];

    //let response = is_allowed(&board, 1, 4, 1);
    //println!("{}", response);
}

fn is_in_a_section(board: &[[u8; N]; N], cell: &Cell, num: u8) -> bool {
    let row = cell.x;
    let col = cell.y;

    let module_row = row % 3;
    let module_column = col % 3;

    let start_row = row - module_row;
    let start_column = col - module_column;

    for n in start_row..2 {
        for m in start_column..2 {
            if board[n][m] == num {
                return true;
            }
        }
    }
    false
}

fn is_allowed(board: &[[u8; N]; N], cell: &Cell, num: u8) -> bool {
    let row = cell.x;
    let col = cell.y;

    for n in 0..N {
        if board[n][col] == num || board[row][n] == num {
            return false;
        }
    }
    is_in_a_section(board, cell, num)
}



