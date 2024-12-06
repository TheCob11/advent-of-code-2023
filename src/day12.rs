use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
enum Spring {
    Working = b'.',
    Broken = b'#',
}

trait SpringCondition {
    fn is_working(&self) -> bool;
    fn is_broken(&self) -> bool;
    fn is_not_working(&self) -> bool {
        !self.is_working()
    }
    fn is_not_broken(&self) -> bool {
        !self.is_broken()
    }
}

impl SpringCondition for Spring {
    fn is_working(&self) -> bool {
        matches!(self, Spring::Working)
    }
    fn is_broken(&self) -> bool {
        matches!(self, Spring::Broken)
    }
}

impl SpringCondition for Option<Spring> {
    fn is_working(&self) -> bool {
        matches!(self, Some(Spring::Working))
    }
    fn is_broken(&self) -> bool {
        matches!(self, Some(Spring::Broken))
    }
}

impl From<Spring> for bool {
    fn from(value: Spring) -> Self {
        value == Spring::Working
    }
}

impl std::ops::Not for Spring {
    type Output = Self;
    fn not(self) -> Self::Output {
        use Spring as S;
        match self {
            S::Working => S::Broken,
            S::Broken => S::Working,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Record {
    working_springs: Vec<Option<Spring>>,
    group_sizes: Vec<usize>,
}

impl Record {
    fn is_valid(broken_springs: &[bool], group_sizes: impl IntoIterator<Item = usize>) -> bool {
        broken_springs
            .split(|x| *x)
            .filter(|x| !x.is_empty())
            // .inspect(|x| println!("{x:?}"))
            .map(<[_]>::len)
            .eq(group_sizes)
    }

    fn brute_force_operational_possibilities_from_parts(
        springs: &[Option<Spring>],
        group_sizes: &[usize],
    ) -> usize {
        match springs.iter().position(Option::is_none) {
            None => {
                return Self::is_valid(
                    springs
                        .iter()
                        .map(|x| {
                            x.expect("shouldn't have nones bc they weren't found")
                                .is_working()
                        })
                        .collect::<Vec<_>>()
                        .as_slice(),
                    group_sizes.to_owned(),
                )
                .into()
            }
            Some(curr) => {
                let mut springs = springs.to_vec();
                *springs.get_mut(curr).expect("curr should be in springs") = Some(Spring::Working);
                // println!("{springs:?}");
                let working_count = Self::brute_force_operational_possibilities_from_parts(
                    springs.as_slice(),
                    group_sizes,
                );
                *springs.get_mut(curr).expect("curr should be in springs") = Some(Spring::Broken);
                // println!("{springs:?}");
                let broken_count = Self::brute_force_operational_possibilities_from_parts(
                    springs.as_slice(),
                    group_sizes,
                );
                working_count + broken_count
            }
        }
    }

    fn brute_force_operational_possibilities(&self) -> usize {
        Self::brute_force_operational_possibilities_from_parts(
            self.working_springs.as_slice(),
            self.group_sizes.as_slice(),
        )
    }

    fn operational_possibilities(self) -> usize {
        match self.group_sizes.as_slice() {
            [_] => (0..self.working_springs.len())
                .filter_map(|i| self.trim_first_group_from(i))
                .filter(|x| !x.is_some_and(|post| post.contains(&Some(Spring::Broken))))
                .count(),
            [size, ref remaining_sizes @ ..] => (0..self
                .working_springs
                .len()
                .saturating_sub(size - 1 + remaining_sizes.iter().map(|x| x + 1).sum::<usize>()))
                .filter_map(|i| self.trim_first_group_from(i).flatten())
                .map(|x| {
                    Self {
                        working_springs: x.into(),
                        group_sizes: remaining_sizes.into(),
                    }
                    .operational_possibilities()
                })
                .sum(),
            _ => 0,
        }
    }

    fn trim_first_group_from(&self, i: usize) -> Option<Option<&[Option<Spring>]>> {
        let end = i + self.group_sizes[0];
        let Some((before, group)) = self.working_springs.get(..end).map(|x| x.split_at(i)) else {
            return None;
        };
        (!(before.contains(&Some(Spring::Broken))
            || group.contains(&Some(Spring::Working))
            || self
                .working_springs
                .get(end)
                .is_some_and(SpringCondition::is_broken)))
        .then_some(self.working_springs.get(end + 1..))
    }

    fn operational_possibilities_caching(self, cache: &mut HashMap<Self, usize>) -> usize {
        if let Some(&count) = cache.get(&self) {
            count
        } else {
            let count = match self.group_sizes.as_slice() {
                [_] => (0..self.working_springs.len())
                    .filter_map(|i| self.trim_first_group_from(i))
                    .filter(|x| !x.is_some_and(|post| post.contains(&Some(Spring::Broken))))
                    .count(),
                [size, ref remaining_sizes @ ..] => (0..self.working_springs.len().saturating_sub(
                    size - 1 + remaining_sizes.iter().map(|x| x + 1).sum::<usize>(),
                ))
                    .filter_map(|i| self.trim_first_group_from(i).flatten())
                    .map(|springs| {
                        Self {
                            working_springs: springs
                                .iter()
                                .copied()
                                .skip_while(|&x| x.is_working())
                                .collect(),
                            group_sizes: remaining_sizes.into(),
                        }
                        .operational_possibilities_caching(cache)
                    })
                    .sum(),
                _ => 0,
            };
            cache.insert(self, count);
            count
        }
    }
}

impl FromStr for Record {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (spring_str, sizes_str) = s.split_once(' ').expect("should have the space");
        let working_springs = spring_str
            .as_bytes()
            .iter()
            .map(|x| {
                Ok(match x {
                    b'#' => Some(Spring::Broken),
                    b'.' => Some(Spring::Working),
                    b'?' => None,
                    _ => return Err(anyhow::anyhow!("chars should be '#', '.', or '?'")),
                })
            })
            .collect::<Result<_, _>>()?;
        let group_sizes = sizes_str
            .split(',')
            .map(usize::from_str)
            .collect::<Result<_, _>>()?;
        Ok(Self {
            working_springs,
            group_sizes,
        })
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use fmt::Write;
        use std::convert::identity;
        for x in &self.working_springs {
            f.write_char(match x {
                Some(x) => char::from(*x as u8),
                None => '?',
            })?;
        }
        f.write_char(' ')?;
        f.write_str(
            self.group_sizes
                .iter()
                .map(usize::to_string)
                .reduce(|a, ref b| a + "," + b)
                .map_or(String::new(), identity)
                .as_str(),
        )
    }
}

#[aoc(day12, part1)]
fn part1(input: &str) -> usize {
    input
        .lines()
        .map(Record::from_str)
        .map(|x| x.expect("should be valid record"))
        .map(|x| x.operational_possibilities())
        .sum()
}

#[aoc(day12, part1, caching)]
fn part1_caching(input: &str) -> usize {
    let mut cache = HashMap::new();
    input
        .lines()
        .map(Record::from_str)
        .map(|x| x.expect("should be valid record"))
        .map(|x| x.operational_possibilities_caching(&mut cache))
        .sum()
}

#[aoc(day12, part1, brute_force)]
fn part1_brute_force(input: &str) -> usize {
    input
        .lines()
        .map(Record::from_str)
        .map(|x| x.expect("should be valid record"))
        .map(|x| x.brute_force_operational_possibilities())
        .sum()
}

#[aoc(day12, part2)]
#[allow(unused)]
fn part2(input: &str) -> usize {
    let mut cache = HashMap::new();
    input
        .lines()
        .map(Record::from_str)
        .map(|x| x.expect("should be valid record"))
        .map(
            |Record {
                 working_springs: springs,
                 group_sizes: groups,
             }| {
                let mut working_springs = springs;
                working_springs.push(None);
                working_springs = working_springs.repeat(5);
                working_springs.pop();
                Record {
                    working_springs,
                    group_sizes: groups.repeat(5),
                }
            },
        )
        .map(|x| x.operational_possibilities_caching(&mut cache))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    const EXAMPLE: &str = "\
    ???.### 1,1,3\n\
    .??..??...?##. 1,1,3\n\
    ?#?#?#?#?#?#?#? 1,3,1,6\n\
    ????.#...#... 4,1,1\n\
    ????.######..#####. 1,6,5\n\
    ?###???????? 3,2,1";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 21);
    }

    #[test]
    #[ignore = "too long"]
    fn part1_answer1() {
        let sum = part1(&read_to_string("./input/2023/day12.txt").expect("should be there"));
        println!("{sum}");
        assert!(sum < 9029);
    }

    #[test]
    #[ignore = "too long"]
    fn part1_answer2() {
        let sum = part1(&read_to_string("./input/2023/day12.txt").expect("should be there"));
        println!("{sum}");
        assert!(sum < 7562);
    }

    #[test]
    #[ignore = "too long"]
    fn part1_correct() {
        let sum = part1(&read_to_string("./input/2023/day12.txt").expect("should be there"));
        assert_eq!(sum, 7236);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 525_152);
    }
}
