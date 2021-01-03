/// A town with x and y coordinate.
#[derive(Clone, Copy)]
pub struct Town {
    pub x: f64,
    pub y: f64,
}

impl Town {
    /// Returns distance to another town `other`.
    pub fn dist(self: &Self, other: &Town) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

/// Solves the Travelling Salesperson problem given vector of towns. Returns an ordering (vector
/// of indices) and optimal distance.
///
/// # Examples
///
/// ```
/// use tsp_rust::{tsp_solve, Town};
/// let towns = vec![
///     Town { x: 0.0, y: 0.0 },
///     Town { x: 2.0, y: 0.0 },
///     Town { x: 1.0, y: 0.0 },
/// ];
/// let (order, dist) = tsp_solve(&towns);
/// assert_eq!(dist, 2.0);
/// assert!(order == [0, 2, 1] || order == [1, 2, 0]);
/// ```
pub fn tsp_solve(towns: &Vec<Town>) -> (Vec<usize>, f64) {
    let mut smallest_dist = f64::INFINITY;
    let mut best_order: Vec<usize> = vec![0; towns.len()];
    let mut visited = vec![false; towns.len()];
    for start in 0..towns.len() {
        visited[start] = true;
        if let Some(new_dist) = tsp_solve_with_start(
            towns,
            &mut visited,
            start,
            &mut best_order,
            0.0,
            smallest_dist,
        ) {
            smallest_dist = new_dist;
            best_order[0] = start;
        }
        visited[start] = false;
    }
    (best_order, smallest_dist)
}

/// Solve half-finished TSP where some towns are already visited and a starting town is specified.
/// Returns the length of a better path if it is found; None otherwise.
///
/// `cur_dist` specifies total distance of already existing path and `cur_smallest_dist` specifies
/// current known shortest path. This value will be used to stop hopeless search path.
///
/// The argument `visited` is passed in as mutable reference to save space. It will be left with
/// the same values at function return.
///
/// The argumetn `best_order` is passed in as mutable reference to save space. It will be updated
/// to contain the new best ordering at function return.
fn tsp_solve_with_start(
    towns: &Vec<Town>,
    visited: &mut Vec<bool>,
    start: usize,
    best_order: &mut Vec<usize>,
    cur_dist: f64,
    cur_smallest_dist: f64,
) -> Option<f64> {
    if towns.len() != visited.len() || towns.len() != best_order.len() || start >= towns.len() {
        panic!();
    }
    let mut new_smallest_dist = cur_smallest_dist;
    let start_town = towns[start];
    let cur_path_pos = visited.iter().filter(|&&x| x).count();
    if cur_path_pos == towns.len() {
        return Some(cur_dist);
    }
    let mut new_dist_found = false;
    for (idx, town) in towns.iter().enumerate() {
        let dist = cur_dist + town.dist(&start_town);
        if visited[idx] || dist > new_smallest_dist {
            continue;
        }
        // The path through all the rest of the towns has to be longer than the distance between
        // the start and the town furthest away so it is a valid heuristic
        let mut longest_dist_unvisited = 0.0;
        for (other_idx, other) in towns.iter().enumerate() {
            if !visited[other_idx] && other.dist(&town) > longest_dist_unvisited {
                longest_dist_unvisited = other.dist(&town);
            }
        }
        if dist + longest_dist_unvisited > new_smallest_dist {
            continue;
        }
        visited[idx] = true;
        if let Some(new_dist) =
            tsp_solve_with_start(towns, visited, idx, best_order, dist, new_smallest_dist)
        {
            new_smallest_dist = new_dist;
            best_order[cur_path_pos] = idx;
            new_dist_found = true;
        }
        visited[idx] = false;
    }
    if new_dist_found {
        Some(new_smallest_dist)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    /// Reference brute-force TSP solver.
    fn tsp_ref(towns: &Vec<Town>) -> (Vec<usize>, f64) {
        let mut smallest_dist: f64 = std::f64::INFINITY;
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

    fn path_dist(towns: &Vec<Town>, order: &Vec<usize>) -> f64 {
        if towns.len() != order.len() {
            panic!();
        }
        order
            .iter()
            .tuple_windows()
            .fold(0.0, |acc, (town1, town2)| {
                acc + towns[*town1].dist(&towns[*town2])
            })
    }

    #[test]
    fn tsp_three_towns() {
        let towns = vec![
            Town { x: 0.0, y: 1.0 },
            Town { x: 0.0, y: 0.0 },
            Town { x: 1.0, y: 0.0 },
        ];
        let (order, dist) = tsp_ref(&towns);
        assert_eq!(dist, 2.0);
        assert!(order == [0, 1, 2] || order == [2, 1, 0]);
        let (order, dist) = tsp_solve(&towns);
        assert_eq!(dist, 2.0);
        assert!(order == [0, 1, 2] || order == [2, 1, 0]);
    }

    #[test]
    fn tsp_five_towns() {
        let towns: Vec<Town> = (0..5)
            .map(|_| Town {
                x: rand::random(),
                y: rand::random(),
            })
            .collect();
        let (_, ref_dist) = tsp_ref(&towns);
        let (order, dist) = tsp_solve(&towns);
        assert_eq!(ref_dist, dist);
        assert_eq!(path_dist(&towns, &order), dist);
    }
}
