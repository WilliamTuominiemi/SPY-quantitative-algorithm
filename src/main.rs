use serde::Deserialize;
use std::{error::Error, fs::File, process};

#[derive(Debug, Deserialize)]
struct Row {
    date: String,
    close: f32,
    volume: i32,
    open: f32,
    high: f32,
    low: f32,
}

fn example() -> Result<(), Box<dyn Error>> {
    let file = File::open("data.csv")?;
    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.deserialize() {
        let record: Row = result?;
        println!("{:?}", record);
    }

    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
