use pyo3::prelude::*;
use rand::Rng;

const ROWS: usize = 6;
const COLUMNS: usize = 7;

#[pymodule]
fn connect4(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(play_game, m)?)?;
    Ok(())
}

#[pyfunction]
fn play_game(desired_length: usize) -> PyResult<Vec<i32>> {
    let mut board = [[0; COLUMNS]; ROWS];
    let mut turn = 1;
    let mut moves = Vec::new();

    while !is_board_full(&board) && moves.len() < desired_length {
        let column = rand::thread_rng().gen_range(1..=COLUMNS as i32);
        // check if the column is full
        if board[0][column as usize - 1] != 0 {
            continue;
        }
        moves.push(column);
        if make_move(&mut board, column, turn) {
            moves.push(8); // Win
            break;
        }
        turn = 3 - turn; // Switch turns
    }

    // if the game is not over (check if last move is 8)
    if moves.len() < desired_length && moves[moves.len() - 1] != 8 {
        moves.push(9); // Tie or end of game
    }

    // Pad the moves vector with zeros if it's shorter than the desired length
    while moves.len() < desired_length {
        moves.push(0);
    }

    Ok(moves)
}
fn make_move(board: &mut [[i32; COLUMNS]; ROWS], column: i32, player: i32) -> bool {
    for row in (0..ROWS).rev() {
        if board[row][column as usize - 1] == 0 {
            board[row][column as usize - 1] = player;
            return check_win(board, row, column as usize - 1, player);
        }
    }
    false // Column is full
}

fn check_win(board: &[[i32; COLUMNS]; ROWS], row: usize, col: usize, player: i32) -> bool {
    // Check horizontal
    let count = count_consecutive(board, row, col, 0, 1, player) + count_consecutive(board, row, col, 0, -1, player) - 1;
    if count >= 4 { return true; }

    // Check vertical
    let count = count_consecutive(board, row, col, 1, 0, player);
    if count >= 4 { return true; }

    // Check diagonals
    let count = count_consecutive(board, row, col, 1, 1, player) + count_consecutive(board, row, col, -1, -1, player) - 1;
    if count >= 4 { return true; }
    let count = count_consecutive(board, row, col, 1, -1, player) + count_consecutive(board, row, col, -1, 1, player) - 1;
    if count >= 4 { return true; }

    false 
}

fn count_consecutive(board: &[[i32; COLUMNS]; ROWS], row: usize, col: usize, dr: isize, dc: isize, player: i32) -> usize {
    let mut r = row as isize;
    let mut c = col as isize;
    let mut count = 0;


    while r >= 0 && r < ROWS as isize && c >= 0 && c < COLUMNS as isize && board[r as usize][c as usize] == player {
        count += 1;
        if r == 0 && dr == -1 {
            break;
        }
        r += dr;
        if c == 0 && dc == -1 {
            break;
        }
        c += dc;
    }

    count
}


fn is_board_full(board: &[[i32; COLUMNS]; ROWS]) -> bool {
    board[0].iter().all(|&cell| cell != 0) // Check if the top row is full
}


