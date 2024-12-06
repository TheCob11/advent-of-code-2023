use std::collections::HashMap;
use std::fmt::{Display, Formatter, Write};
use std::ops::{ControlFlow, Index};
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
}

impl From<Direction> for char {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Left => 'L',
            Direction::Right => 'R',
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char((*self).into())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Node([u8; 3]);

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.0[0] as char, self.0[1] as char, self.0[2] as char
        )
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Fork {
    left: Node,
    right: Node,
}

impl Index<Direction> for Fork {
    type Output = Node;
    fn index(&self, index: Direction) -> &Self::Output {
        match index {
            Direction::Left => &self.left,
            Direction::Right => &self.right,
        }
    }
}

impl Display for Fork {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.left, self.right)
    }
}

const START: Node = Node(*b"AAA");
const FINISH: Node = Node(*b"ZZZ");

#[aoc(day8, part1)]
fn part1(input: &str) -> usize {
    let (dirs_str, nodes_str) = input.split_once("\n\n").expect("should have newline");
    let nodes: HashMap<Node, Fork> = nodes_str
        .lines()
        .map(|x| {
            let [k0, k1, k2, b' ', b'=', b' ', b'(', v00, v01, v02, b',', b' ', v10,v11,v12, b')'] = *x.as_bytes() else {
                panic!("should've matched node pattern")
            };
            (Node([k0, k1, k2]), Fork{left: Node([v00, v01, v02]), right: Node([v10, v11, v12])})
        })
        .collect();
    dirs_str
        .chars()
        .map(Direction::try_from)
        .map(|x| x.expect("should be valid direction"))
        .cycle()
        .scan(nodes[&START], |curr, dir| match curr[dir] {
            FINISH => None,
            ref x => {
                *curr = nodes[x];
                Some(())
            }
        })
        .count()
        + 1
}

fn populate_find<'a, K: Eq + std::hash::Hash, V>(
    search_key: &K,
    iter: &mut impl Iterator<Item = (K, V)>,
    cache: &'a mut HashMap<K, V>,
) -> Option<&'a V> {
    match iter.try_for_each(|(key, val)| {
        let found = key.eq(search_key);
        cache.insert(key, val);
        if found {
            ControlFlow::Break(())
        } else {
            ControlFlow::Continue(())
        }
    }) {
        ControlFlow::Continue(()) => None,
        ControlFlow::Break(()) => Some(
            cache
                .get(search_key)
                .expect("should've made sure it was in"),
        ),
    }
}

#[aoc(day8, part1, caching)]
fn part1_caching(input: &str) -> usize {
    let (dirs_str, nodes_str) = input.split_once("\n\n").expect("should have newline");
    let mut node_parser = nodes_str.lines().map(|x| {
        let [k0, k1, k2, b' ', b'=', b' ', b'(', v00, v01, v02, b',', b' ', v10, v11, v12, b')'] =
            *x.as_bytes()
        else {
            panic!("should've matched node pattern")
        };
        (
            Node([k0, k1, k2]),
            Fork {
                left: Node([v00, v01, v02]),
                right: Node([v10, v11, v12]),
            },
        )
    });
    let mut nodes = HashMap::new();
    let start_fork =
        *populate_find(&START, &mut node_parser, &mut nodes).expect("should have start");
    dirs_str
        .chars()
        .map(Direction::try_from)
        .map(|x| x.expect("should be valid direction"))
        .cycle()
        .scan(start_fork, |curr, dir| match curr[dir] {
            FINISH => None,
            ref next => {
                *curr = match nodes.get(next) {
                    Some(&x) => x,
                    None => *populate_find(next, &mut node_parser, &mut nodes)
                        .expect("should have next"),
                };
                Some(())
            }
        })
        .count()
        + 1
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * (b / gcd(a, b))
}

#[aoc(day8, part2)]
fn part2(input: &str) -> usize {
    let (dirs_str, nodes_str) = input.split_once("\n\n").expect("should have newline");
    let nodes: HashMap<Node, Fork> = nodes_str
        .lines()
        .map(|x| {
            let [k0, k1, k2, b' ', b'=', b' ', b'(', v00, v01, v02, b',', b' ', v10,v11,v12, b')'] = *x.as_bytes() else {
                panic!("should've matched node pattern")
            };
            (Node([k0, k1, k2]), Fork{left: Node([v00, v01, v02]), right: Node([v10, v11, v12])})
        })
        .collect();
    let mut forks: Vec<Fork> = nodes
        .iter()
        .filter(|(&Node([_, _, x]), _)| x == b'A')
        // .inspect(|x| println!("{}: {}", x.0, x.1))
        .map(|(_, &x)| x)
        .collect();
    let mut full_count = 1usize;
    for (i, dir) in dirs_str
        .chars()
        .map(Direction::try_from)
        .map(|x| x.expect("should be valid direction"))
        .cycle()
        .enumerate()
    {
        let mut any_finished: bool = false;
        forks = forks
            .iter()
            .filter_map(|fork| match fork[dir] {
                Node([_, _, b'Z']) => {
                    any_finished = true;
                    None
                }
                ref x => Some(nodes[x]),
            })
            .collect();
        if any_finished {
            full_count = lcm(full_count, i + 1);
            if forks.is_empty() {
                break;
            }
        }
    }
    full_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const INPUT_2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT_1), 6);
    }

    #[test]
    fn test_part1_caching() {
        assert_eq!(part1_caching(INPUT_1), 6);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT_2), 6);
    }
}
