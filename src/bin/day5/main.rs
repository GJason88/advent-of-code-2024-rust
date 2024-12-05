use std::collections::{HashMap, HashSet};

use advent_of_code::read_lines;
use itertools::enumerate;

fn main() {
    let mapping_input = read_lines("src/bin/day5/input1").unwrap();

    let mut map = HashMap::new();
    mapping_input.for_each(|line| {
        let mapping = line.unwrap().split("|").map(|x| x.parse().unwrap()).collect::<Vec<i8>>();
        let k = mapping[0];
        let v_val = mapping[1];
        let v = map.entry(k).or_insert(HashSet::<i8>::new());
        v.insert(v_val);
    });

    let row_input = read_lines("src/bin/day5/input2").unwrap();

    let part_one = row_input.fold(0, |acc, line| {
        let line = parse_line(line.unwrap());
        if is_safe(&map, &line) {
            return acc + line[line.len() / 2] as i32;
        }
        acc
    });

    println!("part 1: {}", part_one);

    let row_input = read_lines("src/bin/day5/input2").unwrap();

    let part_two = row_input.fold(0, |acc, line| {
        let parsed = parse_line(line.unwrap());
        let is_safe = is_safe(&map, &parsed);
        if !is_safe {
            let x = make_safe(&map, &parsed);
            if x.len() > 0 {
                return acc + x[x.len() / 2] as i32;
            }
        }
        acc
    });

    println!("part 2: {}", part_two)
}

fn parse_line(line: String) -> Vec<i8> {
    line.split(",").filter(|num_str| num_str.len() > 0).map(|num_str| num_str.parse().unwrap()).collect()
}

fn is_safe(map: &HashMap<i8, HashSet<i8>>, line: &Vec<i8>) -> bool {
    if line.len() == 0 {
        return false;
    }
    for (i, val) in enumerate(line) {
        for j in 0..i {
            if map[&val].contains(&line[j]) {
                return false;
            }
        }
        for j in i+1..line.len() {
            if !map[&val].contains(&line[j]) {
                return false;
            }
        }
    }
    true
}

fn make_safe(map: &HashMap<i8, HashSet<i8>>, line: &Vec<i8>) -> Vec<i8> {
    let line_set: HashSet<i8> = line.clone().into_iter().collect();
    let mut score_to_num = HashMap::new();
    for num in line {
        let post_set = &map[&num];
        score_to_num.insert(line_set.intersection(post_set).collect::<Vec<&i8>>().len(), num);
    }

    let mut new_line = line.clone();
    for (k, v) in score_to_num.into_iter() {
        new_line[k] = *v;
    }

    new_line
}
