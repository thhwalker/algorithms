use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() {
    let mut my_vec: Vec<u32> = (0..100).collect(); /* Build a vector of integers */
    my_vec.shuffle(&mut thread_rng()); /* Shuffle said vector with some randomness */

    let guessing: u32 = rand::random::<u32>() % 100;
    sequential_search(guessing, &my_vec);
    println!("{:?}", my_vec);
}


// Linear/Sequential search
fn sequential_search(guess: u32, data: &Vec<u32>) -> i32 {
    let mut steps: i32 = 0;

    for x in 0..data.len() {
        steps += 1;
        println!("Trying step {}: vec={}, guess={}", steps, data[x], guess);
        if data[x] == guess {
            return steps;
        }
    }
    return -1;
}


// Binary search

// Recursive Binary search
