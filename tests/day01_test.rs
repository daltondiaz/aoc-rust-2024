use std::fs::File;

#[test]
fn test_day_01_demo(){
    let file = File::open("input/day01/demo1.in");
    let res = aoc_rust_2024::days::day01::solution(file).unwrap();
    let exp = 11;
    assert_eq!(exp, res);
}
#[test]
fn test_day_01_part_one(){
    let file = File::open("input/day01/data.in");
    let res = aoc_rust_2024::days::day01::solution(file).unwrap();
    let exp = 2756096;
    assert_eq!(exp, res);
}
