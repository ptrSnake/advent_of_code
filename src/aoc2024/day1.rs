use std::collections::HashMap;

use crate::utils::read_file;

pub fn solve() {
    let input = read_file("src/input/2024/1/input.txt");

    let mut first: Vec<i32> = Vec::new();
    let mut second: Vec<i32> = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let mut iter = line.split_whitespace();
        let first_value = iter.next().unwrap().parse::<i32>().unwrap();
        let second_value = iter.next().unwrap().parse::<i32>().unwrap();
        first.push(first_value);
        second.push(second_value);
    }

    first.sort();
    second.sort();

    let mut total = 0;

    for (first, second) in first.iter().zip(second.iter()) {
        let diff = first - second;

        total += diff.abs() as i32;
    }

    println!("1: Total: {}", total);

    let mut second_list_frequency: HashMap<i32, i32> = HashMap::new();

    for second_value in second.iter() {
        *second_list_frequency.entry(*second_value).or_insert(0) += 1;
    }

    total = 0;

    for first_value in first.iter() {
        if let Some(frequency) = second_list_frequency.get(first_value) {
            total += frequency * first_value;
        }
    }

    println!("2: Total: {}", total);
}
