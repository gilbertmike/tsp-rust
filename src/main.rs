use rand;
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
            x: rand::random(),
            y: rand::random(),
        })
        .collect();

    let start = std::time::Instant::now();
    let (order, dist) = tsp::tsp_solve(&towns);
    let duration = start.elapsed().as_secs_f32();

    print!("Result: ");
    for town in order {
        print!("{}, ", town);
    }
    println!();
    println!("Dist: {}", dist);
    println!("Elapsed time: {} towns in {} sec", ntowns, duration);
}
