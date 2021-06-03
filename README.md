# Permutations
Rust implementation of Heap's algorithm to generate all possible permutations with repetitions of a vector of numbers.

## Time and space complexity of the implementation

### Time complexity
There are _n!_ possible permutations for a list of _n_ numbers and generating all of them requires _O(n!)_ time. The worst case scenario requires additional loops that do not generate any permutation. Since the algorithm will generate a permutation after a maximum of _n_ iterations the time complexity of the worst case scenation is _O(n * n!)_.

### Space complexity
This algorithm works with two vectors of _n_ numbers each:
* The vector of numbers to generate the permutations
* The control vector that supports the `swap` operation

Since the `swap` operations does not use any additional variables to keep temporal values the space complexity of the algorithm is _O(2n)_.

## How to run the application?

### With cargo
* `cat input | cargo run > output`

### With the binari
* `cat input | ./target/debug/permutations > output`

## Running the tests
`cargo test`

## Generating the docs
`cargo doc`

## Additional libraries used
In order to simplify the testing of the command line application `assert_cmd` and `predicates` were used. This libraries were not used to facilitate the implementation of the algorithm.
