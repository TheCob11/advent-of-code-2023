use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use anyhow::anyhow;

#[derive(Debug)]
struct Card {
    pub scratch_nums: HashSet<usize>,
    pub win_nums: HashSet<usize>,
}

impl FromStr for Card {
    type Err = anyhow::Error;
    // Expects like "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (scratches, winners) = s[s.find(':').expect("input should have colon") + 2..]
            .split_once(" | ")
            .ok_or(anyhow!(
                "Input should have a pipe delimiting scratches and winners"
            ))?;
        let scratch_nums = scratches
            .split_whitespace()
            .map(|x| x.trim().parse())
            .collect::<Result<_, _>>()?;
        let win_nums = winners
            .split_whitespace()
            .map(|x| x.trim().parse())
            .collect::<Result<_, _>>()?;
        Ok(Self {
            scratch_nums,
            win_nums,
        })
    }
}

impl Card {
    pub fn win_count(&self) -> usize {
        self.scratch_nums.intersection(&self.win_nums).count()
    }

    pub fn points(&self) -> usize {
        match self.win_count() {
            0 => 0,
            x => 1 << (x - 1),
        }
    }

    pub fn calculate_copies(
        &self,
        id: usize,
        other_card_counts: &mut HashMap<usize, usize>,
    ) -> usize {
        let copy_count = (id + 1..=id + self.win_count())
            .map(|x| 1 + other_card_counts.get(&x).expect("shouldnt be unknown"))
            .sum::<usize>();
        other_card_counts.insert(id, copy_count);
        copy_count
    }
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .map(Card::from_str)
        .map(Result::unwrap)
        .map(|x| x.points())
        .sum()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &str) -> usize {
    let total_card_count = input.matches('\n').count() + 1;
    let mut cached_card_counts = HashMap::with_capacity(total_card_count);
    total_card_count
        + input
            .lines()
            .rev()
            .enumerate()
            .map(|(i, x)| {
                (
                    total_card_count - i,
                    x.parse::<Card>().expect("should be valid card"),
                )
            })
            .map(|(i, x)| x.calculate_copies(i, &mut cached_card_counts))
            .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(solve_part2(INPUT), 30);
    }
}
