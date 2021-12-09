fn a(map: &[Vec<usize>]) -> usize {
    let mut sum = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let points = match (y,x) {
                (0, 0) => vec![(y+1, x), (y, x+1)],
                (0, _) => vec![(y+1, x), (y, x+1), (y, x-1)],
                (_, 0) => vec![(y+1, x), (y, x+1), (y-1, x)],
                _ => vec![(y+1, x), (y, x+1), (y-1, x), (y, x-1)],
            };

            let adjacent: Vec<&usize> = points
                .iter()
                .filter_map(|(y, x)| if let Some(row) = map.get(*y) {
                    row.get(*x)
                } else {
                    None
                })
                .collect();

            let this = map[y][x];
            if adjacent.iter().all(|x| **x > this) {
                sum += 1 + this;
            }
        }
    }
    sum
}

fn main() {
    let map = load_map("inputs/day09.txt");

    println!("First answer: {}", a(&map));
}

fn load_map(path: &str) -> Vec<Vec<usize>> {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            line.chars().map(|c| c.to_string().parse::<usize>().unwrap()).collect()
        })
        .collect()
}

#[test]
fn test() {
    let map = load_map("inputs/day09_test.txt");

    assert_eq!(a(&map), 15);
}
