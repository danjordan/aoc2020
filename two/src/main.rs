use std::env;

struct Rule {
    min: u32,
    max: u32,
    letter: String,
}

type PasswordRuleTuple = (String, Rule);

type PasswordsWithRule = Vec<PasswordRuleTuple>;

fn parse(input: &str) -> PasswordsWithRule {
    let lines = input.lines();

    let parsed = lines
        .map(|x| {
            let pairs: Vec<&str> = x.splitn(2, ':').collect();
            let splitted: Vec<&str> = pairs[0].split_whitespace().collect();
            let min_max: Vec<u32> = splitted[0]
                .split('-')
                .map(|x| x.trim().parse().unwrap())
                .collect();

            let rule: Rule = Rule {
                min: min_max[0],
                max: min_max[1],
                letter: splitted[1].to_string(),
            };

            let password = pairs[1].to_string();

            return (password, rule);
        })
        .collect();

    return parsed;
}

fn validate_one(input: PasswordsWithRule) -> PasswordsWithRule {
    let valid = input
        .into_iter()
        .filter(|item| {
            let password = &item.0;
            let letter = &item.1.letter;
            let min = item.1.min as usize;
            let max = item.1.max as usize;
            let matches = password.matches(letter);
            let count = matches.count();

            return count >= min && count <= max;
        })
        .collect();

    return valid;
}

fn validate_two(input: PasswordsWithRule) -> PasswordsWithRule {
    let valid = input
        .into_iter()
        .filter(|item| {
            let password = &item.0;
            let letter = &item.1.letter;
            let min = item.1.min as usize;
            let max = item.1.max as usize;
            let char_1 = password.chars().nth(min).unwrap().to_string();
            let char_2 = password.chars().nth(max).unwrap().to_string();

            return char_1.eq(letter) ^ char_2.eq(letter);
        })
        .collect();

    return valid;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let p_1 = parse(input);
    let v_1 = validate_one(p_1);

    println!("Invalid count: {}", v_1.len());

    let p_2 = parse(input);
    let v_2 = validate_two(p_2);

    println!("Invalid count: {}", v_2.len());
}
