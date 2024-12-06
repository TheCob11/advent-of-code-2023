// didnt actually end up using this
mod terrain {
    use std::fmt;

    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    #[repr(u8)]
    pub enum Terrain {
        Ash = b'.',
        Rock = b'#',
    }

    impl TryFrom<u8> for Terrain {
        type Error = ();
        fn try_from(value: u8) -> Result<Self, Self::Error> {
            Ok(match value {
                b'.' => Terrain::Ash,
                b'#' => Terrain::Rock,
                _ => return Err(()),
            })
        }
    }

    impl std::ops::Not for Terrain {
        type Output = Self;
        fn not(self) -> Self::Output {
            match self {
                Terrain::Ash => Terrain::Rock,
                Terrain::Rock => Terrain::Ash,
            }
        }
    }

    impl fmt::Display for Terrain {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            use fmt::Write;
            f.write_char(char::from(*self as u8))
        }
    }
}

use std::collections::HashSet;
use std::ops::ControlFlow::{Break, Continue};

fn filter_reflex_points<T>(reflex_candidates: HashSet<usize>, list: &[T]) -> HashSet<usize>
where
    T: PartialEq,
{
    let mut reflx_cans = reflex_candidates;
    let len = list.len();
    let midpt = len.div_ceil(2);
    let odd_offset = len % 2;
    reflx_cans.retain(|&i| match i.checked_sub(midpt) {
        Some(diff) => list[i..].iter().rev().eq(&list[2 * diff + odd_offset..i]),
        None => list[i..2 * i].iter().rev().eq(&list[..i]),
    });
    reflx_cans
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Reflector {
    ReflexCol(usize),
    ReflexRow(usize),
}

impl Reflector {
    fn _into_inner(self) -> usize {
        let (Reflector::ReflexRow(x) | Reflector::ReflexCol(x)) = self;
        x
    }

    fn inner_scaled(self) -> usize {
        match self {
            Reflector::ReflexCol(x) => x,
            Reflector::ReflexRow(x) => x * 100,
        }
    }

    fn _is_col(self) -> bool {
        matches!(self, Reflector::ReflexCol(_))
    }
    fn _is_row(self) -> bool {
        matches!(self, Reflector::ReflexRow(_))
    }
}

impl From<Reflector> for (usize, usize) {
    fn from(value: Reflector) -> Self {
        match value {
            Reflector::ReflexRow(i) => (i, 0),
            Reflector::ReflexCol(j) => (0, j),
        }
    }
}

fn get_reflector(grid: &[Vec<u8>]) -> Reflector {
    let fold_res = grid
        .iter()
        .try_fold((1..grid[0].len()).collect(), |reflx_cols, row| {
            let reflx_cols = filter_reflex_points(reflx_cols, row);
            if reflx_cols.is_empty() {
                Break(())
            } else {
                Continue(reflx_cols)
            }
        });
    let is_col = fold_res.is_continue();
    let index = match fold_res {
        Continue(mut x) => x,
        Break(()) => (0..grid[0].len())
            .map(|j| (0..grid.len()).map(|i| grid[i][j]).collect::<Vec<_>>())
            .fold((1..grid.len()).collect(), |reflx_rows, ref col| {
                filter_reflex_points(reflx_rows, col)
            }),
    }
    .drain()
    .next()
    .unwrap();
    if is_col {
        Reflector::ReflexCol(index)
    } else {
        Reflector::ReflexRow(index)
    }
}

fn parse(input: &str) -> impl Iterator<Item = Vec<Vec<u8>>> + '_ {
    input.split("\n\n").map(|pattern| {
        pattern
            .lines()
            .map(str::as_bytes)
            .map(<[_]>::to_vec)
            .collect::<Vec<_>>()
    })
}

#[aoc(day13, part1)]
fn part1(input: &str) -> usize {
    parse(input)
        .map(|ref x| get_reflector(x).inner_scaled())
        .sum()
}

fn get_reflector_except(grid: &[Vec<u8>], wrong_reflector: Reflector) -> Option<Reflector> {
    let (line_len, grid_len) = (grid[0].len(), grid.len());
    let (reflx_cols_init, reflx_rows_init): (HashSet<usize>, HashSet<usize>) = match wrong_reflector
    {
        Reflector::ReflexCol(j) => (
            (1..line_len).filter(|x| j.ne(x)).collect(),
            (1..grid_len).collect(),
        ),
        Reflector::ReflexRow(i) => (
            (1..line_len).collect(),
            (1..grid_len).filter(|x| i.ne(x)).collect(),
        ),
    };
    let fold_res = grid.iter().try_fold(reflx_cols_init, |reflx_cols, row| {
        let reflx_cols = filter_reflex_points(reflx_cols, row);
        if reflx_cols.is_empty() {
            Break(())
        } else {
            Continue(reflx_cols)
        }
    });
    let is_col = fold_res.is_continue();
    let index = match fold_res {
        Continue(x) => x,
        Break(()) => (0..line_len)
            .map(|j| (0..grid_len).map(|i| grid[i][j]).collect::<Vec<_>>())
            .fold(reflx_rows_init, |reflx_rows, ref col| {
                filter_reflex_points(reflx_rows, col)
            }),
    }
    .drain()
    .next();
    if is_col {
        index.map(Reflector::ReflexCol)
    } else {
        index.map(Reflector::ReflexRow)
    }
}

#[aoc(day13, part2)]
fn part2(input: &str) -> usize {
    parse(input)
        .map(|x| (get_reflector(x.as_slice()), x))
        .map(|(reflector_init, mut grid)| {
            let (grid_len, line_len) = (grid.len(), grid[0].len());
            for (i, j) in (0..grid_len).flat_map(|i| std::iter::repeat(i).zip(0..line_len)) {
                let old_point = grid[i][j];
                grid[i][j] = match old_point {
                    b'.' => b'#',
                    b'#' => b'.',
                    _ => panic!("should be ash or rock"),
                };
                let reflx = get_reflector_except(&grid, reflector_init);
                // println!("{i} {j} {old_point} {reflx:?}");
                if let Some(x) = reflx {
                    if x != reflector_init {
                        return x;
                    }
                };
                grid[i][j] = old_point;
            }
            panic!("should find a smudge")
        })
        .map(Reflector::inner_scaled)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
    #.##..##.\n\
    ..#.##.#.\n\
    ##......#\n\
    ##......#\n\
    ..#.##.#.\n\
    ..##..##.\n\
    #.#.##.#.\n\
    \n\
    #...##..#\n\
    #....#..#\n\
    ..##..###\n\
    #####.##.\n\
    #####.##.\n\
    ..##..###\n\
    #....#..#";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 405);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 400);
    }
}
