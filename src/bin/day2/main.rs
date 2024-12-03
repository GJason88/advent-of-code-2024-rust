use advent_of_code::read_lines;

fn main() {
    let num_safe = read_lines("src/bin/day2/input")
        .unwrap()
        .fold(0, |acc, line| acc + is_safe(line.unwrap()));
    let num_safe_with_dampener = read_lines("src/bin/day2/input")
        .unwrap()
        .fold(0, |acc, line| acc + is_safe_with_dampener(line.unwrap()));
    println!("{}", num_safe);
    println!("{}", num_safe_with_dampener);
}

fn is_safe(line: String) -> i32 {
    let report: Vec<i32> = line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let inc = report
        .windows(2)
        .all(is_safe_increasing);
    let dec = report
        .windows(2)
        .all(is_safe_decreasing);

    (inc || dec) as i32
}

fn is_safe_with_dampener(line: String) -> i32 {
    let report: Vec<i32> = line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    for i in 0..report.len() {
        let inc = report
            .clone()
            .into_iter()
            .enumerate()
            .filter(|(j, _)| i != *j)
            .map(|(_, val)| val)
            .collect::<Vec<i32>>()
            .windows(2)
            .all(is_safe_increasing);
        let dec = report
            .clone()
            .into_iter()
            .enumerate()
            .filter(|(j, _)| i != *j)
            .map(|(_, val)| val)
            .collect::<Vec<i32>>()
            .windows(2)
            .all(is_safe_decreasing);

        if inc || dec {
            return 1;
        }
    }
    return 0;
}

fn is_safe_increasing(pair: &[i32]) -> bool {
    pair[1] > pair[0] && (pair[1] - pair[0] > 0) && (pair[1] - pair[0]) < 4
}

fn is_safe_decreasing(pair: &[i32]) -> bool {
    pair[0] > pair[1] && (pair[0] - pair[1] > 0) && (pair[0] - pair[1]) < 4
}
