use enigo::{Enigo, MouseControllable};
use rand::Rng;
use std::{thread, time};

const SCREEN_WIDTH: i32 = 1920;
const SCREEN_HEIGHT: i32 = 1080;
const CENTER_X: i32 = SCREEN_WIDTH / 2;
const CENTER_Y: i32 = SCREEN_HEIGHT / 2;
const RANGE: i32 = 400;

fn main() {
    let mut enigo = Enigo::new();
    let mut rng = rand::rng(); 

    println!(">>> LINEAR AIMBOT (BASIC) ACTIVATED <<<");
    println!(">>> PRESS CTRL+C TO STOP <<<");
    
    println!("Starting in 3 seconds...");
    thread::sleep(time::Duration::from_secs(3));

    enigo.mouse_move_to(CENTER_X, CENTER_Y);

    loop {
        let (start_x, start_y) = enigo.mouse_location();

        let target_x = rng.random_range((CENTER_X - RANGE)..(CENTER_X + RANGE));
        let target_y = rng.random_range((CENTER_Y - RANGE)..(CENTER_Y + RANGE));

        let duration_ms = rng.random_range(100..300); 
        
        let steps = duration_ms / 5;
        let step_delay = time::Duration::from_millis(5);

        let dx = (target_x - start_x) as f64;
        let dy = (target_y - start_y) as f64;

        for i in 1..=steps {
            let t = i as f64 / steps as f64;
            
            let next_x = start_x as f64 + dx * t;
            let next_y = start_y as f64 + dy * t;

            enigo.mouse_move_to(next_x as i32, next_y as i32);
            thread::sleep(step_delay);
        }

        let idle_time = rng.random_range(100..400);
        thread::sleep(time::Duration::from_millis(idle_time));
    }
}