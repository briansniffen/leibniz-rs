use std::f64::consts::PI;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};

const TARGET: u32 = 10;
const GUESS: u32 = 100;

struct Leibniz {
    x: f64,
    d: f64,
    ticks: u32,
    tocks: u32,
}

fn computer(state: Arc<Mutex<Leibniz>>) {
    loop {
        let mut s = state.lock().unwrap();
        for _ in 1..s.ticks {
            s.x += 1.0 / s.d;
            s.d += 2.0;
            s.x -= 1.0 / s.d;
            s.d += 2.0;
        }
        s.tocks += 1;
    }
}

fn inspector(state: Arc<Mutex<Leibniz>>) {
    let mut old_d = 1.0;
    let now = SystemTime::now();
    loop {
        thread::sleep(Duration::from_secs(1));
        let mut s = state.lock().unwrap();
        if s.tocks <= TARGET {
            s.ticks /= 2;
        } else if s.tocks > TARGET {
            s.ticks = s.ticks + s.ticks / 10;
        }
        println!("{:?} {} {} {} {}", now.elapsed().unwrap(), s.ticks, s.tocks, s.d - old_d, PI - 4.0 * s.x);
        old_d = s.d;
        s.tocks = 0;
    }
}

fn main() {
    println!("ARC std version");
    let state = Arc::new(Mutex::new(Leibniz {
        x: 0.0,
        d: 1.0,
        ticks: GUESS,
        tocks: 0,
    }));
    let state_i = state.clone();
    thread::spawn(move || computer(state));
    inspector(state_i);
}
