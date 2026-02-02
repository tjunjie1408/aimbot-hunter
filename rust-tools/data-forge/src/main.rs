use anyhow::Result;
use csv::Writer;
use rand::Rng;
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

const TOTAL_SAMPLES_PER_BOT: usize = 15_000;
const CENTER_X: f64 = 960.0;
const CENTER_Y: f64 = 540.0;
const RANGE: f64 = 450.0;

#[derive(Debug, Serialize)]
struct MouseRecord {
    timestamp: u128,
    x: f64,
    y: f64,
}

fn main() -> Result<()> {
    println!(">>> DATA FORGE: FABRICATING EVIDENCE... <<<");

    println!("[1/2] Forging Linear Bot Data...");
    generate_linear("../data/golden_linear.csv")?;

    println!("[2/2] Forging Bezier Bot Data...");
    generate_bezier("../data/golden_bezier.csv")?;

    println!(">>> MISSION ACCOMPLISHED. DATA READY IN ../data/ <<<");
    Ok(())
}

fn generate_linear(path: &str) -> Result<()> {
    let mut wtr = Writer::from_path(path)?;
    let mut rng = rand::rng();
    
    let mut current_ts = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    
    let mut curr_x = CENTER_X;
    let mut curr_y = CENTER_Y;
    let mut generated_count = 0;

    while generated_count < TOTAL_SAMPLES_PER_BOT {
        let target_x = rng.random_range((CENTER_X - RANGE)..(CENTER_X + RANGE));
        let target_y = rng.random_range((CENTER_Y - RANGE)..(CENTER_Y + RANGE));

        let steps = rng.random_range(30..80);
        let dx = (target_x - curr_x) / steps as f64;
        let dy = (target_y - curr_y) / steps as f64;

        for _ in 0..steps {
            curr_x += dx;
            curr_y += dy;
            
            current_ts += 5_000_000; 

            wtr.serialize(MouseRecord {
                timestamp: current_ts,
                x: curr_x.round(),
                y: curr_y.round(),
            })?;

            generated_count += 1;
            if generated_count >= TOTAL_SAMPLES_PER_BOT { break; }
        }
    }
    wtr.flush()?;
    Ok(())
}

fn generate_bezier(path: &str) -> Result<()> {
    let mut wtr = Writer::from_path(path)?;
    let mut rng = rand::rng();
    
    let mut current_ts = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let mut curr_x = CENTER_X;
    let mut curr_y = CENTER_Y;
    let mut generated_count = 0;

    while generated_count < TOTAL_SAMPLES_PER_BOT {
        let start_x = curr_x;
        let start_y = curr_y;
        let target_x = rng.random_range((CENTER_X - RANGE)..(CENTER_X + RANGE));
        let target_y = rng.random_range((CENTER_Y - RANGE)..(CENTER_Y + RANGE));

        let mid_x = (start_x + target_x) / 2.0;
        let mid_y = (start_y + target_y) / 2.0;
        let control_x = mid_x + rng.random_range(-200.0..200.0);
        let control_y = mid_y + rng.random_range(-200.0..200.0);

        let steps = rng.random_range(40..100);

        for i in 1..=steps {
            let t_raw = i as f64 / steps as f64;
            let t = t_raw * t_raw * (3.0 - 2.0 * t_raw);

            let u = 1.0 - t;
            let tt = t * t;
            let uu = u * u;

            let next_x = (uu * start_x) + (2.0 * u * t * control_x) + (tt * target_x);
            let next_y = (uu * start_y) + (2.0 * u * t * control_y) + (tt * target_y);

            curr_x = next_x;
            curr_y = next_y;
            current_ts += 5_000_000; // 5ms

            wtr.serialize(MouseRecord {
                timestamp: current_ts,
                x: curr_x.round(),
                y: curr_y.round(),
            })?;

            generated_count += 1;
            if generated_count >= TOTAL_SAMPLES_PER_BOT { break; }
        }
    }
    wtr.flush()?;
    Ok(())
}
