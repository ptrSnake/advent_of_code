use advent_of_code::aoc2024::*;

fn main() {
    let start = std::time::Instant::now();
    day1::solve();
    day2::solve();
    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}
