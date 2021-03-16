use core::{
    fmt::{self, Display, Formatter},
    ops::{Index, IndexMut},
};

use anyhow::{self, anyhow as anyhow_error};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileState {
    X,
    O,
    Empty,
}

impl From<Player> for TileState {
    fn from(player: Player) -> Self {
        match player {
            Player::X => Self::X,
            Player::O => Self::O,
        }
    }
}

impl Display for TileState {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        write!(
            fmt,
            "{}",
            match self {
                Self::X => "X",
                Self::O => "O",
                Self::Empty => " ",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    X,
    O,
}

impl Player {
    pub fn opponent(&self) -> Self {
        match self {
            Self::X => Self::O,
            Self::O => Self::X,
        }
    }
}

impl Display for Player {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        write!(
            fmt,
            "{}",
            match self {
                Self::X => "X",
                Self::O => "O",
            }
        )
    }
}

impl From<TileState> for Option<Player> {
    fn from(tile: TileState) -> Self {
        match tile {
            TileState::X => Some(Player::X),
            TileState::O => Some(Player::O),
            TileState::Empty => None,
        }
    }
}

pub const BOARD_SIZE: usize = 3;

#[derive(Debug, PartialEq)]
pub struct BoardState {
    tiles: Vec<TileState>,
    next: Player,
}

impl BoardState {
    pub fn new() -> Self {
        BoardState {
            tiles: vec![TileState::Empty; BOARD_SIZE * BOARD_SIZE],
            next: Player::X,
        }
    }

    pub fn play(&mut self, (x, y): (usize, usize)) -> anyhow::Result<&mut Self> {
        if x > BOARD_SIZE || y > BOARD_SIZE {
            return Err(anyhow_error!("({}, {}) is out of bounds", x, y));
        }

        match self[(x, y)] {
            TileState::Empty => {
                self[(x, y)] = self.next.into();
                self.next = self.next.opponent();
                Ok(self)
            }
            _ => Err(anyhow_error!("({}, {}) has already been played", x, y)),
        }
    }

    pub fn next(&self) -> Player {
        self.next
    }

    pub fn iter_row(&self, row: usize) -> impl Iterator<Item = TileState> + '_ {
        (0..BOARD_SIZE).map(move |x| self[(x, row)])
    }

    pub fn iter_col(&self, col: usize) -> impl Iterator<Item = TileState> + '_ {
        (0..BOARD_SIZE).map(move |y| self[(col, y)])
    }

    pub fn iter_diag(&self, sinister: bool) -> impl Iterator<Item = TileState> + '_ {
        (0..BOARD_SIZE).map(move |i| self[(if sinister { BOARD_SIZE - 1 - i } else { i }, i)])
    }

    pub fn won(&self) -> Option<Player> {
        (0..BOARD_SIZE)
            .map(|row| all_eq(self.iter_row(row)))
            .chain((0..BOARD_SIZE).map(|col| all_eq(self.iter_col(col))))
            .chain(
                [false, true]
                    .iter()
                    .map(|&sinister| all_eq(self.iter_diag(sinister))),
            )
            .find_map(|opt_tile| opt_tile.and_then(|tile| tile.into()))
    }

    pub fn drawn(&self) -> bool {
        self.tiles.iter().all(|&tile| tile != TileState::Empty)
    }
}

/// If `iter` is nonempty and all its items are equal, returns an item
fn all_eq<T, I>(mut iter: I) -> Option<T>
where
    I: Iterator<Item = T>,
    T: PartialEq,
{
    iter.next().and_then(|first| {
        if iter.all(|item| item == first) {
            Some(first)
        } else {
            None
        }
    })
}

impl Index<(usize, usize)> for BoardState {
    type Output = TileState;

    /// Boards are indexed row-major starting in the upper left
    fn index(&self, (x, y): (usize, usize)) -> &<Self as Index<(usize, usize)>>::Output {
        &self.tiles[x + y * BOARD_SIZE]
    }
}

impl IndexMut<(usize, usize)> for BoardState {
    fn index_mut(
        &mut self,
        (x, y): (usize, usize),
    ) -> &mut <Self as Index<(usize, usize)>>::Output {
        &mut self.tiles[x + y * BOARD_SIZE]
    }
}

impl Display for BoardState {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                write!(fmt, "{}", self[(x, y)])?;

                if x != BOARD_SIZE - 1 {
                    write!(fmt, "|")?;
                }
            }

            if y != BOARD_SIZE - 1 {
                writeln!(fmt, "")?;

                for x in 0..BOARD_SIZE {
                    write!(fmt, "-")?;

                    if x != BOARD_SIZE - 1 {
                        write!(fmt, "+")?;
                    }
                }

                writeln!(fmt, "")?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod tile_state {
        use super::*;

        #[test]
        fn display() {
            assert_eq!(format!("{}", TileState::X), "X");
            assert_eq!(format!("{}", TileState::O), "O");
            assert_eq!(format!("{}", TileState::Empty), " ");
        }
    }

    mod player {
        use super::*;

        #[test]
        fn opponent() {
            assert_eq!(Player::X.opponent(), Player::O);
            assert_eq!(Player::X.opponent().opponent(), Player::X);
        }
    }

    mod board_state {
        use super::*;

        #[test]
        fn display() {
            assert_eq!(
                format!("{}", BoardState::new().play((1, 1)).unwrap()),
                " | | \n\
-+-+-
 |X| \n\
-+-+-
 | | \
"
            );
        }

        #[test]
        fn next() {
            assert_eq!(BoardState::new().next(), Player::X);
            assert_eq!(BoardState::new().play((0, 0)).unwrap().next(), Player::O);
        }

        #[test]
        fn iter_row() {
            use TileState::*;
            let board = BoardState {
                tiles: vec![X, X, Empty, O, X, Empty, Empty, Empty, O],
                next: Player::O,
            };
            assert_eq!(board.iter_row(0).collect::<Vec<_>>(), vec![X, X, Empty]);
            assert_eq!(board.iter_row(1).collect::<Vec<_>>(), vec![O, X, Empty]);
        }

        #[test]
        fn iter_col() {
            use TileState::*;
            let board = BoardState {
                tiles: vec![X, X, Empty, O, X, Empty, Empty, Empty, O],
                next: Player::O,
            };
            assert_eq!(board.iter_col(0).collect::<Vec<_>>(), vec![X, O, Empty]);
            assert_eq!(board.iter_col(1).collect::<Vec<_>>(), vec![X, X, Empty]);
        }

        #[test]
        fn iter_diag() {
            use TileState::*;
            let board = BoardState {
                tiles: vec![X, X, Empty, O, X, Empty, Empty, Empty, O],
                next: Player::O,
            };
            assert_eq!(board.iter_diag(false).collect::<Vec<_>>(), vec![X, X, O]);
            assert_eq!(
                board.iter_diag(true).collect::<Vec<_>>(),
                vec![Empty, X, Empty]
            );
        }

        #[test]
        fn won() {
            use TileState::*;
            assert_eq!(BoardState::new().won(), None);
            let board = BoardState {
                tiles: vec![X, O, X, O, X, X, O, X, O],
                next: Player::O,
            };
            assert_eq!(board.won(), None);
            let board = BoardState {
                tiles: vec![X, O, X, O, X, O, X, Empty, Empty],
                next: Player::O,
            };
            assert_eq!(board.won(), Some(Player::X));
        }
    }
}
