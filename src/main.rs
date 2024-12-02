use std::{env, error::Error, str::FromStr};

mod day1;
mod day2;
mod day3;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();

    if let Some(day) = args.nth(1) {
        let daynum = u32::from_str(&day)?;

        match daynum {
            1 => day1::run(),
            2 => day2::run(),
            3 => day3::run(),
            _ => Err(format!("Unrecognised day {}", daynum).into()),
        }
    } else {
        Err("First argument must be a day number to run"
            .to_string()
            .into())
    }
}
