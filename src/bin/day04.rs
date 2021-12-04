fn a(draws: Vec<usize>, boards: Vec<Vec<usize>>) -> usize {
    let mut shortest = usize::MAX;
    let mut shortest_board = Vec::<usize>::new();

    for board in boards {
        for i in 0..5 {
            let mut coli = col(&board, i);
            let mut rowi = row(&board, i);
            for (i, draw) in draws.iter().enumerate() {
                coli.retain(|x| x != &draw);
                rowi.retain(|x| x != &draw);
                match (coli.is_empty(), rowi.is_empty()) {
                    _ if i > shortest => break,
                    (true, _) | (_, true) if i < shortest => {
                        shortest = i;
                        shortest_board = board.clone();
                    }
                    _ => {},
                }
            }
        }
    }

    let score: usize = shortest_board
        .iter()
        .filter(|x| draws[..shortest+1].iter().all(|draw| x != &draw))
        .sum();

    score * draws[shortest]
}

fn col(board: &Vec<usize>, i: usize) -> Vec<&usize> {
    board.iter().skip(i).step_by(5).take(5).collect()
}

fn row(board: &Vec<usize>, i: usize) -> Vec<&usize> {
    board.iter().skip(5 * i).take(5).collect()
}

fn main() {
    let (draws, boards) = load_boards("inputs/day04.txt");
    
    println!("First answer: {}", a(draws, boards));
}

fn load_boards(path: &str) -> (Vec<usize>, Vec<Vec<usize>>) {
    let input = std::fs::read_to_string(path).unwrap();
    let mut input_lines = input.lines();

    let draws = input_lines
        .next()
        .unwrap()
        .split(",")
        .filter_map(|draw| draw.parse::<usize>().ok())
        .collect();

    let boards = input_lines
        .collect::<Vec<&str>>()
        .chunks(6)
        .map(|chunk| {
            chunk.iter().skip(1).map(|line| -> Vec<usize> {
                line.split_ascii_whitespace()
                    .filter_map(|num| num.parse::<usize>().ok())
                    .collect()
            })
            .flatten()
            .collect()
        })
        .collect();

    (draws, boards)
}

#[test]
fn test() {
    let (draws, boards) = load_boards("inputs/day04_test.txt");

    assert_eq!(a(draws, boards), 4512);
}
