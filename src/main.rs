use std::{env, error::Error, str::FromStr};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod grid_extensions;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();

    if let Some(day) = args.nth(1) {
        let daynum = u32::from_str(&day)?;

        match daynum {
            1 => day1::run(),
            2 => day2::run(),
            3 => day3::run(),
            4 => day4::run(),
            5 => day5::run(),
            _ => Err(format!("Unrecognised day {}", daynum).into()),
        }
    } else {
        Err("First argument must be a day number to run"
            .to_string()
            .into())
    }
}
