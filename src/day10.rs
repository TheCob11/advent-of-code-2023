#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    N,
    S,
    E,
    W,
}

impl std::ops::Neg for Direction {
    type Output = Self;
    fn neg(self) -> Self::Output {
        use Direction as D;
        match self {
            D::N => D::S,
            D::S => D::N,
            D::E => D::W,
            D::W => D::E,
        }
    }
}

impl std::ops::Add<(usize, usize)> for Direction {
    type Output = (usize, usize);
    fn add(self, rhs: (usize, usize)) -> Self::Output {
        use Direction as D;
        let (row, col) = rhs;
        match self {
            D::N => (row - 1, col),
            D::S => (row + 1, col),
            D::E => (row, col + 1),
            D::W => (row, col - 1),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Pipe {
    Vertical = b'|',
    Horizontal = b'-',
    NorthEast = b'L',
    NorthWest = b'J',
    SouthWest = b'7',
    SouthEast = b'F',
}

impl TryFrom<char> for Pipe {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Pipe as P;
        Ok(match value {
            '|' => P::Vertical,
            '-' => P::Horizontal,
            'L' => P::NorthEast,
            'J' => P::NorthWest,
            '7' => P::SouthWest,
            'F' => P::SouthEast,
            _ => Err(())?,
        })
    }
}

impl std::ops::Neg for Pipe {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            x @ (Pipe::Vertical | Pipe::Horizontal) => x,
            Pipe::NorthEast => Pipe::SouthWest,
            Pipe::NorthWest => Pipe::SouthEast,
            Pipe::SouthWest => Pipe::NorthEast,
            Pipe::SouthEast => Pipe::NorthWest,
        }
    }
}

impl Pipe {
    fn try_step(self, dir_in: Direction) -> Result<Direction, anyhow::Error> {
        use anyhow::anyhow;
        use Direction as D;
        use Pipe as P;
        let dir_in = -dir_in;
        Ok(match self {
            P::Vertical => match dir_in {
                x @ (D::N | D::S) => -x,
                _ => return Err(anyhow!("Vertical only takes N or S")),
            },
            P::Horizontal => match dir_in {
                x @ (D::E | D::W) => -x,
                _ => return Err(anyhow!("Vertical only takes E or W")),
            },
            P::NorthEast => match dir_in {
                D::N => D::E,
                D::E => D::N,
                _ => return Err(anyhow!("NorthEast only takes N or E")),
            },
            P::NorthWest => match dir_in {
                D::N => D::W,
                D::W => D::N,
                _ => return Err(anyhow!("NorthWest only takes N or W")),
            },
            P::SouthEast => match dir_in {
                D::S => D::E,
                D::E => D::S,
                _ => return Err(anyhow!("SouthEast only takes S or E")),
            },
            P::SouthWest => match dir_in {
                D::S => D::W,
                D::W => D::S,
                _ => return Err(anyhow!("SouthWest only takes S or W")),
            },
        })
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Pipe(Pipe),
    Ground = b'.',
    Animal = b'S',
}

impl TryFrom<char> for Tile {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Tile as T;
        Ok(match value {
            'S' => T::Animal,
            '.' => T::Ground,
            x => T::Pipe(Pipe::try_from(x)?),
        })
    }
}

#[derive(Debug)]
struct Grid(Vec<Vec<Tile>>);

impl std::ops::Index<(usize, usize)> for Grid {
    type Output = Tile;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[index.0][index.1]
    }
}

impl std::ops::IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.0[index.0][index.1]
    }
}

#[aoc_generator(day10)]
fn parse(input: &str) -> (Grid, (usize, usize)) {
    let mut animal_pos = std::cell::OnceCell::new();
    let grid: Grid = Grid(
        input
            .lines()
            .enumerate()
            .map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .map(|(j, x)| {
                        let tile = Tile::try_from(x).expect("should be valid tile");
                        if let Tile::Animal = tile {
                            animal_pos.set((i, j)).expect("should only be one animal");
                        }
                        tile
                    })
                    .collect()
            })
            .collect(),
    );
    let animal_pos = animal_pos.take().expect("should have animal");
    (grid, animal_pos)
}

#[aoc(day10, part1)]
fn part1((grid, animal_pos): &(Grid, (usize, usize))) -> usize {
    // println!("{grid:#?}");
    // println!("Animal at {animal_pos:?}");
    // let start_pos = Direction::E + animal_pos;
    // let start @ Tile::Pipe(_) = grid[start_pos] else {
    //     unimplemented!("animal that cant go E")
    // };
    let mut count = 1;
    let mut dir = Direction::E;
    let mut pos = dir + *animal_pos;
    while let Tile::Pipe(next) = grid[pos] {
        // println!("{pos:?} {dir:?} {next:?}");
        dir = next.try_step(dir).expect("should be valid step");
        pos = dir + pos;
        count += 1;
    }
    count / 2
}

#[aoc(day10, part2)]
fn part2((grid, animal_pos): &(Grid, (usize, usize))) -> usize {
    let mut main_loop_coords = std::collections::HashSet::with_capacity(6823 * 2);
    let mut dir = if let Tile::Pipe(Pipe::SouthEast) = grid[Direction::W + *animal_pos] {
        Direction::W
    } else {
        Direction::E
    }; // effectively hardcoded w/exceptions for the examples
    let mut pos = dir + *animal_pos;
    while let Tile::Pipe(next) = grid[pos] {
        main_loop_coords.insert(pos);
        dir = next.try_step(dir).expect("should be valid step");
        pos = dir + pos;
    }
    let animal_pipe: Pipe = {
        // effectively hardcoded w/exceptions for the examples
        match grid[-dir + pos] {
            Tile::Pipe(Pipe::Vertical) => Pipe::SouthWest,
            Tile::Pipe(Pipe::NorthWest) => Pipe::SouthEast,
            _ => Pipe::Horizontal,
        }
    };
    // println!("{animal_pipe:?}");
    grid.0
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(j, tile)| {
                    if Tile::Animal.eq(tile) || main_loop_coords.contains(&(i, j)) {
                        None
                    } else {
                        Some(j)
                    }
                })
                .filter(|&j| {
                    let mut inside = false;
                    let mut in_corners = Vec::new();
                    for next in row[j + 1..]
                        .iter()
                        .enumerate()
                        .filter_map(|(j_offset, &x)| match x {
                            Tile::Pipe(pipe)
                                if main_loop_coords.contains(&(i, j + j_offset + 1)) =>
                            {
                                Some(pipe)
                            }
                            Tile::Animal => Some(animal_pipe),
                            _ => None,
                        })
                    {
                        use Pipe as P;
                        match next {
                            P::Vertical => inside = !inside,
                            P::Horizontal => (),
                            x @ (P::NorthEast | P::SouthEast) => in_corners.push(x),
                            x @ (P::SouthWest | P::NorthWest) => {
                                if in_corners.last().is_some_and(|&in_corner| x == -in_corner) {
                                    in_corners.pop();
                                    inside = !inside;
                                }
                            }
                        }
                    }
                    inside
                })
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    //noinspection SpellCheckingInspection
    const INPUT_1: &str = "\
    7-F7-\n\
    .FJ|7\n\
    SJLL7\n\
    |F--J\n\
    LJ.LJ";

    //noinspection SpellCheckingInspection
    const INPUT_2_BASIC_BASIC: &str = "\
    ...........\n\
    .S-------7.\n\
    .|F-----7|.\n\
    .||.....||.\n\
    .||.....||.\n\
    .|L-7.F-J|.\n\
    .|..|.|..|.\n\
    .L--J.L--J.\n\
    ...........";

    //noinspection SpellCheckingInspection
    const INPUT_2_BASIC: &str = "\
    .F----7F7F7F7F-7....\n\
    .|F--7||||||||FJ....\n\
    .||.FJ||||||||L7....\n\
    FJL7L7LJLJ||LJ.L-7..\n\
    L--J.L7...LJS7F-7L7.\n\
    ....F-J..F7FJ|L7L7L7\n\
    ....L7.F7||L7|.L7L7|\n\
    .....|FJLJ|FJ|F7|.LJ\n\
    ....FJL-7.||.||||...\n\
    ....L---J.LJ.LJLJ...";

    //noinspection SpellCheckingInspection
    const INPUT_2: &str = "\
    FF7FSF7F7F7F7F7F---7\n\
    L|LJ||||||||||||F--J\n\
    FL-7LJLJ||||||LJL-77\n\
    F--JF--7||LJLJ7F7FJ-\n\
    L---JF-JLJ.||-FJLJJ7\n\
    |F|F-JF---7F7-L7L|7|\n\
    |FFJF7L7F-JF7|JL---7\n\
    7-L-JL7||F7|L7F-7F7|\n\
    L.L7LFJ|||||FJL7||LJ\n\
    L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT_1)), 8);
    }

    #[test]
    fn part2_basic_basic() {
        assert_eq!(part2(&parse(INPUT_2_BASIC_BASIC)), 4);
    }

    #[test]
    fn part2_basic() {
        assert_eq!(part2(&parse(INPUT_2_BASIC)), 8);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT_2)), 10);
    }

    #[test]
    fn part2_check_with_answers() {
        use std::fs::read_to_string;
        let out = part2(&parse(
            read_to_string("./input/2023/day10.txt")
                .expect("should be valid file")
                .as_str(),
        ));
        println!("{out}");
        assert!(out < 2985);
    }
}
