/**
--- Day 2: Password Philosophy ---

Your flight departs in a few days from the coastal airport; the easiest way down to the coast from here is via toboggan.

The shopkeeper at the North Pole Toboggan Rental Shop is having a bad day. "Something's wrong with our computers; we can't log in!" You ask if you can take a look.

Their password database seems to be a little corrupted: some of the passwords wouldn't have been allowed by the Official Toboggan Corporate Policy that was in effect when they were chosen.

To try to debug the problem, they have created a list (your puzzle input) of passwords (according to the corrupted database) and the corporate policy when that password was set.

For example, suppose you have the following list:

1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc

Each line gives the password policy and then the password. The password policy indicates the lowest and highest number of times a given letter must appear for the password to be valid. For example, 1-3 a means that the password must contain a at least 1 time and at most 3 times.

In the above example, 2 passwords are valid. The middle password, cdefg, is not; it contains no instances of b, but needs at least 1. The first and third passwords are valid: they contain one a or nine c, both within the limits of their respective policies.

How many passwords are valid according to their policies?
*/
use regex::Regex;

static INPUT: &str = include_str!("day02-input.txt");

fn validate_part_1(a: u8, b: u8, c: char, password: &str) -> bool {
    let count: u8 = password
        .chars()
        .map(|cur| if cur == c { 1 } else { 0 })
        .sum();

    count >= a && count <= b
}

/*--- Part Two ---

While it appears you validated the passwords correctly, they don't seem to be what the Official Toboggan Corporate Authentication System is expecting.

The shopkeeper suddenly realizes that he just accidentally explained the password policy rules from his old job at the sled rental place down the street! The Official Toboggan Corporate Policy actually works a little differently.

Each policy actually describes two positions in the password, where 1 means the first character, 2 means the second character, and so on. (Be careful; Toboggan Corporate Policies have no concept of "index zero"!) Exactly one of these positions must contain the given letter. Other occurrences of the letter are irrelevant for the purposes of policy enforcement.

Given the same example list from above:

    1-3 a: abcde is valid: position 1 contains a and position 3 does not.
    1-3 b: cdefg is invalid: neither position 1 nor position 3 contains b.
    2-9 c: ccccccccc is invalid: both position 2 and position 9 contain c.

How many passwords are valid according to the new interpretation of the policies?
*/
fn validate_part_2(i0: usize, i1: usize, c: char, password: &str) -> bool {
    let aa = password.chars().nth(i0 - 1).unwrap() == c;
    let bb = match password.chars().nth(i1 - 1) {
        Some(ch) => ch == c,
        None => false,
    };

    xor(aa, bb)
}

fn xor(a: bool, b: bool) -> bool {
    (a || b) && !(a == b)
}

fn main() {
    let re = Regex::new(r"([0-9]+)-([0-9]+) ([a-z]): ([a-z]*)$").unwrap();

    let mut valid_part1 = 0;
    let mut valid_part2 = 0;

    for line in INPUT.lines() {
        // min-max character:
        let captures = re.captures(&line).unwrap();
        let min = captures.get(1).unwrap().as_str().parse::<u8>().unwrap();
        let max = captures.get(2).unwrap().as_str().parse::<u8>().unwrap();
        let passchar = captures.get(3).unwrap().as_str().chars().next().unwrap();
        let password = captures.get(4).unwrap().as_str();

        if validate_part_1(min, max, passchar, &password) {
            valid_part1 += 1;
        }

        if validate_part_2(min as usize, max as usize, passchar, &password) {
            valid_part2 += 1;
        }
    }
    println!("Number of valid passwords - part 1: {}", valid_part1);
    println!("Number of valid passwords - part 2: {}", valid_part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xor_test() {
        assert_eq!(false, xor(false, false));
        assert_eq!(true, xor(true, false));
        assert_eq!(true, xor(false, true));
        assert_eq!(false, xor(true, true));
    }
}
