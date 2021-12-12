use std::collections::{HashMap, HashSet};

type Graph = HashMap<String, HashSet<String>>;

fn is_small(id: &str) -> bool {
    id.chars().all(char::is_lowercase)
}

fn make_graph(links: &[(String, String)]) -> Graph {
    let mut graph = Graph::default();
    for (from, to) in links {
        let mut add_link = |start: &str, end: &str| {
            graph
                .entry(start.to_owned())
                .or_default()
                .insert(end.to_owned()); 
        };
        add_link(from, to);
        add_link(to, from);
    }
    graph
}

fn dfs(cur: String, path: &mut Vec<String>, graph: &Graph) -> usize {
    if cur == "end" {
        1
    } else {
        path.push(cur.clone());
        graph[&cur]
            .iter()
            .filter(|node| !(path.contains(node) && is_small(node)))
            .map(|new_node| dfs(new_node.to_owned(), &mut path.clone(), graph))
            .sum()
    }
}

fn a(graph: &Graph) -> usize {
    dfs("start".to_owned(), &mut Vec::new(), graph)
}

fn main() {
    let links = load_links("inputs/day12.txt");
    let graph = make_graph(&links);

    println!("First answer: {}", a(&graph));
}

fn load_links(path: &str) -> Vec<(String, String)> {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .filter_map(|line| line.split_once('-'))
        .map(|(from, to)| (from.to_owned(), to.to_owned()))
        .collect()
}

#[test]
fn test1() {
    let links = load_links("inputs/day12_test1.txt");
    let graph = make_graph(&links);
    assert_eq!(a(&graph), 10);
}

#[test]
fn test2() {
    let links = load_links("inputs/day12_test2.txt");
    let graph = make_graph(&links);
    assert_eq!(a(&graph), 19);
}

#[test]
fn test3() {
    let links = load_links("inputs/day12_test3.txt");
    let graph = make_graph(&links);
    assert_eq!(a(&graph), 226);
}
