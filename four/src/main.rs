#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::env;

type Passport = HashMap<String, String>;

fn validate_passport(passport: &Passport) -> bool {
    let required = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];
    if passport.keys().len() == required.len() {
        return true;
    }

    if passport.keys().len() == 7 && !passport.contains_key("cid") {
        return true;
    }

    return false;
}

fn validate_byr(input: &str) -> bool {
    let byr: usize = input.parse().unwrap();
    return byr >= 1920 && byr <= 2002;
}

fn validate_iyr(input: &str) -> bool {
    let byr: usize = input.parse().unwrap();
    return byr >= 2010 && byr <= 2020;
}

fn validate_eyr(input: &str) -> bool {
    let byr: usize = input.parse().unwrap();
    return byr >= 2020 && byr <= 2030;
}

fn validate_hgt(input: &str) -> bool {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"(\d*)(cm|in)").unwrap();
    }

    if !REGEX.is_match(input) {
        return false;
    }

    let captures = REGEX.captures(input).unwrap();
    let number: usize = captures.get(1).unwrap().as_str().parse().unwrap();
    let unit = captures.get(2).unwrap().as_str();

    if unit == "in" {
        return number >= 59 && number <= 76;
    }

    if unit == "cm" {
        return number >= 150 && number <= 193;
    }

    return false;
}

fn validate_hcl(input: &str) -> bool {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"#[a-f0-9]{6}").unwrap();
    }
    return REGEX.is_match(input);
}

fn validate_ecl(input: &str) -> bool {
    let valid = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

    return valid.contains(&input);
}

fn validate_pid(input: &str) -> bool {
    return input.len() == 9;
}

fn validate_one(passport: &Passport) -> bool {
    return validate_passport(&passport);
}

fn validate_two(passport: &Passport) -> bool {
    if !validate_passport(passport) {
        return false;
    }

    if !validate_byr(passport.get("byr").unwrap()) {
        return false;
    }

    if !validate_iyr(passport.get("iyr").unwrap()) {
        return false;
    }

    if !validate_eyr(passport.get("eyr").unwrap()) {
        return false;
    }

    if !validate_hgt(passport.get("hgt").unwrap()) {
        return false;
    }

    if !validate_hcl(passport.get("hcl").unwrap()) {
        return false;
    }

    if !validate_ecl(passport.get("ecl").unwrap()) {
        return false;
    }

    if !validate_pid(passport.get("pid").unwrap()) {
        return false;
    }

    return true;
}

fn parse(input: &str) -> Vec<Passport> {
    let lines = input.split("\n\n");
    let passports: Vec<Passport> = lines
        .map(|line| {
            let mut passport = HashMap::new();

            for attr in line.split_whitespace() {
                let pairs: Vec<&str> = attr.split(':').collect();
                let key = pairs[0].to_string();
                let value = pairs[1].to_string();
                passport.insert(key, value);
            }

            return passport;
        })
        .collect();
    return passports;
}

fn one(input: &str) -> usize {
    let passports = parse(input);

    let mut valid = 0;

    for passport in passports {
        if validate_one(&passport) {
            valid += 1;
        }
    }

    return valid;
}

fn two(input: &str) -> usize {
    let passports = parse(input);

    let mut valid = 0;

    for passport in passports {
        if validate_two(&passport) {
            valid += 1;
        }
    }

    return valid;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let one_valid = one(input);
    println!("Valid count {}", one_valid);

    let two_valid = two(input);
    println!("Valid count {}", two_valid);
}
