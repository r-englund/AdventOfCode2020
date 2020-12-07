/*
--- Day 7: Handy Haversacks ---

You land at the regional airport in time for your next flight. In fact, it looks like you'll even have time to grab some food: all flights are currently delayed due to issues in luggage processing.

Due to recent aviation regulations, many rules (your puzzle input) are being enforced about bags and their contents; bags must be color-coded and must contain specific quantities of other color-coded bags. Apparently, nobody responsible for these regulations considered how long they would take to enforce!

For example, consider the following rules:

light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.

These rules specify the required contents for 9 bag types. In this example, every faded blue bag is empty, every vibrant plum bag contains 11 bags (5 faded blue and 6 dotted black), and so on.

You have a shiny gold bag. If you wanted to carry it in at least one other bag, how many different bag colors would be valid for the outermost bag? (In other words: how many colors can, eventually, contain at least one shiny gold bag?)

In the above rules, the following options would be available to you:

    A bright white bag, which can hold your shiny gold bag directly.
    A muted yellow bag, which can hold your shiny gold bag directly, plus some other bags.
    A dark orange bag, which can hold bright white and muted yellow bags, either of which could then hold your shiny gold bag.
    A light red bag, which can hold bright white and muted yellow bags, either of which could then hold your shiny gold bag.

So, in this example, the number of bag colors that can eventually contain at least one shiny gold bag is 4.

How many bag colors can eventually contain at least one shiny gold bag? (The list of rules is quite long; make sure you get all of it.)

*/

/*
--- Part Two ---

It's getting pretty expensive to fly these days - not because of ticket prices, but because of the ridiculous number of bags you need to buy!

Consider again your shiny gold bag and the rules from the above example:

    faded blue bags contain 0 other bags.
    dotted black bags contain 0 other bags.
    vibrant plum bags contain 11 other bags: 5 faded blue bags and 6 dotted black bags.
    dark olive bags contain 7 other bags: 3 faded blue bags and 4 dotted black bags.

So, a single shiny gold bag must contain 1 dark olive bag (and the 7 bags within it) plus 2 vibrant plum bags (and the 11 bags within each of those): 1 + 1*7 + 2 + 2*11 = 32 bags!

Of course, the actual rules have a small chance of going several levels deeper than this example; be sure to count all of the bags, even if the nesting becomes topologically impractical!

Here's another example:

shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.

In this example, a single shiny gold bag must contain 126 other bags.

How many individual bags are required inside your single shiny gold bag?

*/

#![feature(str_split_once)]

use std::collections::HashSet;
use std::{collections::HashMap, string::String};

static INPUT: &str = include_str!("day07-input.txt");

fn parse_line(rule: &str) -> (&str, Vec<(usize, &str)>) {
    let splited: Vec<&str> = rule.split(" bags contain ").collect();
    assert_eq!(2, splited.len());
    let bag_color = *splited.first().unwrap();
    let can_contain = splited.get(1).unwrap().strip_suffix(".").unwrap();

    if can_contain == "no other bags" {
        (bag_color, vec![])
    } else {
        let res = can_contain
            .split(", ")
            .map(|a| {
                let (count, bag_name): (&str, &str) = a
                    .trim_end_matches("s")
                    .trim_end_matches("bag")
                    .split_once(" ")
                    .unwrap();
                (count.parse::<usize>().unwrap(), bag_name.trim())
            })
            .collect();
        (bag_color, res)
    }
}

fn count_bags_helper(
    bag_can_be_in: &HashMap<&str, Vec<(usize, &str)>>,
    bag_type: &str,
    mut set: &mut HashSet<String>,
) -> usize {
    if let Some(can_be_in) = bag_can_be_in.get(bag_type) {
        can_be_in.iter().for_each(|(_, s)| {
            if set.insert(s.to_string()) {
                count_bags_helper(bag_can_be_in, s, &mut set);
            }
        });
    }
    set.len()
}

fn count_bags(rules: &str) -> usize {
    let mut bag_can_be_in: HashMap<&str, Vec<(usize, &str)>> = HashMap::new();

    rules.lines().map(parse_line).for_each(|(bag, can_carry)| {
        for (count, bagname) in can_carry {
            bag_can_be_in
                .entry(bagname)
                .or_insert(Vec::new())
                .push((count, bag));
        }
    });

    let mut tmp: HashSet<String> = HashSet::new();
    count_bags_helper(&bag_can_be_in, "shiny gold", &mut tmp)
}

fn count_sub_bags_inside_helper(
    bag_contains: &HashMap<&str, Vec<(usize, &str)>>,
    bag: &str,
) -> usize {
    if let Some(a) = bag_contains.get(bag) {
        a.iter()
            .map(|(count, b)| {
                assert_ne!(*count, 0);
                *count * (count_sub_bags_inside_helper(&bag_contains, b) + 1)
            })
            .sum::<usize>()
    } else {
        0
    }
}

fn count_bags_inside(rules: &str) -> usize {
    let bag_contains: HashMap<_, _> = rules.lines().map(parse_line).collect();
    count_sub_bags_inside_helper(&bag_contains, "shiny gold")
}

fn main() {
    println!("Can carry shiny gold: {}", count_bags(INPUT));
    println!("Are inside a shiny gold: {}", count_bags_inside(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT1: &str = include_str!("day07-test-input1.txt");
    static TEST_INPUT2: &str = include_str!("day07-test-input2.txt");

    #[test]
    fn test_part1() {
        assert_eq!(4, count_bags(TEST_INPUT1));
        assert_eq!(164, count_bags(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(32, count_bags_inside(TEST_INPUT1));
        assert_eq!(126, count_bags_inside(TEST_INPUT2));
        assert_eq!(7872, count_bags_inside(INPUT));
    }
}
