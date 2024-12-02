use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

type Pair = (i32, i32);

fn main() {
    let lines = read_lines("src/bin/day1/input").unwrap();

    let (first_ids, second_ids) = get_sorted_ids(lines);

    println!(
        "total distance: {}",
        get_total_distance(&first_ids, &second_ids)
    );
    println!(
        "similarity score: {}",
        get_similarity_score(&first_ids, &second_ids)
    );
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_sorted_ids(lines: Lines<BufReader<File>>) -> (Vec<i32>, Vec<i32>) {
    let mut first_ids = vec![];
    let mut second_ids = vec![];
    for line in lines {
        let id_pair = parse_line(&line);
        first_ids.push(id_pair.0);
        second_ids.push(id_pair.1);
    }
    first_ids.sort();
    second_ids.sort();

    (first_ids, second_ids)
}

fn get_similarity_score(first_ids: &Vec<i32>, second_ids: &Vec<i32>) -> i32 {
    let mut second_ids_counts = HashMap::new();
    for id in second_ids {
        *second_ids_counts.entry(id).or_insert(0) += 1;
    }

    first_ids.iter().fold(0, |acc, id| {
        acc + (id * second_ids_counts.get(id).unwrap_or(&0))
    })
}

fn get_total_distance(first_ids: &Vec<i32>, second_ids: &Vec<i32>) -> i32 {
    first_ids
        .iter()
        .zip(second_ids)
        .fold(0, |acc, id_pair| acc + (id_pair.0 - id_pair.1).abs())
}

fn parse_line(line_res: &Result<String, io::Error>) -> Pair {
    let ids: Vec<&str> = match line_res {
        Ok(line) => line.split_whitespace().collect(),
        Err(e) => panic!("Failed to iterate on line: {}", e),
    };

    if ids.len() != 2 {
        panic!("Unexpected number of elements in line: {:#?}", ids);
    };

    (ids[0].parse().unwrap(), ids[1].parse().unwrap())
}
