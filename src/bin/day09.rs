fn a(map: &[Vec<usize>]) -> usize {
    let mut sum = 0;
    let maxx = map[0].len() - 1;
    let maxy = map.len() - 1;
    for y in 1..maxy {
        for x in 1..maxx {
            let this = map[y][x];
            let surrounding = [(y+1, x), (y, x+1), (y-1, x), (y, x-1)];
            if surrounding.iter().all(|(y, x)| map[*y][*x] > this) {
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
    let mut map: Vec<Vec<usize>> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            line.chars().map(|c| c.to_string().parse::<usize>().unwrap()).collect()
        })
        .collect();

    // Surround with 9's
    map = [vec![vec![9; map[0].len()]], map].concat();
    map.iter_mut().for_each(|row| *row = [vec![9], row.to_vec()].concat());
    map.iter_mut().for_each(|row| row.push(9));
    map.push(vec![9;map[0].len()]);

    map
}

#[test]
fn test() {
    let map = load_map("inputs/day09_test.txt");

    assert_eq!(a(&map), 15);
}
