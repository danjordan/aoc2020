use std::env;

fn get_seat_id(input: &str) -> usize {
    let characters: Vec<char> = input.chars().collect();
    let mut row_min: usize = 0;
    let mut row_max: usize = 127;
    let mut column_min: usize = 0;
    let mut column_max: usize = 7;

    for character in characters {
        let row_difference: usize = (row_max - row_min) / 2;
        let column_difference: usize = (column_max - column_min) / 2;

        match character {
            'F' => row_max = row_min + row_difference,
            'B' => row_min = row_min + row_difference + 1,
            'R' => column_min = column_min + column_difference + 1,
            'L' => column_max = column_min + column_difference,
            _ => println!("Missing implementation for {}", character),
        }
    }

    return (8 * row_max) + column_max;
}

fn one(input: &str) -> usize {
    let mut highest_seat_id = 0;

    let lines: Vec<&str> = input.lines().collect();

    for line in lines.iter() {
        let seat_id = get_seat_id(line);

        if seat_id > highest_seat_id {
            highest_seat_id = seat_id;
        }
    }

    return highest_seat_id;
}

fn two(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let mut seats: [usize; 1024] = [0; 1024];

    for line in lines.iter() {
        let seat_id = get_seat_id(line);
        seats[seat_id] = seat_id;
    }

    let mut find_seat: bool = false;

    for (index, _) in seats.iter().enumerate() {
        if seats[index] != 0 && !find_seat {
            find_seat = true
        }

        if seats[index] == 0 && find_seat {
            return index;
        }
    }

    return 0;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let seat_id = one(input);

    println!("Largest ID: {}", seat_id);

    let seat_id = two(input);

    println!("Seat ID: {}", seat_id);
}
