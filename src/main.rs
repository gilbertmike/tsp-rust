use itertools::Itertools;
use rand::prelude::random;
use std::env;

/// A town with x and y coordinate.
#[derive(Clone, Copy)]
struct Town {
    pub x: f64,
    pub y: f64,
}

impl Town {
    fn dist(self: &Self, other: &Town) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

/// TSP algorithm.
struct TSP {}

impl TSP {
    /// Initialize a TSP algorithm.
    fn create() -> TSP {
        TSP {}
    }

    /// Solve a TSP problem specified by a vector of towns.
    fn solve(self: &Self, towns: &Vec<Town>) -> (Vec<usize>, f64) {
        let mut smallest_dist: f64 = f64::INFINITY;
        let mut best_order: Vec<usize> = vec![];
        for order in (0..towns.len()).permutations(towns.len()) {
            let dist = order
                .iter()
                .tuple_windows()
                .fold(0.0, |acc, (town1, town2)| {
                    acc + towns[*town1].dist(&towns[*town2])
                });
            if dist < smallest_dist {
                smallest_dist = dist;
                best_order = order;
            }
        }
        (best_order, smallest_dist)
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

    let tsp = TSP::create();
    let start = std::time::Instant::now();
    let (order, dist) = tsp.solve(&towns);
    let duration = start.elapsed();

    print!("Result: ");
    for town in order {
        print!("{}, ", town);
    }
    println!();
    println!("Dist: {}", dist);
    println!("Elapsed time: {} sec", duration.as_secs_f32());
}
