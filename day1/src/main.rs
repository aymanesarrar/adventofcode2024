use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("./src/input.txt")?;
    let reader = BufReader::new(file);
    let mut differences: Vec<i32> = Vec::new();
    let mut count: i32 = 0;
    let mut left_nums: Vec<i32> = Vec::new();
    let mut right_nums: Vec<i32> = Vec::new();
    let mut similarity_score: i32 = 0;
    for line in reader.lines() {
        match line {
            Ok(line) => {
                
                let parts: Vec<&str> =  line.split_whitespace().collect();
                left_nums.push(parts[0].parse().unwrap());
                right_nums.push(parts[1].parse().unwrap());
            },
            Err(e) => println!("Error: {}", e),
        }
    }


    for value in left_nums.iter_mut() {
        let mut score  = 0;

        for value2 in right_nums.iter_mut() {
            if value == value2 {
                score += 1;
            }
        }
        similarity_score += *value * score;
    }
    println!("{}", similarity_score);
    left_nums.sort_by(|a, b| b.cmp(a));
    right_nums.sort_by(|a, b| b.cmp(a));

    for (index, value) in left_nums.iter().enumerate() {
        differences.push((value - right_nums[index]).abs())
    }
    for value in differences {
        count += value;
    }
    println!("{}", count);
    Ok(())
}
