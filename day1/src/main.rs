fn solve1(input: &[i32]) -> i32 {
    let mut increased = 0;
    let mut prev = input[0];
    for val in input.iter().skip(1) {
        if *val > prev {
            increased += 1;
        }
        prev = *val;
    }

    increased
}

fn solve2(input: &[i32]) -> i32 {
    let mut increased = 0;
    let mut prev = None;
    for window in input.windows(3) {
        let val: i32 = window.iter().sum();
        if let Some(prev) = prev {
            if val > prev {
                increased += 1;
            }
        }

        prev = Some(val);
    }

    increased
}

fn main() {
    let input: Vec<i32> = include_str!("input.txt")
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();
    println!("part 1: {}", solve1(&input));
    println!("part 2: {}", solve2(&input));
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &[i32] = &[199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn test_part_1() {
        assert_eq!(solve1(&INPUT), 7);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve2(&INPUT), 5);
    }
}
