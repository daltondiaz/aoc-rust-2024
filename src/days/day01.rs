use std::{
    fs::File, i32, io::{self, BufRead, BufReader}
};

pub fn solution(file: io::Result<File>) -> io::Result<i32> {
    let reader = BufReader::new(file?);

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        let l: i32 = parts[0].parse().unwrap();
        let r: i32 = parts[1].parse().unwrap();
        //println!("line: {} \t left {} \t right {}", line, l, r);
        left = sort(l, left);
        //println!("left arr: {:?}", left);
        right = sort(r, right);
        //println!("right arr: {:?}", right);
    }

    let mut sum :i32 = 0;
    for i in 0..right.len() {
        sum += i32::abs(right[i]-left[i])
    }
    Ok(sum)
}
/*
* I implemented this sort only to learn more about rust and practice!
*/
fn sort(val: i32, mut arr: Vec<i32>) -> Vec<i32> {
    arr.push(val);
    //print!("value of array: ");
    let len = arr.len();
    for i in 0..len {
        for j in 0..len- i -1{
            if arr[j] > arr[j + 1] {
                arr.swap(j, j+1);
            }
        }
    }
    //println!();
    arr
}
