use std::env;

fn count_trees(map: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
    let mut count = 0;
    let mut current_x = 0;
    let mut current_y = 0;

    for row in map {
        if current_y % y == 0 {
            if row[current_x] == '#' {
                count += 1;
            }

            current_x += x;

            if current_x >= row.len() {
                current_x = current_x - row.len();
            }
        }

        current_y += 1;
    }

    return count;
}

fn one(input: &str) -> usize {
    let lines = input.lines();
    let map: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();
    let count = count_trees(&map, 3, 1);

    return count;
}

fn two(input: &str) -> usize {
    let lines = input.lines();
    let map: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();

    let combinations = [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]];
    let mut total = 1;

    for combination in combinations.iter() {
        let count = count_trees(&map, combination[0], combination[1]);
        total = total * count;
    }

    return total;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let one_count = one(input);

    println!("one count {}", one_count);

    let two_count = two(input);

    println!("two count {}", two_count);
}
