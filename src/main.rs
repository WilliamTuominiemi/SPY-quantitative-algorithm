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

fn calculate_log_returns(rows: Vec<Row>) -> Vec<f32> {
    let mut result: Vec<f32> = vec![];

    for n in 1..rows.len() {
        let log_return = f32::ln(rows[n].close / rows[n - 1].close);
        result.push(log_return);
    }

    result
}

fn start() -> Result<(), Box<dyn Error>> {
    let file = File::open("data.csv")?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut rows: Vec<Row> = vec![];

    for result in rdr.deserialize() {
        rows.push(result?);
    }

    let log_returns = calculate_log_returns(rows);

    println!("{:?}", log_returns);

    Ok(())
}

fn main() {
    if let Err(err) = start() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
