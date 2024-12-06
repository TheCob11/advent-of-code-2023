// #![allow(unused_variables, unused_imports, dead_code)]

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
struct Coords(usize, usize);

impl Coords {
    fn manhattan_dist(&self, b: &Self) -> usize {
        self.1.abs_diff(b.1) + self.0.abs_diff(b.0)
    }
}

impl std::ops::Add for Coords {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Sub for Coords {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl From<(usize, usize)> for Coords {
    fn from(value: (usize, usize)) -> Self {
        Self(value.0, value.1)
    }
}

impl std::fmt::Display for Coords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

fn solve(input: &str, expansion_size: usize) -> usize {
    let tiles = input.lines().map(str::as_bytes).collect::<Vec<_>>();
    let line_len_initial = tiles.len();
    let n_cols_initial = tiles[0].len();
    let blank_rows: Vec<usize> = (0..n_cols_initial)
        .filter(|&i| (0..line_len_initial).all(|j| tiles[i][j] == b'.'))
        .collect();
    // println!("{blank_rows:?}");
    let blank_cols: Vec<usize> = (0..line_len_initial)
        .filter(|&j| (0..n_cols_initial).all(|i| tiles[i][j] == b'.'))
        .collect();
    let galaxies = (0..line_len_initial)
        .flat_map(|i| (0..n_cols_initial).map(move |j| (i, j)))
        .filter(|&(i, j)| tiles[i][j] == b'#')
        .map(|(i, j)| {
            Coords(i, j)
                + Coords(
                    blank_rows.iter().filter(|&x| i.gt(x)).count() * (expansion_size - 1),
                    blank_cols.iter().filter(|&x| j.gt(x)).count() * (expansion_size - 1),
                )
        })
        .collect::<Vec<_>>();
    galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, galaxy)| galaxies[i..].iter().map(|x| galaxy.manhattan_dist(x)))
        .sum()
}

#[aoc(day11, part1)]
fn part1(input: &str) -> usize {
    solve(input, 2)
}

#[aoc(day11, part2)]
fn part2(input: &str) -> usize {
    solve(input, 1_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = "\
    ...#......\n\
    .......#..\n\
    #.........\n\
    ..........\n\
    ......#...\n\
    .#........\n\
    .........#\n\
    ..........\n\
    .......#..\n\
    #...#.....";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT_1), 374);
    }

    #[test]
    #[ignore = "just for experiments"]
    fn describe_expansion() {
        const INPUT_1_EXPANDED: &str = "\
        ....#........\n\
        .........#...\n\
        #............\n\
        .............\n\
        .............\n\
        ........#....\n\
        .#...........\n\
        ............#\n\
        .............\n\
        .............\n\
        .........#...\n\
        #....#.......";
        let gals_initial = INPUT_1
            .lines()
            .enumerate()
            .flat_map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, x)| '#'.eq(x))
                    .map(move |(j, _)| (i, j))
            })
            .collect::<Vec<_>>();
        println!("{gals_initial:?}");
        let rift_rows = INPUT_1
            .lines()
            .enumerate()
            .filter(|(_, x)| [b'.'; 10].eq(x.as_bytes()))
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        println!("rows {rift_rows:?}");
        let rift_cols = (0..10)
            .filter(|&j| INPUT_1.lines().all(|x| x.as_bytes()[j] == b'.'))
            .collect::<Vec<_>>();
        println!("cols {rift_cols:?}");
        let gals_expanded = INPUT_1_EXPANDED
            .lines()
            .enumerate()
            .flat_map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, x)| '#'.eq(x))
                    .map(move |(j, _)| (i, j))
            })
            .collect::<Vec<_>>();
        println!("{gals_expanded:?}");
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(INPUT_1, 10), 1030);
        assert_eq!(solve(INPUT_1, 100), 8410);
    }
}
