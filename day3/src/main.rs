use std::fs;
use regex::Regex;

fn evaluate_operation(op: &str) -> i32 {
    let mut nums = op.split("(").nth(1).unwrap().split(",");
    let a = nums.next().unwrap().parse::<i32>().unwrap();
    let b = nums.next().unwrap().split(")").next().unwrap().parse::<i32>().unwrap();
    a * b
}


fn main() -> std::io::Result<()> {
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let mut res = 0;
    let contents = fs::read_to_string("src/input.txt")?;
    let operations: Vec<&str> = re.find_iter(contents.as_str()).map(|op| op.as_str()).collect();
    for value in operations {
        res += evaluate_operation(value)
    }
    println!("Result: {}", res);
    Ok(())
}
