fn a(log2: usize, nums: &[usize]) -> usize {
    let mut gamma = 0;

    for i in 0..=(log2 + 1) {
        let bit = 1 << i;
        let ones = nums.iter().filter(|num| *num & bit == bit).count();

        if ones > nums.len() / 2 {
            gamma |= bit;
        }
    }

    let eps = (usize::MAX << log2) ^ !gamma;

    gamma * eps
}

fn b(log2: usize, nums: &[usize]) -> usize {
    let mut o2 = nums.to_owned();
    let mut co2 = nums.to_owned();

    for i in (0..=(log2 - 1)).rev() {
        let bit = 1 << i;

        let count_ones =
            |nums: &Vec<usize>| -> usize { nums.iter().filter(|num| *num & bit == bit).count() };

        let filter = |nums: &Vec<usize>, want: usize| -> Vec<usize> {
            nums.iter()
                .filter(|num| *num & bit == want)
                .map(|num| num.to_owned())
                .collect()
        };

        if o2.len() > 1 {
            let ones = count_ones(&o2);
            let zeros = o2.len() - ones;
            let want = if ones >= zeros { bit } else { 0 };
            o2 = filter(&o2, want);
        }

        if co2.len() > 1 {
            let ones = count_ones(&co2);
            let zeros = co2.len() - ones;
            let want = if ones < zeros { bit } else { 0 };
            co2 = filter(&co2, want);
        }
    }

    o2[0] * co2[0]
}

fn main() {
    let (log2, nums) = load_lines("inputs/day03.txt");

    println!("First answer: {}", a(log2, &nums));
    println!("Second answer: {}", b(log2, &nums));
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
    assert_eq!(b(log2, &nums), 230);
}
