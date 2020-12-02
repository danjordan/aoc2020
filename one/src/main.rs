use std::env;

fn one(input: &str) -> u32 {
    let lines = input.lines();
    let nums: Vec<u32> = lines.map(|x| x.trim().parse().unwrap()).collect();

    for a in nums.iter() {
        for b in nums.iter() {
            if a + b == 2020 {
                return a * b;
            }
        }
    }

    return 0;
}

fn two(input: &str) -> u32 {
    let lines = input.lines();
    let nums: Vec<u32> = lines.map(|x| x.trim().parse().unwrap()).collect();

    for a in nums.iter() {
        for b in nums.iter() {
            for c in nums.iter() {
                if a + b + c == 2020 {
                    return a * b * c;
                }
            }
        }
    }

    return 0;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];
    let output_1 = one(input);
    println!("output one {}", output_1);
    let output_2 = two(input);
    println!("output two {}", output_2);
}
