use pyo3::prelude::*;
use rand::Rng;

const ROWS: usize = 6;
const COLUMNS: usize = 7;

#[pyclass]
struct Game {
    board: [[i32; COLUMNS]; ROWS],
    turn: i32,
    moves: Vec<i32>,
}

#[pymethods]
impl Game {
    #[new]
    fn new() -> Self {
        Game {
            board: [[0; COLUMNS]; ROWS],
            turn: 1,
            moves: Vec::new(),
        }
    }

    fn play(&mut self) -> PyResult<Vec<i32>> {
        self.moves.clear();
        self.turn = 1;
        self.board = [[0; COLUMNS]; ROWS];

        while !self.is_full() {
            let column = rand::thread_rng().gen_range(1..=COLUMNS as i32);
            if self.can_play_column(column) {
                self.make_move(column);
                if self.check_win() {
                    self.moves.push(8); // win
                    return Ok(self.moves.clone());
                }
            }
        }

        self.moves.push(9); // Tie / Board full
        Ok(self.moves.clone())
    }

    fn can_play_column(&self, column: i32) -> bool {
        for r in 0..ROWS {
            if self.board[r][column as usize - 1] == 0 {
                return true;
            }
        }
        false
    }

    fn make_move(&mut self, column: i32) {
        for r in (0..ROWS).rev() {
            if self.board[r][column as usize - 1] == 0 {
                self.board[r][column as usize - 1] = self.turn;
                self.moves.push(column);
                break;
            }
        }
        self.turn = 3 - self.turn; // Switch turns after making a move
    }

    fn check_win(&self) -> bool {
        for r in 0..ROWS {
            for c in 0..COLUMNS {
                let player = self.board[r][c];
                if player == 0 {
                    continue;
                }
    
                // Check right
                if c + 3 < COLUMNS
                    && player == self.board[r][c + 1]
                    && player == self.board[r][c + 2]
                    && player == self.board[r][c + 3]
                {
                    return true;
                }
    
                // Check down
                if r + 3 < ROWS
                    && player == self.board[r + 1][c]
                    && player == self.board[r + 2][c]
                    && player == self.board[r + 3][c]
                {
                    return true;
                }
    
                // Check down-right
                if r + 3 < ROWS
                    && c + 3 < COLUMNS
                    && player == self.board[r + 1][c + 1]
                    && player == self.board[r + 2][c + 2]
                    && player == self.board[r + 3][c + 3]
                {
                    return true;
                }
    
                // Check down-left
                if r + 3 < ROWS
                    && c >= 3
                    && player == self.board[r + 1][c - 1]
                    && player == self.board[r + 2][c - 2]
                    && player == self.board[r + 3][c - 3]
                {
                    return true;
                }
            }
        }
        false
    }

    fn is_full(&self) -> bool {
        for c in 0..COLUMNS {
            if self.can_play_column(c as i32 + 1) {
                return false;
            }
        }
        true
    }
}

#[pyclass]
struct GameIterator {
    game: Game,
}

#[pymethods]
impl GameIterator {
    #[new]
    fn new() -> Self {
        GameIterator { game: Game::new() }
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<Vec<i32>> {
        let moves = slf.game.play().ok()?;
        if moves.is_empty() {
            None
        } else {
            Some(moves)
        }
    }
}

#[pymodule]
fn connect4(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<GameIterator>()?;
    Ok(())
}
