/*
--- Day 5: Binary Boarding ---

You board your plane only to discover a new problem: you dropped your boarding pass! You aren't sure which seat is yours, and all of the flight attendants are busy with the flood of people that suddenly made it through passport control.

You write a quick program to use your phone's camera to scan all of the nearby boarding passes (your puzzle input); perhaps you can find your seat through process of elimination.

Instead of zones or groups, this airline uses binary space partitioning to seat people. A seat might be specified like FBFBBFFRLR, where F means "front", B means "back", L means "left", and R means "right".

The first 7 characters will either be F or B; these specify exactly one of the 128 rows on the plane (numbered 0 through 127). Each letter tells you which half of a region the given seat is in. Start with the whole list of rows; the first letter indicates whether the seat is in the front (0 through 63) or the back (64 through 127). The next letter indicates which half of that region the seat is in, and so on until you're left with exactly one row.

For example, consider just the first seven characters of FBFBBFFRLR:

    Start by considering the whole range, rows 0 through 127.
    F means to take the lower half, keeping rows 0 through 63.
    B means to take the upper half, keeping rows 32 through 63.
    F means to take the lower half, keeping rows 32 through 47.
    B means to take the upper half, keeping rows 40 through 47.
    B keeps rows 44 through 47.
    F keeps rows 44 through 45.
    The final F keeps the lower of the two, row 44.

The last three characters will be either L or R; these specify exactly one of the 8 columns of seats on the plane (numbered 0 through 7). The same process as above proceeds again, this time with only three steps. L means to keep the lower half, while R means to keep the upper half.

For example, consider just the last 3 characters of FBFBBFFRLR:

    Start by considering the whole range, columns 0 through 7.
    R means to take the upper half, keeping columns 4 through 7.
    L means to take the lower half, keeping columns 4 through 5.
    The final R keeps the upper of the two, column 5.

So, decoding FBFBBFFRLR reveals that it is the seat at row 44, column 5.

Every seat also has a unique seat ID: multiply the row by 8, then add the column. In this example, the seat has ID 44 * 8 + 5 = 357.

Here are some other boarding passes:

    BFFFBBFRRR: row 70, column 7, seat ID 567.
    FFFBBBFRRR: row 14, column 7, seat ID 119.
    BBFFBBFRLL: row 102, column 4, seat ID 820.

As a sanity check, look through your list of boarding passes. What is the highest seat ID on a boarding pass?

*/

/*
Ding! The "fasten seat belt" signs have turned on. Time to find your seat.

It's a completely full flight, so your seat should be the only missing boarding pass in your list. However, there's a catch: some of the seats at the very front and back of the plane don't exist on this aircraft, so they'll be missing from your list as well.

Your seat wasn't at the very front or back, though; the seats with IDs +1 and -1 from yours will be in your list.

What is the ID of your seat?
*/

static INPUT: &str = include_str!("day05-input.txt");

fn main() {
    let mut used_seats: [bool; 109 * 8] = [false; 109 * 8]; // rows ranges from 9 to 108

    let mut max_seat_id = 0;
    let mut min_row = 128;
    let mut max_row = 0;

    for line in INPUT.lines() {
        let (row, _, seat_id) = parse_seat(line);
        min_row = min_row.min(row);
        max_row = max_row.max(row);
        max_seat_id = max_seat_id.max(seat_id);
        used_seats[seat_id as usize] = true;
    }

    // Print seat plan
    // for row in used_seats.chunks(8) {
    //     for seat in row.iter() {
    //         print!("{}", (if *seat { "#" } else { "." }));
    //     }
    //     println!("");
    // }

    let a = used_seats.get(0..).unwrap().iter();
    let b = used_seats.get(1..).unwrap().iter();
    let c = used_seats.get(2..).unwrap().iter();
    let your_seat_id = a
        .zip(b)
        .zip(c)
        .enumerate()
        .find(|(_, ((pre, cur), next))| **pre && !**cur && **next)
        .unwrap()
        .0
        + 1;

    println!("Your seat id: {:?}", your_seat_id);
    println!("Max seat id: {}", max_seat_id);
    println!("Min:Max row id: {}:{}", min_row, max_row);
}

fn parse_bin(s: &str, on: char) -> u8 {
    let exp = s.len() as u32 - 1;
    s.chars()
        .enumerate()
        .map(|(i, c)| {
            if c == on {
                (2 as u8).pow(exp - i as u32)
            } else {
                0
            }
        })
        .sum::<u8>()
}

fn parse_seat(s: &str) -> (u8, u8, u16) {
    let row = parse_bin(&s.get(0..7).unwrap(), 'B');
    let col = parse_bin(&s.get(7..).unwrap(), 'R');

    (row, col, row as u16 * 8 + col as u16)
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    Here are some other boarding passes:

        BFFFBBFRRR: row 70, column 7, seat ID 567.
        FFFBBBFRRR: row 14, column 7, seat ID 119.
        BBFFBBFRLL: row 102, column 4, seat ID 820.
        */

    #[test]
    fn test_basic_seats() {
        const A: &str = "BFFFBBFRRR";
        const B: &str = "FFFBBBFRRR";
        const C: &str = "BBFFBBFRLL";

        {
            let (row, col, seat_id) = parse_seat(A);
            assert_eq!(row, 70);
            assert_eq!(col, 7);
            assert_eq!(seat_id, 567);
        }

        {
            let (row, col, seat_id) = parse_seat(B);
            assert_eq!(row, 14);
            assert_eq!(col, 7);
            assert_eq!(seat_id, 119);
        }

        {
            let (row, col, seat_id) = parse_seat(C);
            assert_eq!(row, 102);
            assert_eq!(col, 4);
            assert_eq!(seat_id, 820);
        }
    }
}
