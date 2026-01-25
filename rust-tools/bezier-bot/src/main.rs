use enigo::{Enigo, MouseControllable};
use rand::Rng;
use std::{thread, time};

const SCREEN_WIDTH: i32 = 1920;
const SCREEN_HEIGHT: i32 = 1080;
const CENTER_X: i32 = SCREEN_WIDTH / 2;
const CENTER_Y: i32 = SCREEN_HEIGHT / 2;
const RANGE: i32 = 450; 

fn get_bezier_point(t: f64, p0: f64, p1: f64, p2: f64) -> f64 {
    let u = 1.0 - t;
    let tt = t * t;
    let uu = u * u;
    (uu * p0) + (2.0 * u * t * p1) + (tt * p2)
}

fn smooth_step(t: f64) -> f64 {
    t * t * (3.0 - 2.0 * t)
}

fn main() {
    let mut enigo = Enigo::new();
    let mut rng = rand::rng(); 

    println!(">>> BEZIER BOT (BURST MODE) ACTIVATED <<<");
    println!(">>> WARNING: HIGH SPEED MOUSE MOVEMENT <<<");
    
    println!("Starting in 3 seconds...");
    thread::sleep(time::Duration::from_secs(3));

    let mut current_x = CENTER_X;
    let mut current_y = CENTER_Y;
    enigo.mouse_move_to(current_x, current_y);

    loop {
        let start_x = current_x; 
        let start_y = current_y;
        
        let target_x = rng.random_range((CENTER_X - RANGE)..(CENTER_X + RANGE));
        let target_y = rng.random_range((CENTER_Y - RANGE)..(CENTER_Y + RANGE));

        let mid_x = (start_x + target_x) / 2;
        let mid_y = (start_y + target_y) / 2;
        let offset_x = rng.random_range(-300..300);
        let offset_y = rng.random_range(-300..300);
        let control_x = mid_x + offset_x;
        let control_y = mid_y + offset_y;

        let dist = ((target_x - start_x).pow(2) as f64 + (target_y - start_y).pow(2) as f64).sqrt();
        let steps = (dist / 2.0) as i32 + 1;

        for i in 1..=steps {
            let t_raw = i as f64 / steps as f64;
            let t = smooth_step(t_raw);

            let next_x_f = get_bezier_point(t, start_x as f64, control_x as f64, target_x as f64);
            let next_y_f = get_bezier_point(t, start_y as f64, control_y as f64, target_y as f64);

            let next_x = next_x_f as i32;
            let next_y = next_y_f as i32;

            if next_x != current_x || next_y != current_y {
                enigo.mouse_move_to(next_x, next_y);
                current_x = next_x;
                current_y = next_y;
            }

            if i % 20 == 0 {
                thread::sleep(time::Duration::from_micros(300));
            }
        }

        thread::sleep(time::Duration::from_millis(10));
    }
}