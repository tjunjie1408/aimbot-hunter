use anyhow::Result;
use colored::*;
use csv::WriterBuilder;
use rdev::{listen, EventType};
use serde::Serialize;
use std::fs;
use std::path::Path;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize)]
struct MouseRecord {
    timestamp: u128,
    x: f64,
    y: f64,
}

fn main() -> Result<()> {
    println!("{}", ">>> HIGH-PERFORMANCE KERNEL LOGGER STARTED...".green().bold());
    println!("{}", ">>> DO NOT CLICK THIS WINDOW! IT WILL PAUSE RECORDING!".red().bold());
    
    let file_path = "../data/captured.csv";
    if let Some(parent) = Path::new(file_path).parent() {
        fs::create_dir_all(parent)?;
    }

    let mut wtr = WriterBuilder::new()
        .has_headers(true)
        .from_path(file_path)?;

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        if let Err(error) = listen(move |event| {
            match event.event_type {
                EventType::MouseMove { x, y } => {
                    let ts = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap_or(Duration::ZERO)
                        .as_nanos();
                    
                    let _ = tx.send(MouseRecord { timestamp: ts, x, y });
                }
                _ => {}
            }
        }) {
            println!("Listener Error: {:?}", error);
        }
    });

    println!("{}", ">>> RECORDING... (Use Ctrl+C to stop)".yellow());

    let mut count: u64 = 0;
    let mut last_print = Instant::now();

    for record in rx {
        wtr.serialize(&record)?;
        count += 1;

        if last_print.elapsed() >= Duration::from_secs(1) {
            print!("\r>>> STATUS: {} packets captured | Rate: {:.1} Hz", 
                count.to_string().cyan(), 
                count as f64 / last_print.elapsed().as_secs_f64()
            );
            wtr.flush()?; 
            last_print = Instant::now();
        }
    }

    Ok(())
}