use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub struct Mapping {
    pub dest_start: usize,
    pub source_start: usize,
    pub len: usize,
}

impl Mapping {
    pub fn try_map(&self, source: usize) -> Option<usize> {
        let offset = source.checked_sub(self.source_start)?;
        if offset < self.len {
            return Some(self.dest_start + offset);
        }
        None
    }
}

impl FromStr for Mapping {
    type Err = anyhow::Error;

    // Expects 3 space separated numbers
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use anyhow::anyhow;
        let mut nums = s.split_whitespace();
        let dest_start = nums
            .next()
            .ok_or(anyhow!("should have a first number"))?
            .parse()?;
        let source_start = nums
            .next()
            .ok_or(anyhow!("should have a second number"))?
            .parse()?;
        let len = nums
            .next()
            .ok_or(anyhow!("should have a third number"))?
            .parse()?;
        let None = nums.next() else {
            return Err(anyhow!("should only have 3 numbers"));
        };
        Ok(Self {
            dest_start,
            source_start,
            len,
        })
    }
}

#[derive(Debug)]
pub struct Map(pub Vec<Mapping>);

impl FromStr for Map {
    type Err = anyhow::Error;
    // expects lines of 3 numbers, space separated
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mappings: Vec<Mapping> = s.lines().map(Mapping::from_str).collect::<Result<_, _>>()?;
        Ok(Self(mappings))
    }
}

impl Map {
    pub fn map(&self, source: &mut usize) -> usize {
        if let Some(dest) = self.0.iter().find_map(|x| x.try_map(*source)) {
            *source = dest;
        }
        *source
    }
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut input_sections = input.split("\n\n");
    let mut curr_sources: Vec<usize> = input_sections
        .next()
        .expect("should have initial seeds section")[7..] // skipping "seeds: "
        .split_whitespace()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .expect("should be valid seeds");
    input_sections
        .map(|x| &x[x.find(':').expect("map label should have colon") + 2/*2 chars of whitespace*/..])
        .map(Map::from_str)
        .map(|x| x.expect("should be valid map"))
        .map(|map| curr_sources.iter_mut().map(|x| map.map(x)).min())
        .last()
        .expect("shouldnt be empty")
        .expect("shouldnt be empty")
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut input_sections = input.split("\n\n");
    let mut curr_sources: Vec<usize> = input_sections
        .next()
        .expect("should have initial seeds section")[7..] // skipping "seeds: "
        .split_whitespace()
        .collect::<Vec<_>>()
        .chunks_exact(2)
        .flat_map(|range_str| {
            let [start_str, len_str] = range_str else {
                panic!("should be two nums")
            };
            let (x, len) = (
                start_str.parse::<usize>().expect("should be valid num"),
                len_str.parse::<usize>().expect("should be valid num"),
            );
            x..x + len
        })
        .collect();
    input_sections
        .map(|x| &x[x.find(':').expect("map label should have colon") + 2/*2 chars of whitespace*/..])
        .map(Map::from_str)
        .map(|x| x.expect("should be valid map"))
        .map(|map| {
            curr_sources
                .iter_mut()
                .map(|x| map.map(x))
                .min()
        })
        .last()
        .expect("shouldnt be empty")
        .expect("shouldnt be empty")
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(INPUT), 35);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(INPUT), 46);
    }
}
