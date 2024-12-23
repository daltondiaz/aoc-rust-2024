use std::{fs::File, io::{self, BufRead, BufReader}};

pub fn solution() -> io::Result<()>{
    //let mut file = File::open("input/day01/data.in")?;
    /*
    * TODO add this in one file separeted to be reusable
    */
    let file = File::open("input/day01/demo1.in")?;
    let reader = BufReader::new(file);

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in reader.lines(){
        let line = line?;
        println!("line: {}", line);
        let parts: Vec<&str> = line.split_whitespace().collect();
        println!("left");
        left = sort(parts[0].parse().unwrap(), left);
        println!("right");
        right = sort(parts[1].parse().unwrap(), right);
    }

    println!("{:?}", left);
    println!("{:?}", right);

    Ok(())
}

fn sort(val:i32,  arr:Vec<i32>) -> Vec<i32> {
    let mut new_arr: Vec<i32> = Vec::new();
    if arr.len() == 0 {
        println!("empty {}", val);
        new_arr.push(val);
        return new_arr
    }
    for value in arr {
        if val >= value{
            println!("value {}", value);
            new_arr.push(value);
        } else {
            println!("val {}", val);
            new_arr.push(val);
        }
    }
    new_arr
}

