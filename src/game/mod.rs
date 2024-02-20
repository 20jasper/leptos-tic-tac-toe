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
    let row_win = board.chunks(3).any(|row| {
        row.iter()
            .all(|t| *t != Token::Empty && t == &row[0])
    });

    if row_win {
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
    fn no_winner_should_be_draw() {
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
    fn middle_row_should_win() {
        let board = [
            [Token::Empty, Token::Empty, Token::Empty],
            [Token::O, Token::O, Token::O],
            [Token::Empty, Token::Empty, Token::Empty],
        ]
        .flatten();

        assert_eq!(get_turn_outcome(board), Outcome::Win)
    }
}
