use anyhow::Result;
use colored::*;
use csv::Writer;
use rdev::{listen,EventType};
use serde::Serialize;
use std::fs;
use std::path::Path;
use std::sync::mpsc;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize)]
struct MouseRecord {
    timestamp: u128,
    x: f64,
    y: f64,
}

fn main() -> Result<()> {
    println!("{}", ">>> SYSTEM KERNEL HOOK INITIATED...".green().bold());
    println!("{}", ">>> TARGET: MOUSE_INPUT_DEVICE".green());
    println!("{}", ">>> WAITING FOR DATA STREAM...".green().blink());

    let file_path = "../data/captured.csv";
    
    if let Some(parent) = Path::new(file_path).parent() {
        fs::create_dir_all(parent)?;
    }

    let mut wtr = Writer::from_path(file_path)?;

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        if let Err(error) = listen(move |event| {
            match event.event_type {
                EventType::MouseMove { x, y } => {
                    let ts = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_nanos();

                    tx.send(MouseRecord { timestamp: ts, x, y }).unwrap();
                }
                _ => {}
            }
        }) {
            println!("Error: {:?}", error);
        }
    });

    let mut count = 0;
    println!("{}", ">>> RECORDING STARTED. PRESS CTRL+C TO STOP.".yellow());

    for record in rx {
        wtr.serialize(&record)?;
        
        count += 1;
        if count % 100 == 0 {
            wtr.flush()?;
            print!("\r>>> CAPTURED PACKETS: {} | LAST POS: ({:.1}, {:.1})", 
                count.to_string().cyan(), record.x, record.y);
        }
    }

    Ok(())
}