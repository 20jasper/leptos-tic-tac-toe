use std::fmt::Display;

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq)]
pub enum Token {
    X,
    O,
    #[default]
    Empty,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Token::X => "X",
            Token::O => "O",
            Token::Empty => "Empty",
        };

        write!(f, "{}", s)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Outcome {
    Win,
    Draw,
    Continue,
}

pub fn get_turn_outcome(board: &[Token]) -> Outcome {
    let count = board
        .iter()
        .filter(|t| matches!(t, Token::Empty))
        .count();

    println!("count: {}", count);
    let is_win = |tokens: &[Token]| {
        tokens
            .iter()
            .all(|t| *t != Token::Empty && t == &tokens[0])
    };

    let row_win = board.chunks(3).any(is_win);

    let col_win = (0..3)
        .map(|i| [board[i], board[i + 3], board[i + 6]])
        .any(|col| is_win(&col));

    let get_diag = |start: usize, diff| {
        (0..3)
            .scan(start, move |state, _| {
                let tmp = *state;
                *state += diff;
                Some(tmp)
            })
            .map(|i| board[i])
    };

    let left_diag = get_diag(0, 4).collect::<Vec<_>>();
    let right_diag = get_diag(2, 2).collect::<Vec<_>>();

    let diag_win = is_win(&left_diag) || is_win(&right_diag);

    if row_win || col_win || diag_win {
        return Outcome::Win;
    }
    if count == 0 {
        return Outcome::Draw;
    }
    Outcome::Continue
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_winner_with_full_board_should_be_draw() {
        let board = [
            [Token::X, Token::O, Token::X],
            [Token::X, Token::O, Token::X],
            [Token::O, Token::X, Token::O],
        ]
        .flatten();

        assert_eq!(get_turn_outcome(board), Outcome::Draw)
    }

    #[test]
    fn top_row_should_win() {
        let board = [
            [Token::X, Token::X, Token::X],
            [Token::Empty, Token::Empty, Token::Empty],
            [Token::Empty, Token::Empty, Token::Empty],
        ]
        .flatten();

        assert_eq!(get_turn_outcome(board), Outcome::Win)
    }

    #[test]
    fn middle_row_win() {
        let board = [
            [Token::Empty, Token::Empty, Token::Empty],
            [Token::O, Token::O, Token::O],
            [Token::Empty, Token::Empty, Token::Empty],
        ]
        .flatten();

        assert_eq!(get_turn_outcome(board), Outcome::Win)
    }
    #[test]
    fn left_diagonal_win() {
        let board = [
            [Token::X, Token::Empty, Token::Empty],
            [Token::Empty, Token::X, Token::Empty],
            [Token::Empty, Token::Empty, Token::X],
        ]
        .flatten();

        assert_eq!(get_turn_outcome(board), Outcome::Win)
    }

    #[test]
    fn right_diagonal_win() {
        let board = [
            [Token::Empty, Token::Empty, Token::X],
            [Token::Empty, Token::X, Token::Empty],
            [Token::X, Token::Empty, Token::Empty],
        ]
        .flatten();

        assert_eq!(get_turn_outcome(board), Outcome::Win)
    }

    #[test]
    fn vertical_win() {
        let board = [
            [Token::X, Token::Empty, Token::Empty],
            [Token::X, Token::Empty, Token::Empty],
            [Token::X, Token::Empty, Token::Empty],
        ]
        .flatten();

        assert_eq!(get_turn_outcome(board), Outcome::Win)
    }
    #[test]
    fn no_winner_with_incomplete_board_should_continue() {
        let board = [
            [Token::X, Token::O, Token::X],
            [Token::O, Token::X, Token::O],
            [Token::Empty, Token::Empty, Token::Empty],
        ]
        .flatten();

        assert_eq!(get_turn_outcome(board), Outcome::Continue)
    }
}
