use std::fs::File;
use std::io::Read;

fn main () {
    let mut buf = String::new();
    File::open("inputs/day01.txt")
        .unwrap()
        .read_to_string(&mut buf)
        .unwrap();

    let list = buf
        .lines()
        .filter_map(|line| line.parse::<usize>().ok());

    let answer = list
        .clone()
        .zip(list.clone().skip(1))
        .fold(0, |acc, (a, b)| {
            if a < b {
                acc + 1
            } else {
                acc
            }
        });

    println!("First answer: {}", answer);

    let answer = list.clone()
        .zip(list.clone().skip(1))
        .zip(list.clone().skip(2))
        .zip(list.skip(3))
        .fold(0, |acc, (((a, b), c), d)| {
            if (a + b + c) < (b + c + d) {
                acc + 1
            } else {
                acc
            }
        });

    println!("Second answer: {}", answer);
}
