use std::collections::{HashSet, VecDeque};
use std::sync::{Arc, Mutex, mpsc};
use std::time::Instant;
use std::{fs, io, thread};

const N: usize = 9;
const N_ROOT: usize = 3;

#[derive(Clone)]
struct CellPossibilities {
    x: usize,
    y: usize,
    possibles_values: HashSet<usize>,
}

struct Cell {
    x: usize,
    y: usize,
}

#[derive(Clone)]
struct SudokuGameProperties {
    board: [[usize; N]; N],
    cells_possibilities: Vec<CellPossibilities>,
}

fn reduce(
    board: &mut [[usize; N]; N],
    possibilities: &mut Vec<CellPossibilities>,
    placed_cell: &Cell,
    value: usize,
) {
    board[placed_cell.x][placed_cell.y] = value;

    let mut i = 0;
    while i < possibilities.len() {
        let cell = &possibilities[i];
        if cell.x == placed_cell.x && cell.y == placed_cell.y {
            possibilities.remove(i);
        } else {
            i += 1;
        }
    }

    for i in 0..possibilities.len() {
        let cell = &mut possibilities[i];
        let target_cell = Cell {
            x: cell.x,
            y: cell.y,
        };

        if affects_cell(placed_cell, &target_cell) {
            cell.possibles_values.remove(&value);
        }
    }
}

fn affects_cell(placed: &Cell, target: &Cell) -> bool {
    if placed.x == target.x || placed.y == target.y {
        return true;
    }

    let placed_block_x = placed.x / N_ROOT;
    let placed_block_y = placed.y / N_ROOT;
    let target_block_x = target.x / N_ROOT;
    let target_block_y = target.y / N_ROOT;

    placed_block_x == target_block_x && placed_block_y == target_block_y
}

fn find_all_possibilities_by_cell(board: [[usize; N]; N]) -> Vec<CellPossibilities> {
    let mut cells = Vec::new();

    for i in 0..N {
        for j in 0..N {
            if board[i][j] == 0 {
                let mut values = HashSet::new();
                let cell = Cell { x: i, y: j };

                for number in 1..=N {
                    if is_allowed(&board, &cell, number) {
                        values.insert(number);
                    }
                }

                let possibilities = CellPossibilities {
                    x: i,
                    y: j,
                    possibles_values: values,
                };

                cells.push(possibilities);
            }
        }
    }
    return cells;
}

fn find_min_cell(possibilities: &[CellPossibilities]) -> Option<usize> {
    let mut min = 9999999;
    let mut index = None;

    for i in 0..possibilities.len() {
        let cell = &possibilities[i];

        if !cell.possibles_values.is_empty() && cell.possibles_values.len() < min {
            min = cell.possibles_values.len();
            index = Some(i);
        }
    }

    index
}

fn generate_work(
    state: SudokuGameProperties,
    depth: usize,
    current: usize,
    queue: &mut Vec<SudokuGameProperties>,
) {
    if current >= depth || state.cells_possibilities.is_empty() {
        queue.push(state);
        return;
    }

    if let Some(best_index) = find_min_cell(&state.cells_possibilities) {
        let cell = state.cells_possibilities[best_index].clone();

        for &value in &cell.possibles_values {
            let mut new_state = state.clone();

            let cell = Cell {
                x: cell.x,
                y: cell.y,
            };

            reduce(
                &mut new_state.board,
                &mut new_state.cells_possibilities,
                &cell,
                value,
            );
            generate_work(new_state, depth, current + 1, queue);
        }
    }
}

fn solve_state(mut state: SudokuGameProperties) -> Option<[[usize; N]; N]> {
    if state.cells_possibilities.is_empty() {
        return Some(state.board);
    }

    if let Some(best_index) = find_min_cell(&state.cells_possibilities) {
        let cell = state.cells_possibilities[best_index].clone();

        for &value in &cell.possibles_values {
            let mut new_state = state.clone();

            let cell = Cell {
                x: cell.x,
                y: cell.y,
            };

            reduce(
                &mut new_state.board,
                &mut new_state.cells_possibilities,
                &cell,
                value,
            );

            if let Some(solution) = solve_state(new_state) {
                return Some(solution);
            }
        }
    }

    None
}

fn solve_parallel(board: [[usize; N]; N], num_threads: usize) -> Option<[[usize; N]; N]> {
    let cells = find_all_possibilities_by_cell(board);

    let initial_state = SudokuGameProperties {
        board,
        cells_possibilities: cells,
    };

    let mut work_queue = Vec::new();
    generate_work(initial_state, 3, 0, &mut work_queue);

    if work_queue.is_empty() {
        return None;
    }

    let queue = Arc::new(Mutex::new(VecDeque::from(work_queue)));
    let solution = Arc::new(Mutex::new(None));
    let stop_flag = Arc::new(Mutex::new(false));
    let (tx, rx) = mpsc::channel();

    let mut handles = Vec::new();
    for thread_id in 0..num_threads {
        let queue_clone = queue.clone();
        let solution_clone = solution.clone();
        let stop_clone = stop_flag.clone();
        let tx_clone = tx.clone();

        let handle = thread::spawn(move || {
            loop {
                if *stop_clone.lock().unwrap() {
                    break;
                }

                let work = {
                    let mut q = queue_clone.lock().unwrap();
                    q.pop_front()
                };

                match work {
                    Some(state) => {
                        if let Some(result) = solve_state(state) {
                            *solution_clone.lock().unwrap() = Some(result);
                            *stop_clone.lock().unwrap() = true;
                            tx_clone.send(thread_id).unwrap();
                            break;
                        }
                    }
                    None => {
                        std::thread::sleep(std::time::Duration::from_millis(1));
                    }
                }
            }
        });
        handles.push(handle);
    }

    drop(tx);

    if rx.recv().is_ok() {
        *stop_flag.lock().unwrap() = true;
    }

    for handle in handles {
        handle.join().unwrap();
    }

    solution.lock().unwrap().clone()
}
fn is_in_a_section(board: &[[usize; N]; N], cell: &Cell, num: usize) -> bool {
    let row = cell.x;
    let col = cell.y;

    let start_row = (row / N_ROOT) * N_ROOT;
    let start_column = (col / N_ROOT) * N_ROOT;

    for n in start_row..(start_row + N_ROOT) {
        for m in start_column..(start_column + N_ROOT) {
            if board[n][m] == num {
                return true;
            }
        }
    }
    false
}

fn is_allowed(board: &[[usize; N]; N], cell: &Cell, num: usize) -> bool {
    if num == 0 {
        return false;
    }

    let row = cell.x;
    let col = cell.y;

    for n in 0..N {
        if board[n][col] == num || board[row][n] == num {
            return false;
        }
    }
    !is_in_a_section(board, cell, num)
}

fn load_sudoku_from_file(filename: &str) -> io::Result<[[usize; N]; N]> {
    let content = fs::read_to_string(filename)?;
    let mut board = [[0; N]; N];
    let mut row_index = 0;

    for line in content.lines() {
        if row_index >= N {
            break;
        }

        let mut numbers = Vec::new();
        for word in line.split_whitespace() {
            numbers.push(word);
        }

        let mut col_index = 0;
        for num_str in numbers {
            if row_index >= N {
                break;
            }

            match num_str.parse::<usize>() {
                Ok(num) => {
                    if num <= N {
                        board[row_index][col_index] = num;
                    }
                }
                Err(_) => {
                    board[row_index][col_index] = 0;
                }
            }
            col_index += 1;
        }
        row_index += 1;
    }

    Ok(board)
}
fn main() {
    let board = match load_sudoku_from_file("sudoku_file_matrix.txt") {
        Ok(loaded_board) => {
            println!("Sudoku cargado desde archivo");
            loaded_board
        }
        Err(e) => {
            eprintln!("Error al cargar archivo: {}", e);
            return; // O usar un sudoku por defecto
        }
    };

    let num_threads = thread::available_parallelism()
        .map(|p| p.get())
        .unwrap_or(4);

    let start = Instant::now();
    if let Some(solution) = solve_parallel(board, num_threads) {
        let duration = start.elapsed();
        println!(
            "Solución encontrada en {:?} con {} threads",
            duration, num_threads
        );

        for row in solution {
            println!("{:?}", row);
        }
    } else {
        println!("No se encontró solución");
    }
}
