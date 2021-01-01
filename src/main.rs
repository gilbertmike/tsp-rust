use rand::prelude::random;
use std::env;

/// A town with x and y coordinate.
#[derive(Clone, Copy)]
struct Town {
    pub x: f64,
    pub y: f64,
}

/// TSP algorithm.
struct TSP {}

impl TSP {
    /// Initialize a TSP algorithm.
    fn create() -> TSP {
        TSP {}
    }

    /// Solve a TSP problem specified by a vector of towns.
    fn solve(self: &mut Self, towns: &Vec<Town>) -> Vec<usize> {
        unimplemented!();
    }
}

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

    let towns: Vec<Town> = (0..ntowns)
        .map(|_| Town {
            x: random(),
            y: random(),
        })
        .collect();

    let mut tsp = TSP::create();
    let start = std::time::Instant::now();
    let order = tsp.solve(&towns);
    let duration = start.elapsed();

    print!("Result: ");
    for town in order {
        print!("{}, ", town);
    }
    println!();
    println!("Elapsed time: {} sec", duration.as_secs_f32());
}
