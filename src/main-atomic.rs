extern crate crossbeam;

use crossbeam::{atomic::AtomicCell, thread};
use std::f64::consts::PI;
use std::thread as sthread;
use std::time::{Duration, SystemTime};

const TARGET: u32 = 10;
const GUESS: u32 = 2500000;

fn computer(
    xr: &AtomicCell<f64>,
    dr: &AtomicCell<f64>,
    ticks: &AtomicCell<u32>,
    tocks: &AtomicCell<u32>,
) {
    loop {
        let mut x = xr.load();
        let mut d = dr.load();
        let ticks = ticks.load();
        for _ in 1..ticks {
            x += 1.0 / d;
            d += 2.0;
            x -= 1.0 / d;
            d += 2.0;
        }
        tocks.fetch_add(1);
        xr.store(x);
        dr.store(d);
    }
}

fn inspector(
    xr: &AtomicCell<f64>,
    dr: &AtomicCell<f64>,
    ticksr: &AtomicCell<u32>,
    tocksr: &AtomicCell<u32>,
) {
    let mut old_d = 1.0;
    let mut now = SystemTime::now();
    loop {
        sthread::sleep(Duration::from_secs(1));
        let x = xr.load();
        let d = dr.load();
        let tocks = tocksr.load();
        let ticks = ticksr.load();
        if tocks <= TARGET {
            ticksr.store(ticks / 2);
        } else if tocks > TARGET {
            ticksr.store(ticks + ticks / 10);
        }
        println!("{:?} {} {} {} {}", now.elapsed().unwrap(), ticks, tocks, d - old_d, PI - 4.0 * x);
        tocksr.store(0);
    }
}

fn main() {
    println!("Atomic crossbeam version");
    let x = AtomicCell::new(0.0);
    let d = AtomicCell::new(1.0);
    let ticks = AtomicCell::new(GUESS);
    let tocks = AtomicCell::new(0);

    thread::scope(|s| {
        s.spawn(|_| {
            computer(&x, &d, &ticks, &tocks);
        });
        inspector(&x, &d, &ticks, &tocks);
    })
    .unwrap();
}
