use std::iter::Sum;
use std::ops::Add;
use std::str::FromStr;

use anyhow::anyhow;

#[derive(Debug, Clone)]
pub struct Game {
    pub id: usize,
    pub handfuls: Vec<Handful>,
}

impl FromStr for Game {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id_part, handful_part) = s
            .split_once(": ")
            .ok_or(anyhow!("input should have a colon"))?;
        let id = id_part[5..].parse()?;
        let handfuls = handful_part
            .split("; ")
            .map(Handful::from_str)
            .collect::<Result<_, _>>()?;
        Ok(Self { id, handfuls })
    }
}

impl Game {
    pub fn _get_totals(&self) -> Totals {
        self.handfuls.iter().copied().sum()
    }

    pub fn is_possible(&self) -> bool {
        self.handfuls.iter().all(Handful::is_possible)
    }

    pub fn min_handful(&self) -> Handful {
        let red = self
            .handfuls
            .iter()
            .map(|x| x.red)
            .max()
            .expect("shouldnt be empty");
        let green = self
            .handfuls
            .iter()
            .map(|x| x.green)
            .max()
            .expect("shouldnt be empty");
        let blue = self
            .handfuls
            .iter()
            .map(|x| x.blue)
            .max()
            .expect("shouldnt be empty");
        Handful { red, green, blue }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Totals {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

impl From<Handful> for Totals {
    fn from(value: Handful) -> Self {
        let Handful { red, green, blue } = value;
        Self { red, green, blue }
    }
}

impl Add<Handful> for Totals {
    type Output = Self;
    fn add(self, rhs: Handful) -> Self::Output {
        Self {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl Sum<Handful> for Totals {
    fn sum<I: Iterator<Item = Handful>>(iter: I) -> Self {
        iter.fold(
            Self {
                red: 0,
                green: 0,
                blue: 0,
            },
            Self::add,
        )
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Handful {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

// impl Add for Handful {
//     type Output = Totals;
//     fn add(self, rhs: Self) -> Self::Output {
//         Totals {
//             red: self.red + rhs.red,
//             green: self.green + rhs.green,
//             blue: self.blue + rhs.blue,
//         }
//     }
// }

impl FromStr for Handful {
    type Err = anyhow::Error;
    // Expects a comma separated list of showings like "8 green, 6 blue, 20 red"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (mut red, mut green, mut blue) = (0, 0, 0);
        for color_cubes in s.split(", ") {
            let (num, color) = color_cubes
                .split_once(' ')
                .ok_or(anyhow!("Showings should have a space like \"8 green\""))?;
            let num: usize = num.parse()?;
            match color {
                "red" => red += num,
                "green" => green += num,
                "blue" => blue += num,
                _ => {
                    return Err(anyhow!(
                        "Only supports colors \"red\", \"green\", and \"blue\""
                    ))
                }
            }
        }
        Ok(Self { red, green, blue })
    }
}

const MAX: Handful = Handful {
    red: 12,
    green: 13,
    blue: 14,
};

impl Handful {
    pub const fn is_possible(&self) -> bool {
        self.red <= MAX.red && self.green <= MAX.green && self.blue <= MAX.blue
    }
    pub const fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .map(Game::from_str)
        .map(Result::unwrap)
        .filter(Game::is_possible)
        .map(|x| x.id)
        .sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &str) -> usize {
    input
        .lines()
        .map(Game::from_str)
        .map(Result::unwrap)
        .map(|x| x.min_handful())
        .map(|x| x.power())
        .sum()
}
