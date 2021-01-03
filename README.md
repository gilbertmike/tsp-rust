# Adventures in Travelling Salesperson
Adventures in Travelling Salesperson in Rust. Inspired by a performance 
engineering lecture by Jon Bentley at MIT.

## Optimizations
Numbers in "result" is only approximate as best as the input is generated 
randomly and are only quick performance test to help while working. Better
performance numbers can be obtained using the benchmarks although the inputs
are still randomly generated.

### Brute force.
Result: 9 towns in 1.075s; 10 towns in 11.310s.

Just enumerating through all permutations.

### Turn on compiler optimizations.
Result: 9 in 0.038s; 10 in 0.392s; 11 in 4.566s.

Set opts-level to 3 for release mode.

### Precompute town distances.
Did not make much difference. Not committed.

### Stop computing a path already larger than best distance.
Result: 10 in 0.0032s; 13 in 0.924s

Stop computing a path that cannot be smaller than current smallest distance.

### Get better lower bound of best path distance
Result: 13 in 0.088s; 14 in 2.56s.

Use distance between starting town and furthest unvisited town as lower bound
for minimum path length visiting all unvisiting towns.
