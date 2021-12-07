fn a(crab_pos: &[i32]) -> i32 {
    // We need the geometric median: https://en.wikipedia.org/wiki/Geometric_median
    // In the one-dimensional case, this is the regular median
    let mut list = crab_pos.to_owned();
    list.sort_unstable(); 
    let median = list[crab_pos.len() / 2];

    crab_pos.iter()
        .fold(0, |acc, x| acc + (x - median).abs())
}

fn b(crab_pos: &[i32]) -> i32 {
    let mut minimized = i32::MAX;
    for mid in 0..*crab_pos.iter().max().unwrap() {
        let new = crab_pos.iter()
            .fold(0, |acc, x| acc + (1..=(x - mid).abs()).sum::<i32>());

        if new < minimized {
            minimized = new;
        }
    }
    minimized
}

fn main() {
    let crab_pos = load_crab_pos("inputs/day07.txt");

    println!("First answer: {}", a(&crab_pos));
    println!("Second answer: {}", b(&crab_pos));
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
    println!("Avg: {}", crab_pos.iter().sum::<i32>() / crab_pos.len() as i32);

    assert_eq!(a(&crab_pos), 37);
    assert_eq!(b(&crab_pos), 168);
}
