/*
--- Day 9: Encoding Error ---

With your neighbor happily enjoying their video game, you turn your attention to an open data port on the little screen in the seat in front of you.

Though the port is non-standard, you manage to connect it to your computer through the clever use of several paperclips. Upon connection, the port outputs a series of numbers (your puzzle input).

The data appears to be encrypted with the eXchange-Masking Addition System (XMAS) which, conveniently for you, is an old cypher with an important weakness.

XMAS starts by transmitting a preamble of 25 numbers. After that, each number you receive should be the sum of any two of the 25 immediately previous numbers. The two numbers will have different values, and there might be more than one such pair.

For example, suppose your preamble consists of the numbers 1 through 25 in a random order. To be valid, the next number must be the sum of two of those numbers:

    26 would be a valid next number, as it could be 1 plus 25 (or many other pairs, like 2 and 24).
    49 would be a valid next number, as it is the sum of 24 and 25.
    100 would not be valid; no two of the previous 25 numbers sum to 100.
    50 would also not be valid; although 25 appears in the previous 25 numbers, the two numbers in the pair must be different.

Suppose the 26th number is 45, and the first number (no longer an option, as it is more than 25 numbers ago) was 20. Now, for the next number to be valid, there needs to be some pair of numbers among 1-19, 21-25, or 45 that add up to it:

    26 would still be a valid next number, as 1 and 25 are still within the previous 25 numbers.
    65 would not be valid, as no two of the available numbers sum to it.
    64 and 66 would both be valid, as they are the result of 19+45 and 21+45 respectively.

Here is a larger example which only considers the previous 5 numbers (and has a preamble of length 5):

35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576

In this example, after the 5-number preamble, almost every number is the sum of two of the previous 5 numbers; the only number that does not follow this rule is 127.

The first step of attacking the weakness in the XMAS data is to find the first number in the list (after the preamble) which is not the sum of two of the 25 numbers before it. What is the first number that does not have this property?

*/

/*
The final step in breaking the XMAS encryption relies on the invalid number you just found: you must find a contiguous set of at least two numbers in your list which sum to the invalid number from step 1.

Again consider the above example:

35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576

In this list, adding up all of the numbers from 15 through 40 produces the invalid number from step 1, 127. (Of course, the contiguous set of numbers in your actual list might be much longer.)

To find the encryption weakness, add together the smallest and largest number in this contiguous range; in this example, these are 15 and 47, producing 62.

What is the encryption weakness in your XMAS-encrypted list of numbers?
*/

use itertools::Itertools;

static INPUT: &str = include_str!("day09-input.txt");

fn main() {
    let nums = to_numbers_iter(INPUT);
    let res = find_num(&nums, 25);
    let part_2 = part_2(&nums, res);
    println!("First number not addable by N previous {}", res);
    println!(
        "Min and max of the numbers adding up to that number: {}",
        part_2
    );
}

fn find_num(numbers: &Vec<u64>, premable_length: usize) -> u64 {
    let res = numbers.windows(premable_length + 1).find(|slice| {
        let target = *slice.last().unwrap();
        slice
            .get(0..premable_length)
            .unwrap()
            .iter()
            .combinations(2)
            .find(|x| x[0] + x[1] == target)
            .is_none()
    });

    *res.unwrap().last().unwrap()
}

fn to_numbers_iter(input: &str) -> Vec<u64> {
    input.lines().map(|x| x.parse::<u64>().unwrap()).collect()
}
fn part_2(numbers: &Vec<u64>, target: u64) -> u64 {
    let mut i = 0;
    let mut j = 1;
    let mut sum: u64 = numbers.iter().take(2).sum();
    loop {
        if sum == target {
            break;
        } else if sum > target {
            sum = sum - numbers[i];
            i += 1;
        } else {
            j += 1;
            sum = sum + numbers[j];
        }
    }

    match numbers.get(i..=j).unwrap().iter().minmax() {
        itertools::MinMaxResult::NoElements => 0,
        itertools::MinMaxResult::OneElement(a) => *a,
        itertools::MinMaxResult::MinMax(min, max) => min + max,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = include_str!("day09-test-input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(127, find_num(&to_numbers_iter(TEST_INPUT), 5));
        assert_eq!(88311122, find_num(&to_numbers_iter(INPUT), 25));
    }

    #[test]
    fn test_part2() {
        {
            let nums = to_numbers_iter(TEST_INPUT);
            let num = find_num(&nums, 5);
            assert_eq!(62, part_2(&nums, num));
        }
        {
            let nums = to_numbers_iter(INPUT);
            let num = find_num(&nums, 25);
            assert_eq!(13549369, part_2(&nums, num));
        }

        
    }
}
