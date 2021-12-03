use std::collections::HashSet;

fn solve1(input: &[&str]) -> i32 {
    let mut gamma = 0;
    let mut epsilon = 0;

    let number_len = input[0].len();

    for x in (0..number_len).rev() {
        let mut zeros = 0;
        let mut ones = 0;

        for val in input.iter() {
            match val.as_bytes()[x] as char {
                '0' => zeros += 1,
                '1' => ones += 1,
                _ => panic!("bad binary: {}", val),
            };
        }

        if ones > zeros {
            gamma |= 1 << (number_len - x - 1);
        } else {
            epsilon |= 1 << (number_len - x - 1);
        }
    }

    gamma * epsilon
}

fn solve2(input: &[&str]) -> i32 {
    let mut oxygen_set: HashSet<_> = HashSet::from_iter(input.iter());
    let mut scrubber_set: HashSet<_> = HashSet::from_iter(input.iter());

    let number_len = input[0].len();

    for x in 0..number_len {
        if oxygen_set.len() > 1 {
            let mut zeros = 0;
            let mut ones = 0;

            for candidate in oxygen_set.iter() {
                match candidate.as_bytes()[x] as char {
                    '0' => zeros += 1,
                    '1' => ones += 1,
                    _ => panic!("bad binary: {}", candidate),
                };
            }

            let mut remove_set = HashSet::new();

            for candidate in oxygen_set.iter() {
                match candidate.as_bytes()[x] as char {
                    '0' => {
                        if ones >= zeros {
                            remove_set.insert(*candidate);
                        }
                    }
                    '1' => {
                        if zeros > ones {
                            remove_set.insert(*candidate);
                        }
                    }
                    _ => panic!("bad binary: {}", candidate),
                };
            }

            for remove in remove_set {
                oxygen_set.remove(remove);
            }
        }

        if scrubber_set.len() > 1 {
            let mut zeros = 0;
            let mut ones = 0;
            for candidate in scrubber_set.iter() {
                match candidate.as_bytes()[x] as char {
                    '0' => zeros += 1,
                    '1' => ones += 1,
                    _ => panic!("bad binary: {}", candidate),
                };
            }

            let mut remove_set = HashSet::new();

            for candidate in scrubber_set.iter() {
                match candidate.as_bytes()[x] as char {
                    '0' => {
                        if zeros > ones {
                            remove_set.insert(*candidate);
                        }
                    }
                    '1' => {
                        if ones >= zeros {
                            remove_set.insert(*candidate);
                        }
                    }
                    _ => panic!("bad binary: {}", candidate),
                };
            }

            for remove in remove_set {
                scrubber_set.remove(remove);
            }
        }
    }

    let oxygen = i32::from_str_radix(**oxygen_set.iter().next().unwrap(), 2).unwrap();
    let scrubber = i32::from_str_radix(**scrubber_set.iter().next().unwrap(), 2).unwrap();

    oxygen * scrubber
}

fn main() {
    let input: Vec<&str> = include_str!("input.txt").lines().collect();
    println!("part 1: {}", solve1(&input));
    println!("part 2: {}", solve2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[&str] = &[
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];

    #[test]
    fn test_solve1() {
        assert_eq!(solve1(INPUT), 198);
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve2(INPUT), 230);
    }
}
