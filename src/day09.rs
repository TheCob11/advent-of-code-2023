fn delta_list(l: &[isize]) -> impl Iterator<Item = isize> + '_ {
    l.windows(2).map(|x| match x {
        [a, b] => b - a,
        _ => panic!("should be windows of 2"),
    })
}

fn predict_next(l: &[isize]) -> isize {
    let [tail @ .., head] = l else {
        panic!("should have at least 2 elems")
    };
    if tail.iter().all(|x| x == head) {
        *head
    } else {
        *head + predict_next(delta_list(l).collect::<Vec<_>>().as_slice())
    }
}

#[aoc(day9, part1)]
fn part1(input: &str) -> isize {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().expect("should be valid num"))
                .collect::<Vec<_>>()
        })
        .map(|x| predict_next(x.as_slice()))
        .sum()
}

fn predict_prev(l: &[isize]) -> isize {
    let [head, tail @ ..] = l else {
        panic!("should have at least 2 elems")
    };
    if tail.iter().all(|x| x == head) {
        *head
    } else {
        *head - predict_prev(delta_list(l).collect::<Vec<_>>().as_slice())
    }
}

#[aoc(day9, part2)]
fn part2(input: &str) -> isize {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().expect("should be valid num"))
                .collect::<Vec<_>>()
        })
        .map(|x| predict_prev(x.as_slice()))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 114);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 2);
    }
}
