use crate::utils::read_file;

pub fn solve() {
    let input = read_file("src/input/2024/2/input.txt");

    let mut lines: Vec<Vec<i32>> = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let mut numbers: Vec<i32> = Vec::new();
        let mut iter = line.split_whitespace();

        for value in iter.by_ref() {
            if let Ok(num) = value.parse::<i32>() {
                numbers.push(num);
            }
        }

        if !numbers.is_empty() {
            lines.push(numbers);
        }
    }

    let mut safe = 0;

    for (index, vec) in lines.iter().enumerate() {
        if vec.len() > 1 {
            if vec.windows(2).all(|w| w[0] < w[1]) {
                println!(
                    "Vec at index {} is sorted in ascending order. {:?}",
                    index, vec
                );
                safe += 1;
            } else if vec.windows(2).all(|w| w[0] > w[1]) {
                println!(
                    "Vec at index {} is sorted in descending order. {:?}",
                    index, vec
                );
                safe += 1;
            } else {
                println!("Vec at index {} is not sorted. {:?}", index, vec);
            }
        } else {
            println!("Vec at index {} has only one value or is empty.", index);
        }
    }

    println!("Total safe vectors: {}", safe);
}
