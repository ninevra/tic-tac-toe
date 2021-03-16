use core::{
    fmt::{self, Display, Formatter},
    ops::{Index, IndexMut},
};

#[derive(Debug, Clone, PartialEq)]
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
    cells: Vec<TileState>,
    next: Player,
}

impl BoardState {
    pub fn new() -> Self {
        BoardState {
            cells: vec![TileState::Empty; BOARD_SIZE * BOARD_SIZE],
            next: Player::X,
        }
    }

    pub fn play(&mut self, (x, y): (usize, usize)) -> anyhow::Result<&mut Self> {
        if x > BOARD_SIZE || y > BOARD_SIZE {
            return Err(anyhow::anyhow!("({}, {}) is out of bounds", x, y));
        }

        match self[(x, y)] {
            TileState::Empty => {
                self[(x, y)] = self.next.into();
                self.next = self.next.opponent();
                Ok(self)
            }
            _ => Err(anyhow::anyhow!("({}, {}) has already been played", x, y))
        }
    }

    pub fn next(&self) -> Player {
        self.next
    }
}

impl Index<(usize, usize)> for BoardState {
    type Output = TileState;

    /// Boards are indexed row-major starting in the upper left
    fn index(&self, (x, y): (usize, usize)) -> &<Self as Index<(usize, usize)>>::Output {
        &self.cells[x + y * BOARD_SIZE]
    }
}

impl IndexMut<(usize, usize)> for BoardState {
    fn index_mut(
        &mut self,
        (x, y): (usize, usize),
    ) -> &mut <Self as Index<(usize, usize)>>::Output {
        &mut self.cells[x + y * BOARD_SIZE]
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
    }
}
