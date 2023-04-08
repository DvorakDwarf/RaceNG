//! RaceRNG, innovative new way of designing RNG

use std::thread;
use std::sync::mpsc;
use std::time::Instant;

///The function will randomly decide between x and y numbers using a race condition
pub fn race(x: i32, y: i32) -> i32 {
    let mut z = 0;

    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    thread::spawn(move || {
        loop {
            tx1.send(x);
            let slowdown = generate_primes(2000);
        }
    });
    thread::spawn(move || {
        loop {
            tx2.send(y);
            let slowdown = generate_primes(2000);
        }
    });

    let mut now = Instant::now();
    loop {
        match rx1.try_recv() {
            Ok(v) => {z = v},
            Err(_) => ()
        }
        match time_check(&mut now, z) {
            Some(v) => {return v}
            None => (),
        }
        
        match rx2.try_recv() {
            Ok(v) => {z = v},
            Err(_) => ()
        }
        match time_check(&mut now, z) {
            Some(v) => {return v}
            None => (),
        }
    }
}

fn generate_primes(n: usize) -> Vec<usize> {
    let mut primes: Vec<usize> = Vec::new();

    let count = 3;
    loop {
        let magic = ((count as f32).sqrt().round()) as usize + 1;
        for i in 2..magic {
            if count % i == 0 {
                break
            }
            primes.push(count);
        }

        if primes.len() > n {
            return primes;
        }
    }
}

fn time_check(now: &mut Instant, z: i32) -> Option<i32>{
    let elapsed = now.elapsed();
    if elapsed.as_millis() > 300 {
        *now = Instant::now();
        return Some(z)
    }

    return None;
}