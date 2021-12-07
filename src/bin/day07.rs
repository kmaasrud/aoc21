fn a(crab_pos: &[i32]) -> i32 {
    // We need the geometric median: https://en.wikipedia.org/wiki/Geometric_median
    // In the one-dimensional case, this is the regular median
    let mut list = crab_pos.to_owned();
    list.sort_unstable(); 
    let median = list[crab_pos.len() / 2];

    crab_pos.iter()
        .fold(0, |acc, x| acc + (x - median).abs())
}

fn main() {
    let crab_pos = load_crab_pos("inputs/day07.txt");

    println!("First answer: {}", a(&crab_pos));
}

fn load_crab_pos(path: &str) -> Vec<i32> {
    std::fs::read_to_string(path)
        .unwrap()
        .split(',')
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect()
}

#[test]
fn test() {
    let crab_pos = load_crab_pos("inputs/day07_test.txt");

    assert_eq!(a(&crab_pos), 37);
}
