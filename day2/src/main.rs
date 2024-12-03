use std::fs::File;
use std::io::{self, BufRead, BufReader};



fn strs_to_int(strs: Vec<String>) -> Vec<i32> {
    let mut nums: Vec<i32> = Vec::new();
    for s in strs {
        nums.push(s.parse().unwrap());
    }
    return nums
}

// fn check_one_different(strs: Vec<&str>) -> bool {
//     let mut count = 0;
//     let first_item = strs[0];

//     for value in strs {
//         if value != first_item && value != "too long" {
//             count += 1;
//         }
//     }
//     return count == 1;
// }


fn main() -> io::Result<()> {
    let file = File::open("src/input.txt")?;
    let reader = BufReader::new(file);
    let mut safe_zones: Vec<String> = Vec::new();
    let mut unsafe_zones: Vec<String> = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let nums = strs_to_int(line.split_whitespace().map(String::from).collect());
                let mut arr = Vec::new();
                
               for i in 0..nums.len() - 1 {
                    if i == nums.len() - 1 {
                        break;
                    }
                    let diff = nums[i] - nums[i + 1];
                    if diff == 0 {
                        arr.push("so unsafe");
                    } else if diff > 0 && diff <= 3 {
                        arr.push("positive")
                    } else if diff < 0 && diff.abs() <= 3 {
                        arr.push("negative")
                    } else {
                        arr.push("too long");
                    }
                }
                if arr.iter().all(|x| arr[0] == *x) {
                    safe_zones.push(line.clone());
                } else {
                    unsafe_zones.push(line.clone());
                }
            },
            Err(e) => println!("Error: {}", e),
        }
    }
    println!("Safe zones: {:?}", safe_zones);
    println!("Unsafe zones: {:?}", unsafe_zones);
    println!("Safe zones: {}", safe_zones.len());
    println!("Unsafe zones: {}", unsafe_zones.len());
    Ok(())
    
}
