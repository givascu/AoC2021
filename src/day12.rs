use std::collections::{HashMap, HashSet};

type Graph<'a> = HashMap<&'a str, HashSet<&'a str>>;

fn count_paths_rec<'a>(
    graph: &'a Graph,
    src: &'a str,
    dst: &'a str,
    count: &mut i64,
    visited: &mut HashSet<&'a str>,
) {
    visited.insert(src);
    if src == dst {
        *count += 1;
    } else {
        graph.get(src).unwrap().iter().for_each(|&node| {
            if node.chars().all(char::is_uppercase) || !visited.contains(node) {
                count_paths_rec(graph, node, dst, count, visited);
            }
        });
    }
    visited.remove(src);
}

fn count_paths<'a>(graph: &Graph, src: &'a str, dst: &'a str) -> i64 {
    let mut visited = HashSet::<&'a str>::new();
    let mut count = 0i64;
    count_paths_rec(graph, src, dst, &mut count, &mut visited);
    count
}

pub fn solve_1() -> i64 {
    let mut graph = Graph::new();

    include_str!("../input/12.txt").lines().for_each(|line| {
        let (src, dst) = line.split_once('-').unwrap();
        graph.entry(src).or_default().insert(dst);
        graph.entry(dst).or_default().insert(src);
    });

    count_paths(&graph, "start", "end")
}
