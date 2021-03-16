use core::str::FromStr;
use std::io::{self, Write};

use anyhow::{self, anyhow as anyhow_error};

use crate::state::Player;

pub fn input_coords(next: Player) -> anyhow::Result<(usize, usize)> {
    let string = prompt(&format!("{} > ", next))?;
    let numbers = parse_list::<usize>(&string)?;
    if numbers.len() == 2 {
        Ok((numbers[0], numbers[1]))
    } else {
        Err(anyhow_error!(
            "expected exactly 2 input numbers, got {}",
            numbers.len()
        ))
    }
}

pub fn prompt(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}

pub fn parse_list<T: FromStr>(input: &str) -> Result<Vec<T>, <T as FromStr>::Err> {
    input.split(",").map(|item| item.trim().parse()).collect()
}
