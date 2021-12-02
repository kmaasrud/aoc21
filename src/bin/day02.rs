fn a(data: Vec<(String, usize)>) -> usize {
    let mut depth = 0;
    let mut position = 0;
    
    for (direction, add) in data.iter() {
        match direction.as_str() {
            "forward" => position += add,
            "up" => depth -= add,
            "down" => depth += add,
            _ => {}
        }
    }

    depth * position
}

fn b(data: Vec<(String, usize)>) -> usize {
    let mut aim = 0;
    let mut depth = 0;
    let mut position = 0;
    
    for (direction, add) in data.iter() {
        match direction.as_str() {
            "forward" => {
                position += add;
                depth += add * aim;
            }
            "up" => aim -= add,
            "down" => aim += add,
            _ => {}
        }
    }

    depth * position
}

fn main () {
    let data = load_lines("inputs/day02.txt");

    println!("First answer: {}", a(data.clone()));
    println!("Second answer: {}", b(data.clone()));
}

fn load_lines(path: &str) -> Vec<(String, usize)> {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .filter_map(|line| line.split_once(" "))
        .map(|(a, b)| (a.to_owned(), b.parse::<usize>().unwrap()))
        .collect()
}

#[test]
fn test() {
    let data = load_lines("inputs/day02_test.txt");
    assert_eq!(a(data.clone()), 150);
    assert_eq!(b(data), 900);
}
