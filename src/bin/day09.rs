fn a(map: &[Vec<usize>]) -> usize {
    let maxx = map[0].len() - 1;
    let maxy = map.len() - 1;
    let mut sum = 0;
    for y in 1..maxy {
        for x in 1..maxx {
            let this = map[y][x];
            let surrounding = [(y + 1, x), (y, x + 1), (y - 1, x), (y, x - 1)];
            if surrounding.iter().all(|(y, x)| map[*y][*x] > this) {
                sum += 1 + this;
            }
        }
    }
    sum
}

fn b(map: &[Vec<usize>]) -> usize {
    type Point = (usize, usize);
    type Basin = Vec<Point>;
    let maxx = map[0].len() - 1;
    let maxy = map.len() - 1;

    // Find coordinates of lowest points
    let mut lowest_coords: Vec<(usize, usize)> = Vec::new();
    for y in 1..maxy {
        for x in 1..maxx {
            let this = map[y][x];
            let surrounding = [(y + 1, x), (y, x + 1), (y - 1, x), (y, x - 1)];
            if surrounding.iter().all(|(y, x)| map[*y][*x] > this) {
                lowest_coords.push((y, x));
            }
        }
    }

    // Find a basin recursively from a given lowest point
    fn find_basin(y: usize, x: usize, mut basin: &mut Basin, map: &[Vec<usize>]) {
        // All non-duplicate surrounding points bigger than (x, y) and not equal to 9
        let points: Vec<Point> = [(y + 1, x), (y, x + 1), (y - 1, x), (y, x - 1)]
            .iter()
            .filter(|(y, x)| !basin.contains(&(*y, *x)))
            .filter(|(y, x)| map[*y][*x] != 9)
            .filter(|(ynew, xnew)| map[*ynew][*xnew] > map[y][x])
            .cloned()
            .collect();

        if points.len() > 0 {
            basin.append(&mut points.clone());
            points
                .iter()
                .for_each(|(y, x)| find_basin(*y, *x, &mut basin, map));
        }
    }

    // Make list of basin sizes
    let mut basin_sizes: Vec<usize> = lowest_coords
        .iter()
        .map(|(y, x)| {
            let mut basin = vec![(*y, *x)];
            find_basin(*y, *x, &mut basin, map);
            basin.len()
        })
        .collect();

    basin_sizes.sort_unstable();
    basin_sizes.iter().rev().take(3).product()
}

fn main() {
    let map = load_map("inputs/day09.txt");

    println!("First answer: {}", a(&map));
    println!("Second answer: {}", b(&map));
}

fn load_map(path: &str) -> Vec<Vec<usize>> {
    let mut map: Vec<Vec<usize>> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    // Surround with 9's
    map = [vec![vec![9; map[0].len()]], map].concat();
    map.iter_mut()
        .for_each(|row| *row = [vec![9], row.to_vec()].concat());
    map.iter_mut().for_each(|row| row.push(9));
    map.push(vec![9; map[0].len()]);

    map
}

#[test]
fn test() {
    let map = load_map("inputs/day09_test.txt");

    assert_eq!(a(&map), 15);
    assert_eq!(b(&map), 1134);
}
