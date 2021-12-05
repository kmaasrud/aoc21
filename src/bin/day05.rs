type Point = (usize, usize);
type Line = (Point, Point);

fn count_intersections(mut crossed_points: Vec<Point>) -> usize {
    crossed_points.sort_unstable();
    crossed_points
        .iter()
        .fold((0, (0, 0), (0, 0)), |(acc, prev, prevprev), next| {
            if *next == prev && prev != prevprev {
                (acc + 1, *next, prev)
            } else {
                (acc, *next, prev)
            }
        })
        .0
}

fn a(lines: &[Line]) -> usize {
    let mut crossed_points = Vec::<Point>::new();
    for line in lines {
        match line {
            ((x1, y1), (x2, y2)) if x1 == x2 => {
                let min = *y1.min(y2);
                let max = *y1.max(y2);
                crossed_points.extend((min..=max).map(|y| (*x1, y)));
            }
            ((x1, y1), (x2, y2)) if y1 == y2 => {
                let min = *x1.min(x2);
                let max = *x1.max(x2);
                crossed_points.extend((min..=max).map(|x| (x, *y1)));
            }
            _ => {}
        }
    }

    count_intersections(crossed_points)
}

fn b(lines: &[Line]) -> usize {
    let mut crossed_points = Vec::<Point>::new();
    for line in lines {
        match line {
            ((x1, y1), (x2, y2)) if x1 == x2 => {
                let min = *y1.min(y2);
                let max = *y1.max(y2);
                crossed_points.extend((min..=max).map(|y| (*x1, y)));
            }
            ((x1, y1), (x2, y2)) if y1 == y2 => {
                let min = *x1.min(x2);
                let max = *x1.max(x2);
                crossed_points.extend((min..=max).map(|x| (x, *y1)));
            }
            ((x1, y1), (x2, y2)) => {
                let xrange = if x1 < x2 {
                    Box::new(*x1..=*x2) as Box<dyn Iterator<Item = usize>>
                } else {
                    Box::new((*x2..=*x1).rev())
                };
                let yrange = if y1 < y2 {
                    Box::new(*y1..=*y2) as Box<dyn Iterator<Item = usize>>
                } else {
                    Box::new((*y2..=*y1).rev())
                };
                crossed_points.extend(xrange.zip(yrange))
            }
        }
    }

    count_intersections(crossed_points)
}

fn main() {
    let lines = load_lines("inputs/day05.txt");

    println!("First answer: {}", a(&lines));
    println!("First answer: {}", b(&lines));
}

fn load_lines(path: &str) -> Vec<Line> {
    let make_point = |s: &str| -> Point {
        let (x, y) = s.split_once(",").unwrap();
        (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
    };
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .filter_map(|line| line.split_once(" -> "))
        .map(|(a, b)| (make_point(a), make_point(b)))
        .collect()
}

#[test]
fn test() {
    let lines = load_lines("inputs/day05_test.txt");

    assert_eq!(a(&lines), 5);
    assert_eq!(b(&lines), 12);
}
