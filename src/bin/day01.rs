use std::fs::File;
use std::io::Read;

fn a(nums: Vec<usize>) -> usize {
    nums.iter()
        .fold((0, usize::MAX), |(sum, prev), next| {
            (if prev < *next { sum + 1 } else { sum }, *next)
        })
        .0
}

fn b(nums: Vec<usize>) -> usize {
    let mut nums = nums.iter();
    let a = nums.next().unwrap();
    let b = nums.next().unwrap();
    let c = nums.next().unwrap();
    nums.fold((0, a, b, c), |(sum, a, b, c), d| {
        if (a + b + c) < (b + c + d) {
            (sum + 1, b, c, d)
        } else {
            (sum, b, c, d)
        }
    })
    .0
}

fn main() {
    let nums = load_lines("inputs/day01.txt");

    println!("First answer: {}", a(nums.clone()));

    println!("Second answer: {}", b(nums));
}

fn load_lines(path: &str) -> Vec<usize> {
    let mut buf = String::new();
    File::open(path).unwrap().read_to_string(&mut buf).unwrap();

    buf.lines()
        .filter_map(|line| line.parse::<usize>().ok())
        .collect()
}

#[test]
fn test() {
    let nums = load_lines("inputs/day01_test.txt");
    assert_eq!(a(nums.clone()), 7);
    assert_eq!(b(nums), 5);
}
