use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn solve(input: &str) -> usize {
    // +---+---+---+
    // | 7 | 8 | 9 |
    // +---+---+---+
    // | 4 | 5 | 6 |
    // +---+---+---+
    // | 1 | 2 | 3 |
    // +---+---+---+
    //     | 0 | A |
    //     +---+---+
    let graph0 = Graph::from_edges(&[
        ('0', '>', 'A'),
        ('0', '^', '2'),
        ('A', '^', '3'),
        ('1', '>', '2'),
        ('1', '^', '4'),
        ('2', '>', '3'),
        ('2', '^', '5'),
        ('3', '^', '6'),
        ('4', '>', '5'),
        ('4', '^', '7'),
        ('5', '>', '6'),
        ('5', '^', '8'),
        ('6', '^', '9'),
        ('7', '>', '8'),
        ('8', '>', '9'),
    ]);

    //     +---+---+
    //     | ^ | A |
    // +---+---+---+
    // | < | v | > |
    // +---+---+---+
    let graph1 = Graph::from_edges(&[
        ('<', '>', 'v'),
        ('v', '>', '>'),
        ('v', '^', '^'),
        ('>', '^', 'A'),
        ('^', '>', 'A'),
    ]);

    let mut result = 0;
    for line in input.lines() {
        let num: usize = line[..3].parse().unwrap();
        let stops = "A".to_owned() + line;
        for (from, to) in stops.chars().tuple_windows() {
            result += path_len(&graph0, &graph1, from, to) * num;
        }
    }
    result
}

fn path_len(graph0: &Graph, graph1: &Graph, start_0: char, end_0: char) -> usize {
    let start = (start_0, 'A', 'A');
    let end = (end_0, 'A', 'A');
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    while let Some((pos, len)) = queue.pop_front() {
        if pos == end {
            return len + 1;
        }
        if !visited.insert(pos) {
            continue;
        }
        let (x, y, z) = pos;
        let new_len = len + 1;
        for &dir in &['<', '>', '^', 'v'] {
            if let Some(&next_z) = graph1.edges.get(&(z, dir)) {
                queue.push_back(((x, y, next_z), new_len));
            }
        }
        if z != 'A' {
            if let Some(&next_y) = graph1.edges.get(&(y, z)) {
                queue.push_back(((x, next_y, z), new_len));
            }
        } else {
            if y != 'A' {
                if let Some(&next_x) = graph0.edges.get(&(x, y)) {
                    queue.push_back(((next_x, y, z), new_len));
                }
            }
        }
    }
    panic!("no path found")
}

struct Graph {
    edges: HashMap<(char, char), char>,
}

impl Graph {
    fn from_edges(edges: &[(char, char, char)]) -> Self {
        let mut graph = Graph {
            edges: HashMap::new(),
        };
        for &(from, dir, to) in edges {
            graph.edges.insert((from, dir), to);
            graph.edges.insert((to, opposite(dir)), from);
        }
        graph
    }
}

fn opposite(dir: char) -> char {
    match dir {
        '<' => '>',
        '>' => '<',
        '^' => 'v',
        'v' => '^',
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_solve() {
        let input = indoc! {"
            029A
            980A
            179A
            456A
            379A
        "};
        assert_eq!(solve(input), 126384);
    }
}
