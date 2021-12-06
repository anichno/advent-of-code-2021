fn solve1(input: &[i32]) -> usize {
    let mut fish = Vec::from(input);
    let mut new_fish = Vec::new();

    for _ in 0..80 {
        for fish in fish.iter_mut() {
            *fish -= 1;
            if *fish == -1 {
                *fish = 6;
                new_fish.push(8);
            }
        }

        fish.append(&mut new_fish);
    }

    fish.len()
}

fn solve2(input: &[i32]) -> usize {
    let mut fish_tracker = [0; 9];
    for fish in input {
        fish_tracker[*fish as usize] += 1;
    }

    for _ in 0..256 {
        let new_fish = fish_tracker[0];
        for i in 1..9 {
            fish_tracker[i - 1] = fish_tracker[i]
        }
        fish_tracker[6] += new_fish;
        fish_tracker[8] = new_fish;
    }

    fish_tracker.iter().sum::<usize>()
}

fn main() {
    let input: Vec<i32> = include_str!("input.txt")
        .trim()
        .split(',')
        .map(|l| l.parse().unwrap())
        .collect();

    println!("part 1: {}", solve1(&input));
    println!("part 2: {}", solve2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[i32] = &[3, 4, 3, 1, 2];

    #[test]
    fn test_solve1() {
        assert_eq!(solve1(&INPUT), 5934);
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve2(&INPUT), 26984457539);
    }
}
