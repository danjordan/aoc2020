use std::collections::HashSet;
use std::env;

fn one(input: &str) -> usize {
    let mut count = 0;

    for group in input.split("\n\n") {
        let mut answers = HashSet::new();

        for line in group.lines() {
            for c in line.chars() {
                answers.insert(c);
            }
        }

        count += answers.len();
    }

    return count;
}

fn two(input: &str) -> usize {
    let mut count = 0;

    for group in input.split("\n\n") {
        let mut answers = HashSet::new();

        for letter in "abcdefghijklmnopqrstuvwxyz".chars() {
            answers.insert(letter);
        }

        for line in group.lines() {
            answers.retain(|&k| {
                let chars: Vec<char> = line.chars().collect();
                return chars.contains(&k);
            });
        }

        count += answers.len();
    }

    return count;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let count_one = one(input);

    println!("Count one: {}", count_one);

    let count_two = two(input);

    println!("Count two: {}", count_two);
}
