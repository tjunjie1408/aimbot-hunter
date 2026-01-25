use enigo::{Enigo, MouseControllable}; 
use rand::Rng;
use std::{thread, time};

const SCREEN_WIDTH: i32 = 1920;
const SCREEN_HEIGHT: i32 = 1080;
const CENTER_X: i32 = SCREEN_WIDTH / 2;
const CENTER_Y: i32 = SCREEN_HEIGHT / 2;
const RANGE: i32 = 400;


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

    println!(">>> BEZIER AIMBOT (HUMANIZED) ACTIVATED <<<");
    println!(">>> PRESS CTRL+C TO STOP <<<");
    println!("Starting in 3 seconds...");
    thread::sleep(time::Duration::from_secs(3));

    enigo.mouse_move_to(CENTER_X, CENTER_Y);

    loop {
        let (start_x, start_y) = enigo.mouse_location();
        
        let target_x = rng.random_range((CENTER_X - RANGE)..(CENTER_X + RANGE));
        let target_y = rng.random_range((CENTER_Y - RANGE)..(CENTER_Y + RANGE));

        let mid_x = (start_x + target_x) / 2;
        let mid_y = (start_y + target_y) / 2;
        
        let offset_x = rng.random_range(-200..200);
        let offset_y = rng.random_range(-200..200);
        
        let control_x = mid_x + offset_x;
        let control_y = mid_y + offset_y;

        let duration_ms = rng.random_range(400..800);
        let steps = duration_ms / 5; 
        let step_delay = time::Duration::from_millis(5);

        for i in 1..=steps {
            let t_raw = i as f64 / steps as f64;
            let t = smooth_step(t_raw);

            let next_x = get_bezier_point(t, start_x as f64, control_x as f64, target_x as f64);
            let next_y = get_bezier_point(t, start_y as f64, control_y as f64, target_y as f64);

            enigo.mouse_move_to(next_x as i32, next_y as i32);
            thread::sleep(step_delay);
        }

        let idle_time = rng.random_range(150..500);
        thread::sleep(time::Duration::from_millis(idle_time));
    }
}