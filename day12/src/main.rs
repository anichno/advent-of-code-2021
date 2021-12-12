use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum CaveNode<'a> {
    Big(&'a str),
    Small(&'a str),
    Start,
    End,
}

fn parse_input<'a>(input: &'a [&'a str]) -> HashMap<CaveNode<'a>, HashSet<CaveNode<'a>>> {
    let mut map = HashMap::new();
    for line in input {
        let (left, right) = line.trim().split_once('-').unwrap();
        let left_node = match left {
            "start" => CaveNode::Start,
            "end" => CaveNode::End,
            _ => {
                if left.chars().next().unwrap().is_uppercase() {
                    CaveNode::Big(left)
                } else {
                    CaveNode::Small(left)
                }
            }
        };

        let right_node = match right {
            "start" => CaveNode::Start,
            "end" => CaveNode::End,
            _ => {
                if right.chars().next().unwrap().is_uppercase() {
                    CaveNode::Big(right)
                } else {
                    CaveNode::Small(right)
                }
            }
        };

        let mapping = map.entry(left_node.clone()).or_insert_with(HashSet::new);
        mapping.insert(right_node.clone());

        let mapping = map.entry(right_node.clone()).or_insert_with(HashSet::new);
        mapping.insert(left_node);
    }

    map
}

fn visit_nodes1<'a, 'b>(
    cur_node: &'a CaveNode,
    visited: &'b mut HashSet<&'a CaveNode<'a>>,
    map: &'a HashMap<CaveNode, HashSet<CaveNode<'a>>>,
) -> u64 {
    let mut paths_found = 0;

    if *cur_node == CaveNode::End {
        return 1;
    }

    // for next_node in map.get(&cur_node) {
    if let Some(next_node) = map.get(cur_node) {
        for next_node in next_node.iter() {
            match next_node {
                CaveNode::Start => (),
                CaveNode::End | CaveNode::Big(_) => {
                    paths_found += visit_nodes1(next_node, visited, map);
                }
                CaveNode::Small(_) => {
                    if visited.insert(next_node) {
                        paths_found += visit_nodes1(next_node, visited, map);
                        visited.remove(next_node);
                    }
                }
            }
        }
    }

    paths_found
}

fn solve1(input: &HashMap<CaveNode, HashSet<CaveNode>>) -> u64 {
    let mut visited = HashSet::new();
    visited.insert(&CaveNode::Start);

    visit_nodes1(&CaveNode::Start, &mut visited, input)
}

fn visit_nodes2<'a, 'b>(
    cur_node: &'a CaveNode,
    visited: &'b mut HashMap<&'a CaveNode<'a>, u8>,
    map: &'a HashMap<CaveNode, HashSet<CaveNode<'a>>>,
    double_small_visit: bool,
) -> u64 {
    let mut paths_found = 0;

    if *cur_node == CaveNode::End {
        return 1;
    }

    // for next_node in map.get(&cur_node) {
    if let Some(next_node) = map.get(cur_node) {
        for next_node in next_node.iter() {
            match next_node {
                CaveNode::Start => (),
                CaveNode::End | CaveNode::Big(_) => {
                    paths_found += visit_nodes2(next_node, visited, map, double_small_visit);
                }
                CaveNode::Small(_) => {
                    let tracker = visited.entry(next_node).or_insert(0);
                    if *tracker == 0 {
                        *tracker += 1;
                        paths_found += visit_nodes2(next_node, visited, map, double_small_visit);
                        *visited.get_mut(next_node).unwrap() -= 1;
                    } else if *tracker == 1 && !double_small_visit {
                        *tracker += 1;
                        paths_found += visit_nodes2(next_node, visited, map, true);
                        *visited.get_mut(next_node).unwrap() -= 1;
                    }
                }
            }
        }
    }

    paths_found
}

fn solve2(input: &HashMap<CaveNode, HashSet<CaveNode>>) -> u64 {
    let mut visited: HashMap<&CaveNode, u8> = HashMap::new();

    visit_nodes2(&CaveNode::Start, &mut visited, input, false)
}

fn main() {
    let input: Vec<&str> = include_str!("input.txt").trim().lines().collect();
    let parsed = parse_input(&input);

    println!("part 1: {}", solve1(&parsed));
    println!("part 2: {}", solve2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[&str] = &["start-A", "start-b", "A-c", "A-b", "b-d", "A-end", "b-end"];

    #[test]
    fn test_parse() {
        let parsed = parse_input(&INPUT);
        assert!(parsed
            .get(&CaveNode::Start)
            .unwrap()
            .contains(&CaveNode::Big("A")));

        assert!(parsed
            .get(&CaveNode::Small("b"))
            .unwrap()
            .contains(&CaveNode::Big("A")));
    }

    #[test]
    fn test_solve1() {
        let parsed = parse_input(&INPUT);
        assert_eq!(solve1(&parsed), 10);
    }

    #[test]
    fn test_solve2() {
        let parsed = parse_input(&INPUT);
        assert_eq!(solve2(&parsed), 36);
    }
}
