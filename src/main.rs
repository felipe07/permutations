use std::io::Stdin;
use std::io::{self, BufRead};

fn main() {
    let reader = io::stdin();
    let mut elements: Vec<i64> = parse_input(&reader);
    let len = elements.len(); 
    generate_permutations(&mut elements, len);
}

/// Converts a single string of comma separated numbers provided through 
/// standard input into a vector of numbers
/// ```
/// let reader = io::stdin();
/// let mut elements: Vec<i64> = parse_input(&reader);
/// ```
fn parse_input(reader: &Stdin) -> Vec<i64> {
    reader.lock()
        .lines().next().unwrap().unwrap()
        .split(',').map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().expect("An error occurred while parsing input to numbers"))
        .collect()
}

/// Generates all possible permutations with repetitions of a vector of numbers.
/// The function implements the iterative version of Heap's algorithm as descrived on: 
/// https://en.wikipedia.org/wiki/Heap%27s_algorithm
/// ```
/// generate_permutations([1, 2, 3, 4], 4);
/// ```
fn generate_permutations(elements: &mut Vec<i64>, len: usize) {
    let mut control_vec: Vec<usize> = vec![0; len]; 
    let mut i: usize = 0;
    
    println!("{:?}", elements);
    
    while i < len {
        if control_vec[i] < i {
            if i % 2 == 0 {
                elements.swap(0, i);
            } else {
                elements.swap(control_vec[i], i);
            }
            println!("{:?}", elements);
            control_vec[i] += 1;
            i = 0;
        } else {
            control_vec[i] = 0;
            i += 1;
        }
    }
}
