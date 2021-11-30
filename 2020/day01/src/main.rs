use std::fs::File;
use std::io::Read;

fn main() {
    let mut buf = String::new();

    File::open("input.txt")
        .unwrap()
        .read_to_string(&mut buf)
        .unwrap();

    let mut nums: Vec<usize> = vec![];
    for line in buf.lines() {
        nums.push(line.parse().unwrap())
    }
    
    for (i, num1) in nums.iter().enumerate() {
        for (j, num2) in nums[i..].iter().enumerate() {
            for num3 in nums[j..].iter() {
                if num1 + num2 + num3 == 2020 {
                    println!("Answer: {}", num1 * num2 * num3);
                }
            }
        } 
    }
}
