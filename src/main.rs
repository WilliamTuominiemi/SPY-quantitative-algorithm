use rand_distr::{Distribution, Normal};
use serde::Deserialize;
use std::{error::Error, fs::File, process, vec};

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

fn log_returns(rows: &[Row]) -> Vec<f32> {
    let mut result: Vec<f32> = vec![];

    for n in 1..rows.len() {
        result.push(calculate_log_return(rows[n].close, rows[n - 1].close));
    }

    result
}

fn mean(v: &Vec<f32>) -> f32 {
    let sum: f32 = v.iter().sum();
    let count = v.len() as f32;
    sum / count
}

fn variance(v: &Vec<f32>, mean: &f32) -> f32 {
    let mut numerator = 0.0;

    for n in v {
        numerator += (n - mean).powf(2.0);
    }

    numerator / v.len() as f32
}

fn standard_deviation(variance: &f32) -> f32 {
    variance.sqrt()
}

fn drift_component(mean: &f32, variance: &f32) -> f32 {
    mean - 0.5 * variance
}

fn random_component(mean: &f32) -> f32 {
    let normal = Normal::new(0.0, 1.0).unwrap();
    let z = normal.sample(&mut rand::rng());

    mean * z
}

fn geometric_brownian_motion(todays_price: &f32, mean: &f32, variance: &f32) -> f32 {
    let drift = drift_component(&mean, &variance);
    let random = random_component(&mean);

    todays_price * f32::exp(drift + random)
}

fn run_alternate_simulation(
    starting_price: &f32,
    time_horizon: &i32,
    mean: &f32,
    variance: &f32,
) -> Vec<f32> {
    let mut previous_days_price = *starting_price;
    let mut result: Vec<f32> = vec![];
    result.push(previous_days_price);

    for _ in 0..*time_horizon {
        previous_days_price = geometric_brownian_motion(&previous_days_price, mean, variance);
        result.push(previous_days_price);
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

    let log_returns = log_returns(&rows);
    let mean = mean(&log_returns);
    let variance = variance(&log_returns, &mean);

    let time_horizon = 10;
    let starting_price = rows[0].close;

    let result: Vec<f32> =
        run_alternate_simulation(&starting_price, &time_horizon, &mean, &variance);

    println!("{:?}", result);

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
    use super::*;

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
        assert_eq!(mean(&v), 2.5);
    }

    #[test]
    fn test_variance() {
        let v: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0];
        let mean = mean(&v);
        assert_eq!(variance(&v, &mean), 1.25);
    }

    #[test]
    fn test_standard_deviation() {
        let v: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0];
        let mean = mean(&v);
        let variance = variance(&v, &mean);

        assert_eq!(standard_deviation(&variance), 1.118034);
    }

    #[test]
    fn test_drift_component() {
        let mean = 0.55;
        let variance = 0.12;
        assert_eq!(drift_component(&mean, &variance), 0.49);
    }
}
