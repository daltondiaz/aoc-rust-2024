use std::fs::File;

pub mod days;

pub fn main() {
    println!("AOC 2024 in Rust");
    let file = File::open("input/day01/demo1.in");
    let day_01_demo = days::day01::solution(file).unwrap();
    println!("Day01\t demo:{day_01_demo}");
    let file = File::open("input/day01/data.in");
    let day_01_part_01 = days::day01::solution(file).unwrap();
    println!("Day01\t first part:{day_01_part_01}");
}
