use std::mem::needs_drop;

const N: usize = 9;

#[derive(Copy, Clone)]
struct Cell {
    x: usize,
    y: usize,
}

struct SudokuGameProperties {
    board: [[u8; N]; N],
    cells_empty: Vec<Cell>,
    current_index: usize,
}

fn start(board: [[u8; N]; N], depth: usize) -> Vec<SudokuGameProperties> {
    let mut queue = Vec::new();
    let cells_empty = find_empty_cells(&board);

    let state = SudokuGameProperties {
        board,
        cells_empty,
        current_index: 0,
    };
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

fn new_sudoku_game(board: [[u8; N]; N],
                   cells_empty : Vec<Cell>,
current_index : usize) -> SudokuGameProperties {

    return SudokuGameProperties {
        board,
        cells_empty: cells_empty.clone(),
        current_index: current_index + 1,
    }

}

fn solve_recursive(
    state: SudokuGameProperties,
    max_depth: usize,
    current_depth: usize,
    queue: &mut Vec<SudokuGameProperties>,
) {
    if current_depth == max_depth || state.current_index >= state.cells_empty.len() {
        queue.push(state);
        return;
    }

    let cell = state.cells_empty[state.current_index];

    for i in 1..=N {
        if (is_allowed(&state.board, &cell, i as u8)) {

            let mut new_board = state.board;
            new_board[cell.x][cell.y] = i as u8;

            let new_sudoku_game = new_sudoku_game(new_board,
                                                  state.cells_empty.clone(),
                                                  state.current_index);


            solve_recursive(new_sudoku_game, max_depth, current_depth +1, queue);
        }
    }
}

fn solve(board: &mut [[u8; N]; N], cells_empty: &mut Vec<Cell>, index: usize) -> bool {
    if index == cells_empty.len() {
        return true;
    }

    let cell = cells_empty[index];

    for i in 1..=N {
        if (is_allowed(board, &cell, i as u8)) {
            board[cell.x][cell.y] = i as u8;
            if (solve(board, cells_empty, index + 1)) {
                return true;
            }
            board[cell.x][cell.y] = 0;
        }
    }

    false
}

fn is_in_a_section(board: &[[u8; N]; N], cell: &Cell, num: u8) -> bool {
    let row = cell.x;
    let col = cell.y;

    let module_row = row % 3;
    let module_column = col % 3;

    let start_row = row - module_row;
    let start_column = col - module_column;

    for n in start_row..(start_row + 3) {
        for m in start_column..(start_column + 3) {
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
    !is_in_a_section(board, cell, num)
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
}
