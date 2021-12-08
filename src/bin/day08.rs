use std::collections::{HashMap, HashSet};

fn a(entries: &[(Vec<String>, Vec<String>)]) -> usize {
    entries
        .iter()
        .map(|(_, output)| output.iter().map(|val| val.chars().count()))
        .flatten()
        .filter(|len| matches!(len, 2 | 3 | 4 | 7))
        .count()
}

fn b(entries: &[(Vec<String>, Vec<String>)]) -> usize {
    fn char_intersection_len(a: &str, b: &str) -> usize {
        a.chars()
            .collect::<HashSet<char>>()
            .intersection(&b.chars().collect())
            .count()
    }

    fn sort_string(s: &str) -> String {
        let mut chars = s.chars().collect::<Vec<char>>();
        chars.sort();
        String::from_iter(chars)
    }

    let mut sum = 0;
    for entry in entries {
        let (patterns, output) = entry;
        let mut cipher: HashMap<usize, String> = HashMap::new();

        let mut cycle = patterns.iter().cycle();
        loop {
            let pattern = sort_string(cycle.next().unwrap());
            match pattern.len() {
                2 => {
                    cipher.insert(1, pattern.to_owned());
                }
                3 => {
                    cipher.insert(7, pattern.to_owned());
                }
                4 => {
                    cipher.insert(4, pattern.to_owned());
                }
                7 => {
                    cipher.insert(8, pattern.to_owned());
                }
                6 => {
                    if let Some(one) = cipher.get(&1) {
                        if char_intersection_len(one, &pattern) == 1 {
                            cipher.insert(6, pattern.to_owned());
                        }
                    }
                    if let Some(four) = cipher.get(&4) {
                        if char_intersection_len(four, &pattern) == 4 {
                            cipher.insert(9, pattern.to_owned());
                        }
                    }
                    if let Some(five) = cipher.get(&5) {
                        if char_intersection_len(five, &pattern) == 4 {
                            cipher.insert(0, pattern.to_owned());
                        }
                    }
                }
                5 => {
                    if let Some(one) = cipher.get(&1) {
                        if char_intersection_len(one, &pattern) == 2 {
                            cipher.insert(3, pattern.to_owned());
                        }
                    }
                    if let (Some(six), Some(one)) = (cipher.get(&6), cipher.get(&1)) {
                        match (
                            char_intersection_len(six, &pattern),
                            char_intersection_len(one, &pattern),
                        ) {
                            (4, 1) => {
                                cipher.insert(2, pattern.to_owned());
                            }
                            (5, 1) => {
                                cipher.insert(5, pattern.to_owned());
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }

            if cipher.len() == 10 {
                break;
            }
        }

        sum += output
            .iter()
            .filter_map(|signal| {
                cipher.iter().find_map(|(key, val)| {
                    if *val == sort_string(signal) {
                        Some(key.to_string())
                    } else {
                        None
                    }
                })
            })
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
    }
    sum
}

fn main() {
    let entries = load_lines("inputs/day08.txt");

    println!("First answer: {}", a(&entries));
    println!("Second answer: {}", b(&entries));
}

fn load_lines(path: &str) -> Vec<(Vec<String>, Vec<String>)> {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| {
            let (pattern, output) = line.split_once('|').unwrap();
            (
                pattern.trim().split(" ").map(String::from).collect(),
                output.trim().split(" ").map(String::from).collect(),
            )
        })
        .collect()
}

#[test]
fn test() {
    let entries = load_lines("inputs/day08_test.txt");

    assert_eq!(a(&entries), 26);
    assert_eq!(b(&entries), 61229);
}
