use anyhow;

mod input;
mod state;

use input::input_coords;
use state::BoardState;

fn main() -> anyhow::Result<()> {
    let mut state = BoardState::new();

    println!("\n{}\n", state);

    loop {
        loop {
            match input_coords(state.next()) {
                Ok(coords) => match state.play(coords) {
                    Ok(_) => break,
                    Err(error) => println!("{}", error),
                },
                Err(error) => println!("{}", error),
            }

            println!("Enter coordinates x, y");
        }

        println!("\n{}\n", state);

        if let Some(player) = state.won() {
            println!("{} wins!", player);
            break;
        }

        if state.drawn() {
            println!("Draw!");
            break;
        }
    }

    Ok(())
}
