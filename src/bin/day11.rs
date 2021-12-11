type Map = Vec<Vec<usize>>;
type Point = (usize, usize);

fn make_points(ymin: usize, ymax: usize, xmin: usize, xmax: usize) -> impl Iterator<Item = Point> {
    (ymin..=ymax)
        .map(move |y| (xmin..=xmax).map(move |x| (y, x)))
        .flatten()
}

fn update(map: &mut Map) -> usize {
    // Make queue of all points in grid
    let mut queue: Vec<Point> = make_points(0, map.len() - 1, 0, map[0].len() - 1).collect();

    let mut flashes = 0;
    while let Some((y, x)) = queue.pop() {
        map[y][x] += 1;
        if map[y][x] == 10 {
            flashes += 1;

            // Add surrounding points to queue
            make_points(
                y.saturating_sub(1),
                (y + 1).min(map.len() - 1),
                x.saturating_sub(1),
                (x + 1).min(map[0].len() - 1),
            )
            .filter(|(ynew, xnew)| *ynew != y || *xnew != x)
            .for_each(|point| queue.push(point));
        }
    }

    // Reset all flashes to energy level 0
    for (x, y) in make_points(0, map.len() - 1, 0, map[0].len() - 1).collect::<Vec<Point>>() {
        if map[y][x] > 9 {
            map[y][x] = 0;
        }
    }

    flashes
}

fn in_sync(map: &Map) -> bool {
    make_points(0, map.len() - 1, 0, map[0].len() - 1).all(|(y, x)| map[y][x] == 0)
}

fn main() {
    let mut map = load_map("inputs/day11.txt");

    let mut flashes = 0;
    for _ in 0..100 {
        flashes += update(&mut map);
    }

    println!("First answer: {}", flashes);

    for i in 100.. {
        update(&mut map);
        if in_sync(&map) {
            println!("Second answer: {}", i + 1);
            break;
        }
    }
}

fn load_map(path: &str) -> Map {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect()
        })
        .collect()
}

#[test]
fn test() {
    let mut map = load_map("inputs/day11_test.txt");

    let mut flashes = 0;
    for _ in 0..100 {
        flashes += update(&mut map);
    }

    assert_eq!(flashes, 1656);

    for i in 100.. {
        update(&mut map);
        if in_sync(&map) {
            assert_eq!(i + 1, 195);
            break;
        }
    }
}
