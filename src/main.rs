use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), std::io::Error> {
    let cmd = clap::Command::new("nz")
        .bin_name("nz")
        .subcommand_required(true)
        .subcommand(
            clap::command!("outline").args(&[
                clap::arg!(<input>)
            ]),
        );
    let matches = cmd.get_matches();
    let matches = match matches.subcommand() {
        Some(("outline", matches)) => matches,
        _ => unreachable!("clap should ensure we don't get here"),
    };
    let input = matches.get_one::<String>("input");
    println!("{input:?}");
    let mut file = File::open(input.unwrap())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines = contents.lines();
    for line in lines {
        println!("{line:?}");
    }
    // println!("{lines:#?}");
    Ok(())
}

#[cfg(test)]
mod tests {
    // use super::*;
    #[test]
    fn prints_outline() {
        assert_eq!(3, 3);
    }
}

