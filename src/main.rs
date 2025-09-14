
const N: usize = 9;

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

    let response = is_allowed(&board, 1, 4, 1);
    println!("{}", response);

}

fn is_in_a_section(
    board : &[[u8; N]; N],
    row: usize,
    col: usize,
    num: u8
) -> bool {

    let module_row = row % 3;
    let module_column  = col % 3;

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

fn is_allowed(
    board : &[[u8; N]; N],
    row: usize,
    col: usize,
    num: u8
) -> bool {


    for n in 0..N {
        println!("{}", board[n][col]);
        println!("{}", board[row][n]);
        if board[n][col] == num || board[row][n] == num {
            return false;
        }
    }

    if is_in_a_section(board, row, col, num) {
        return false;
    }

    true
}
