use regex::Regex;

#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> usize {
    let line_length = input.find('\n').expect("input should have newlines") + 1;
    let input = input.replace('\n', ".");
    let re = Regex::new(r"\d+").expect("pattern should be valid regex");
    let matches = re.find_iter(input.as_str());
    let bytes = input.as_bytes();
    matches
        // .inspect(|x| println!("{x:?}"))
        .filter(|x| {
            let prev = x.start() - 1;
            let post = x.end();
            bytes[prev] != b'.'
                || bytes[post] != b'.'
                || bytes
                    .get(prev - line_length..=post - line_length)
                    .is_some_and(|y| y != [b'.'].repeat(y.len()))
                || bytes
                    .get(prev + line_length..=post + line_length)
                    .is_some_and(|y| y != [b'.'].repeat(y.len()))
        })
        .map(|x| x.as_str().parse::<usize>().expect("should be num"))
        .sum()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &str) -> usize {
    let line_length = input.find('\n').expect("input should have newlines") + 1;
    let input = input.replace('\n', ".");
    let input = input.as_str();
    let re = Regex::new(r"\*").expect("pattern should be valid regex");
    let left_num_re = Regex::new(r"(\d+)\*").expect("pattern should be valid regex");
    let right_num_re = Regex::new(r"\*(\d+)").expect("pattern should be valid regex");
    let diag_num_re = fancy_regex::Regex::new(r"\d{3}|(?<=.)\d{2}(?=.)|(?<=..)\d(?=..)")
        .expect("pattern should be valid regex");
    let matches = re.find_iter(input);
    let bytes = input.as_bytes();
    matches
        .filter_map(|symbol_match| {
            let mut adj_parts: Vec<usize> = vec![];
            let (prev, post) = (symbol_match.start() - 1, symbol_match.end());
            if bytes[prev].is_ascii_digit() {
                adj_parts.push(
                    left_num_re
                        .captures(&input[prev - 2..=prev + 1])
                        .expect("should have a number since it was just checked")[1]
                        .parse()
                        .expect("should be num"),
                );
            }
            if bytes[post].is_ascii_digit() {
                adj_parts.push(
                    right_num_re
                        .captures(&input[post - 1..=post + 2])
                        .expect("should have a number since it was just checked")[1]
                        .parse()
                        .expect("should be num"),
                );
            }
            adj_parts.extend(
                diag_num_re
                    .find_iter(&input[prev - 2 - line_length..=post + 2 - line_length])
                    .map(|x| {
                        x.expect("should match?")
                            .as_str()
                            .parse::<usize>()
                            .expect("should be num")
                    }),
            );
            if adj_parts.len() > 2 {
                return None;
            }
            adj_parts.extend(
                diag_num_re
                    .find_iter(&input[prev - 2 + line_length..=post + 2 + line_length])
                    .map(|x| {
                        x.expect("should match?")
                            .as_str()
                            .parse::<usize>()
                            .expect("should be num")
                    }),
            );
            if adj_parts.len() == 2 {
                Some(adj_parts[0] * adj_parts[1])
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(solve_part2(input), 467_835);
    }
}
