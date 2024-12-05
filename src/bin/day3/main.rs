use regex::Regex;

use std::fs;

fn main() {
    let input = fs::read_to_string("src/bin/day3/input").unwrap();

    fn sum_of_muls(input: &str) -> i32 {
        Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")
            .unwrap()
            .captures_iter(input)
            .map(|cap| cap[1].parse::<i32>().unwrap() * cap[2].parse::<i32>().unwrap())
            .sum()
    }

    println!("part one: {:#?}", sum_of_muls(&input));

    let mut segments = input.split("don't()");
    let initial = sum_of_muls(segments.next().unwrap_or(""));
    let part_two = segments.fold(initial, |acc, segment| {
        acc + sum_of_muls(&segment[segment.find("do()").unwrap_or(segment.len())..])
    });

    println!("part two: {:#?}", part_two);
}
