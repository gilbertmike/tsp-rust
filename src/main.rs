use rand::prelude::random;
use std::env;

use tsp_rust::tsp;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of arguments!\nUsage: tsp-rust NTOWNS");
        std::process::exit(0);
    }
    let ntowns = match args[1].parse::<usize>() {
        Ok(n) => n,
        Err(_) => {
            println!("NTOWNS has to be an integer!\nUsage: tsp-rust NTOWNS");
            std::process::exit(0);
        }
    };

    let towns: Vec<tsp::Town> = (0..ntowns)
        .map(|_| tsp::Town {
            x: random(),
            y: random(),
        })
        .collect();

    let mut duration: f32 = f32::INFINITY;
    let mut order: Vec<usize> = vec![];
    let mut dist: f64 = f64::INFINITY;
    for _ in 0..5 {
        let start = std::time::Instant::now();
        let tup = tsp::tsp_solve(&towns);
        duration = duration.min(start.elapsed().as_secs_f32());
        order = tup.0;
        dist = tup.1;
    }

    print!("Result: ");
    for town in order {
        print!("{}, ", town);
    }
    println!();
    println!("Dist: {}", dist);
    println!("Elapsed time: {} towns in {} sec", ntowns, duration);
}
