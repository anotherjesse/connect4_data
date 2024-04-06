use pyo3::prelude::*;
use rand::seq::SliceRandom;

const ROWS: usize = 6;
const COLUMNS: usize = 7;

#[pyclass]
struct Game {
    board: [[i32; COLUMNS]; ROWS],
    turn: i32,
    moves: String,
    games: i32,
    wins: i32,
    ties: i32,
}

#[pymethods]
impl Game {
    #[new]
    fn new() -> Game {
        Game {
            board: [[0; COLUMNS]; ROWS],
            turn: 1,
            moves: String::new(),
            games: 0,
            wins: 0,
            ties: 0,
        }
    }

    fn play(&mut self) -> PyResult<Vec<i32>> {
        let mut rng = rand::thread_rng();
        let mut columns: Vec<i32> = (1..=COLUMNS as i32).collect();

        if self.moves.len() > 6 {
            columns.shuffle(&mut rng);
            columns.truncate(2);
        }

        let mut result = Vec::new();

        for &c in &columns {
            if self.can_play_column(c) {
                self.make_move(c);
                if self.check_win() {
                    if self.wins < self.ties {
                        result.push(8);
                        self.games += 1;
                        self.wins += 1;
                        if self.games % 1000 == 0 {
                            eprintln!("{} {} {}", self.games, self.wins, self.ties);
                        }
                    }
                } else if self.is_full() {
                    result.push(9);
                    self.games += 1;
                    self.ties += 1;
                    if self.games % 1000 == 0 {
                        eprintln!("{} {} {}", self.games, self.wins, self.ties);
                    }
                } else {
                    let sub_result = self.play()?;
                    result.extend(sub_result);
                }
                self.undo_move(c);
            }
        }

        Ok(result)
    }

 

    fn can_play_column(&self, column: i32) -> bool {
        for r in 0..ROWS {
            if self.board[r][(column - 1) as usize] == 0 {
                return true;
            }
        }
        false
    }

    fn make_move(&mut self, column: i32) {
        for r in (0..ROWS).rev() {
            if self.board[r][(column - 1) as usize] == 0 {
                self.board[r][(column - 1) as usize] = self.turn;
                self.moves.push_str(&column.to_string());
                break;
            }
        }
        self.turn = 3 - self.turn; // Switch turns after making a move
    }

    fn undo_move(&mut self, column: i32) {
        for r in 0..ROWS {
            if self.board[r][(column - 1) as usize] != 0 {
                self.board[r][(column - 1) as usize] = 0;
                self.moves.pop();
                break;
            }
        }
        // No need to switch turns back here as it's handled after each make_move call
    }

    fn check_win(&self) -> bool {
        for r in 0..ROWS {
            for c in 0..COLUMNS {
                let player = self.board[r][c];
                if player == 0 {
                    continue;
                }

                // Check right
                if c + 3 < COLUMNS && (1..=3).all(|i| player == self.board[r][c + i]) {
                    return true;
                }

                // Check downa
                if r + 3 < ROWS && (1..=3).all(|i| player == self.board[r + i][c]) {
                    return true;
                }

                // Check down-right
                if r + 3 < ROWS && c + 3 < COLUMNS && (1..=3).all(|i| player == self.board[r + i][c + i]) {
                    return true;
                }

                // Check down-left
                if r + 3 < ROWS && c >= 3 && (1..=3).all(|i| player == self.board[r + i][c - i]) {
                    return true;
                }
            }
        }
        false
    }

    fn is_full(&self) -> bool {
        !(1..=COLUMNS as i32).any(|c| self.can_play_column(c))
    }
}


#[pymodule]
fn connect_four(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Game>()?;
    Ok(())
}

// fn main() {
//     let mut game = Game::new();
//     game.play();
// }
