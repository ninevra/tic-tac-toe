use anyhow;

mod input;
mod state;

use input::input_coords;
use state::BoardState;

fn main() -> anyhow::Result<()> {
    let mut state = BoardState::new();

    loop {
        println!("\n{}\n", state);

        loop {
            match input_coords(state.next()) {
                Ok(coords) => match state.play(coords) {
                    Ok(_) => break,
                    Err(error) => println!("{}", error),
                },
                Err(error) => println!("{}", error),
            }
        }
    }
}
