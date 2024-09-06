use super::game::Player;

pub fn default_win_conditions<const COLUMNS: usize, const ROWS: usize>(
) -> Vec<Box<dyn WinCondition<COLUMNS, ROWS>>> {
    vec![
        VerticalWinCondition::boxed(),
        HorizontalWinCondition::boxed(),
        DiagonalWinCondition::boxed(),
        ReverseDiagonalWinCondition::boxed(),
    ]
}

pub trait WinCondition<const COLUMNS: usize, const ROWS: usize> {
    fn is_met(&self, board: &[[Player; ROWS]; COLUMNS], column: usize, row: usize) -> bool;
}

pub struct VerticalWinCondition {}

impl VerticalWinCondition {
    pub fn boxed() -> Box<Self> {
        Box::new(Self {})
    }
}

impl<const COLUMNS: usize, const ROWS: usize> WinCondition<COLUMNS, ROWS> for VerticalWinCondition {
    fn is_met(&self, board: &[[Player; ROWS]; COLUMNS], column: usize, row: usize) -> bool {
        row + 3 < board[column].len()
            && board[column][row] != Player::None
            && board[column][row] == board[column][row + 1]
            && board[column][row] == board[column][row + 2]
            && board[column][row] == board[column][row + 3]
    }
}

#[derive(Debug, Clone)]
pub struct HorizontalWinCondition {}

impl HorizontalWinCondition {
    pub fn boxed() -> Box<Self> {
        Box::new(Self {})
    }
}

impl<const COLUMNS: usize, const ROWS: usize> WinCondition<COLUMNS, ROWS>
    for HorizontalWinCondition
{
    fn is_met(&self, board: &[[Player; ROWS]; COLUMNS], column: usize, row: usize) -> bool {
        column + 3 < board.len()
            && board[column][row] != Player::None
            && board[column][row] == board[column + 1][row]
            && board[column][row] == board[column + 2][row]
            && board[column][row] == board[column + 3][row]
    }
}

#[derive(Debug, Clone)]
pub struct DiagonalWinCondition {}

impl DiagonalWinCondition {
    pub fn boxed() -> Box<Self> {
        Box::new(Self {})
    }
}

impl<const COLUMNS: usize, const ROWS: usize> WinCondition<COLUMNS, ROWS> for DiagonalWinCondition {
    fn is_met(&self, board: &[[Player; ROWS]; COLUMNS], column: usize, row: usize) -> bool {
        column + 3 < board.len()
            && row + 3 < board[column].len()
            && board[column][row] != Player::None
            && board[column][row] == board[column + 1][row + 1]
            && board[column][row] == board[column + 2][row + 2]
            && board[column][row] == board[column + 3][row + 3]
    }
}

#[derive(Debug, Clone)]
pub struct ReverseDiagonalWinCondition {}

impl ReverseDiagonalWinCondition {
    pub fn boxed() -> Box<Self> {
        Box::new(Self {})
    }
}

impl<const COLUMNS: usize, const ROWS: usize> WinCondition<COLUMNS, ROWS>
    for ReverseDiagonalWinCondition
{
    fn is_met(&self, board: &[[Player; ROWS]; COLUMNS], column: usize, row: usize) -> bool {
        column >= 3
            && row + 3 < board[column].len()
            && board[column][row] != Player::None
            && board[column][row] == board[column - 1][row + 1]
            && board[column][row] == board[column - 2][row + 2]
            && board[column][row] == board[column - 3][row + 3]
    }
}
