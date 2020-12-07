/*
--- Day 6: Custom Customs ---

As your flight approaches the regional airport where you'll switch to a much larger plane, customs declaration forms are distributed to the passengers.

The form asks a series of 26 yes-or-no questions marked a through z. All you need to do is identify the questions for which anyone in your group answers "yes". Since your group is just you, this doesn't take very long.

However, the person sitting next to you seems to be experiencing a language barrier and asks if you can help. For each of the people in their group, you write down the questions for which they answer "yes", one per line. For example:

abcx
abcy
abcz

In this group, there are 6 questions to which anyone answered "yes": a, b, c, x, y, and z. (Duplicate answers to the same question don't count extra; each question counts at most once.)

Another group asks for your help, then another, and eventually you've collected answers from every group on the plane (your puzzle input). Each group's answers are separated by a blank line, and within each group, each person's answers are on a single line. For example:

abc

a
b
c

ab
ac

a
a
a
a

b

This list represents answers from five groups:

    The first group contains one person who answered "yes" to 3 questions: a, b, and c.
    The second group contains three people; combined, they answered "yes" to 3 questions: a, b, and c.
    The third group contains two people; combined, they answered "yes" to 3 questions: a, b, and c.
    The fourth group contains four people; combined, they answered "yes" to only 1 question, a.
    The last group contains one person who answered "yes" to only 1 question, b.

In this example, the sum of these counts is 3 + 3 + 3 + 1 + 1 = 11.

For each group, count the number of questions to which anyone answered "yes". What is the sum of those counts?

*/

/*
--- Part Two ---

As you finish the last group's customs declaration, you notice that you misread one word in the instructions:

You don't need to identify the questions to which anyone answered "yes"; you need to identify the questions to which everyone answered "yes"!

Using the same example as above:

abc

a
b
c

ab
ac

a
a
a
a

b

This list represents answers from five groups:

    In the first group, everyone (all 1 person) answered "yes" to 3 questions: a, b, and c.
    In the second group, there is no question to which everyone answered "yes".
    In the third group, everyone answered yes to only 1 question, a. Since some people did not answer "yes" to b or c, they don't count.
    In the fourth group, everyone answered yes to only 1 question, a.
    In the fifth group, everyone (all 1 person) answered "yes" to 1 question, b.

In this example, the sum of these counts is 3 + 0 + 1 + 1 + 1 = 6.

For each group, count the number of questions to which everyone answered "yes". What is the sum of those counts?

*/

use std::collections::HashSet;

static INPUT: &str = include_str!("day06-input.txt");

fn is_answer(c: &char) -> bool {
    c.is_ascii_lowercase()
}

fn count_yeses(group: &str) -> usize {
    group
        .chars()
        .filter(is_answer)
        .collect::<HashSet<char>>()
        .len()
}

fn count_total_yeses(input: &str) -> usize {
    let mut total = 0;
    for group in input.replace("\r\n", "\n").split("\n\n") {
        let yeses: HashSet<char> = group.chars().filter(is_answer).collect();
      //  println!("{}: {:?}", yeses.len(), yeses);
        total += count_yeses(group);
    }
    total
}

fn count_all_yeses(group: &str) -> usize {
    let alpha_num: HashSet<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

    group
        .lines()
        .map(|person| person.chars().collect::<HashSet<char>>())
        .fold(alpha_num, |acum, cur| {
            acum.intersection(&cur).copied().collect::<HashSet<char>>()
        })
        .len()
}

fn count_all_total_yeses(input: &str) -> usize {
    let mut total = 0;
    for group in input.replace("\r\n", "\n").split("\n\n") {
        let yeses: HashSet<char> = group.chars().filter(is_answer).collect();
       // println!("{}: {:?}", yeses.len(), yeses);
        total += count_all_yeses(group);
    }
    total
}

fn main() {
    println!("Total - part 1: {}", count_total_yeses(INPUT));
    println!("Total - part 2: {}", count_all_total_yeses(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = include_str!("day06-test-input.txt");

    #[test]
    fn group_parsing() {
        assert_eq!(3, count_yeses("abc"));
        assert_eq!(3, count_yeses("a\nb\nc"));
        assert_eq!(1, count_yeses("aaaa"));
        assert_eq!(2, count_yeses("ababaa"));
    }

    #[test]
    fn group_summing() {
        assert_eq!(11, count_total_yeses(TEST_INPUT));
    }

    #[test]
    fn group_all_summing() {
        assert_eq!(6, count_total_yeses(TEST_INPUT));
    }
}
