fn a(lines: &[String]) -> usize {
    let mut sum = 0;
    for line in lines {
        let mut buf = Vec::new();
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => buf.push(c),
                ')' => {
                    if buf.pop() != Some('(') {
                        sum += 3;
                        break
                    }
                }
                ']' => {
                    if buf.pop() != Some('[') {
                        sum += 57;
                        break
                    }
                }
                '}' => {
                    if buf.pop() != Some('{') {
                        sum += 1197;
                        break
                    }
                }
                '>' => {
                    if buf.pop() != Some('<') {
                        sum += 25137;
                        break
                    }
                }
                _ => println!("Invalid char: {}", c),
            }
        }
    }
    sum
}

fn main() {
    let lines = load_lines("inputs/day10.txt");

    println!("First answer: {}", a(&lines));
}

fn load_lines(path: &str) -> Vec<String> {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

#[test]
fn test() {
    let lines = load_lines("inputs/day10_test.txt");

    assert_eq!(a(&lines), 26397);
}
