use std::collections::{HashMap, HashSet};

type Graph<'a> = HashMap<&'a str, HashSet<&'a str>>;

fn count_paths(graph: &Graph, src: &str, can_double_pass: bool, seen: &HashSet<&str>) -> i64 {
    if src == "end" {
        return 1;
    }

    let mut count = 0;

    graph
        .get(src)
        .unwrap()
        .iter()
        .filter(|&node| *node != "start")
        .for_each(|&node| {
            if node.chars().all(char::is_uppercase) || !seen.contains(node) {
                let mut seen_copy = seen.clone();
                seen_copy.insert(node);
                count += count_paths(graph, node, can_double_pass, &seen_copy);
            } else if can_double_pass {
                count += count_paths(graph, node, false, seen);
            }
        });

    count
}

pub fn solve_2() -> i64 {
    let mut graph = Graph::new();

    include_str!("../input/12.txt").lines().for_each(|line| {
        let (src, dst) = line.split_once('-').unwrap();
        graph.entry(src).or_default().insert(dst);
        graph.entry(dst).or_default().insert(src);
    });

    let mut seen = HashSet::new();
    seen.insert("start");
    count_paths(&graph, "start", true, &seen)
}

pub fn solve_1() -> i64 {
    let mut graph = Graph::new();

    include_str!("../input/12.txt").lines().for_each(|line| {
        let (src, dst) = line.split_once('-').unwrap();
        graph.entry(src).or_default().insert(dst);
        graph.entry(dst).or_default().insert(src);
    });

    let mut seen = HashSet::new();
    seen.insert("start");
    count_paths(&graph, "start", false, &seen)
}
