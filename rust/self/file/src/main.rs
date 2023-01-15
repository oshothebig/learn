use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let filename = std::env::args().nth(1);
    let filename = if let Some(filename) = filename {
        filename
    } else {
        std::process::exit(1);
    };
    let file = std::fs::File::open(filename)?;
    for line in io::BufReader::new(file).lines() {
        println!("{}", line?);
    }
    Ok(())
}
