#[aoc(day14, part1)]
fn part1(input: &[u8]) -> usize {
    // for line in parse(input) {
    //     println!(
    //         "{:?}",
    //         line.iter().map(|&x| char::from(x)).collect::<Vec<_>>()
    //     );
    // }
    // hardcoded + test case
    let size = if input.len() < 10_000 { 10 } else { 100 };
    // transposing, also size offset for newline
    (0..size)
        .map(|j| {
            (0..size)
                .map(|i| (size - i, input[(size + 1) * i + j]))
                .filter(|(_, x)| b'.'.ne(x))
                .fold((size, 0), |(load_rows, total), (i, x)| {
                    if x == b'#' {
                        (i - 1, total)
                    } else {
                        (load_rows - 1, total + load_rows)
                    }
                })
                .1
        })
        .sum()
}

#[aoc(day14, part2)]
fn part2(input: &[u8]) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = b"\
    O....#....\n\
    O.OO#....#\n\
    .....##...\n\
    OO.#O....O\n\
    .O.....O#.\n\
    O.#..O.#.#\n\
    ..O..#O..O\n\
    .......O..\n\
    #....###..\n\
    #OO..#....";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 136);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 64);
    }
}
