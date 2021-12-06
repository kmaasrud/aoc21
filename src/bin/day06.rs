use std::iter::repeat;

fn how_many_fish(fish: &[usize], days: usize) -> usize {
    let mut fish = fish.to_owned();
    for _ in 0..days {
        let new_fish_iter = repeat(8).take(fish.iter().filter(|x| **x == 0).count());

        fish.iter_mut()
            .for_each(|fish| *fish = if *fish == 0 { 6 } else { *fish - 1 });

        fish.extend(new_fish_iter);
    }
    fish.len()
}

fn main() {
    let fish = load_lanternfish("inputs/day06.txt");

    println!("First answer: {}", how_many_fish(&fish, 80));
}

fn load_lanternfish(path: &str) -> Vec<usize> {
    std::fs::read_to_string(path)
        .unwrap()
        .split(',')
        .map(|x| x.trim().parse::<usize>().unwrap())
        .collect()
}

#[test]
fn test() {
    let fish = load_lanternfish("inputs/day06_test.txt");

    assert_eq!(how_many_fish(&fish, 18), 26);
    assert_eq!(how_many_fish(&fish, 80), 5934);
    // assert_eq!(how_many_fish(&fish, 256), 26984457539);
}
