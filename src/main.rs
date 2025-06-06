use advent_of_code::aoc2024::day1;

fn main() {
    let start = std::time::Instant::now();
    day1::solve();
    let duration = start.elapsed();
    println!("Execution time: {:?}", duration);
}
