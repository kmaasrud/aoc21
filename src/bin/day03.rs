fn a(log2: usize, nums: &Vec<usize>) -> usize {
    let mut gamma = 0;
    for i in 0..=(log2 + 1) {
        let bit = 1 << i;
        let mut ones = 0;
        for num in nums {
            if (num & bit) == bit {
                ones += 1;
            }
        }
        if ones > nums.len() / 2 {
            gamma |= bit;
        }
    }
    let eps = (usize::MAX << log2) ^ !gamma;
    println!("Gamma: {:b}", gamma);
    println!("Epsilon: {:b}", eps);
    gamma * eps
}

fn main() {
    let (log2, nums) = load_lines("inputs/day03.txt");

    println!("First answer: {}", a(log2, &nums));
}

fn load_lines(path: &str) -> (usize, Vec<usize>) {
    let input = std::fs::read_to_string(path).unwrap();
    (
        input.lines().next().unwrap().len(),
        input
            .lines()
            .filter_map(|line| usize::from_str_radix(line, 2).ok())
            .collect(),
    )
}

#[test]
fn test() {
    let (log2, nums) = load_lines("inputs/day03_test.txt");

    assert_eq!(a(log2, &nums), 198);
}
