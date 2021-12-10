fn solve1(input: &[i32]) -> i32 {
    let min_pos: i32 = *input.iter().min().unwrap();
    let max_pos: i32 = *input.iter().max().unwrap();
    let mut min_cost = i32::MAX;

    for align_pos in min_pos..=max_pos {
        let mut sum = 0;
        for crab in input {
            let diff = *crab - align_pos;
            sum += diff.abs();
        }

        if sum < min_cost {
            min_cost = sum;
        }
    }

    min_cost
}

fn solve2(input: &[i32]) -> i32 {
    let min_pos: i32 = *input.iter().min().unwrap();
    let max_pos: i32 = *input.iter().max().unwrap();
    let mut min_cost = i32::MAX;
    let mut diffs = vec![0; (max_pos + 1) as usize];

    for align_pos in min_pos..=max_pos {
        let mut sum = 0;
        for crab in input {
            let diff = (*crab - align_pos).abs();
            sum += if diffs[diff as usize] != 0 {
                diffs[diff as usize]
            } else {
                let mut precalc = 0;
                for x in 1..=diff {
                    precalc += x;
                }
                diffs[diff as usize] = precalc;
                precalc
            };
        }

        if sum < min_cost {
            min_cost = sum;
        }
    }

    min_cost
}

fn main() {
    let input: Vec<i32> = include_str!("input.txt")
        .trim()
        .split(',')
        .map(|l| l.parse::<i32>().unwrap())
        .collect();

    println!("part 1: {}", solve1(&input));
    println!("part 2: {}", solve2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[i32] = &[16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    #[test]
    fn test_solve1() {
        assert_eq!(solve1(&INPUT), 37);
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve2(&INPUT), 168);
    }
}
