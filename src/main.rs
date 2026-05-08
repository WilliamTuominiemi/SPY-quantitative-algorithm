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

fn calculate_log_return(a: f32, b: f32) -> f32 {
    f32::ln(a / b)
}

fn log_returns(rows: Vec<Row>) -> Vec<f32> {
    let mut result: Vec<f32> = vec![];

    for n in 1..rows.len() {
        result.push(calculate_log_return(rows[n].close, rows[n - 1].close));
    }

    result
}

fn mean(v: Vec<f32>) -> f32 {
    let sum: f32 = v.iter().sum();
    let count = v.len() as f32;
    sum / count
}

fn start() -> Result<(), Box<dyn Error>> {
    let file = File::open("data.csv")?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut rows: Vec<Row> = vec![];

    for result in rdr.deserialize() {
        rows.push(result?);
    }

    let log_returns = log_returns(rows);

    let mean = mean(log_returns);

    println!("{:?}", mean);

    Ok(())
}

fn main() {
    if let Err(err) = start() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use crate::{calculate_log_return, mean};

    #[test]
    fn test_calculate_log_return() {
        let a = 417.94;
        let b = 422.12;
        let c = 419.07;

        assert_eq!(calculate_log_return(b, a), 0.009951738);
        assert_eq!(calculate_log_return(c, b), -0.007251624);
    }

    #[test]
    fn test_mean() {
        let v: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0];
        assert_eq!(mean(v), 2.5);
    }
}
