fn matching(c: &char) -> char {
    match c {
        '(' => ')',
        ')' => '(',
        '[' => ']',
        ']' => '[',
        '{' => '}',
        '}' => '{',
        '<' => '>',
        '>' => '<',
        _ => *c,
    }
}

fn a(lines: &[String]) -> (usize, Vec<String>) {
    let val = |c: char| -> usize {
        match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0,
        }
    };

    let mut sum = 0;
    let mut non_corrupt = Vec::new();
    for line in lines {
        let mut stack = Vec::new();
        non_corrupt.push(line.to_owned());
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                _ => {
                    if stack.pop() != Some(matching(&c)) {
                        sum += val(c);
                        non_corrupt.pop();
                        break;
                    }
                }
            }
        }
    }
    (sum, non_corrupt)
}

fn b(lines: &[String]) -> usize {
    let val = |c: char| -> usize {
        match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => 0,
        }
    };

    let mut completion_vals = Vec::new();
    for line in lines {
        let mut stack = Vec::new();
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                _ => {
                    if stack.pop() != Some(matching(&c)) {
                        stack.push(c);
                    }
                }
            }
        }

        completion_vals.push(
            stack
                .iter()
                .rev()
                .map(matching)
                .map(val)
                .fold(0, |acc, x| 5 * acc + x),
        );
    }
    completion_vals.sort_unstable();
    completion_vals[completion_vals.len() / 2]
}

fn main() {
    let lines = load_lines("inputs/day10.txt");

    let (answer, non_corrupt) = a(&lines);
    println!("First answer: {}", answer);

    println!("Second answer: {}", b(&non_corrupt));
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

    let (answer, non_corrupt) = a(&lines);
    assert_eq!(answer, 26397);

    assert_eq!(b(&non_corrupt), 288957);
}
