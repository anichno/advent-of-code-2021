fn solve1(mut input: Vec<Vec<u32>>) -> u64 {
    let mut tot_flashes = 0;

    for _step in 0..100 {
        let mut flashed = vec![vec![false; 10]; 10];

        // inc all
        for row in input.iter_mut() {
            for col in row.iter_mut() {
                *col += 1;
            }
        }

        // check for flashes
        loop {
            let mut flash = false;

            'flash_scanner: for row in 0..10 {
                for col in 0..10 {
                    if input[row][col] > 9 && !flashed[row][col] {
                        //flash
                        flash = true;
                        flashed[row][col] = true;
                        tot_flashes += 1;

                        //power neighbors
                        for ydiff in -1..=1 {
                            let y = row as i32 + ydiff;
                            if !(0..10).contains(&y) {
                                continue;
                            }
                            for xdiff in -1..=1 {
                                let x = col as i32 + xdiff;
                                if (0..10).contains(&x) && !(ydiff == 0 && xdiff == 0) {
                                    input[y as usize][x as usize] += 1;
                                }
                            }
                        }

                        break 'flash_scanner;
                    }
                }
            }

            if !flash {
                break;
            }
        }

        // reset flashed
        for row in input.iter_mut() {
            for col in row.iter_mut() {
                if *col > 9 {
                    *col = 0;
                }
            }
        }
    }

    tot_flashes
}

fn solve2(mut input: Vec<Vec<u32>>) -> u64 {
    let mut step = 0;

    loop {
        let mut flashed = vec![vec![false; 10]; 10];
        let mut num_flashed = 0;

        // inc all
        for row in input.iter_mut() {
            for col in row.iter_mut() {
                *col += 1;
            }
        }

        // check for flashes
        loop {
            let mut flash = false;

            'flash_scanner: for row in 0..10 {
                for col in 0..10 {
                    if input[row][col] > 9 && !flashed[row][col] {
                        //flash
                        flash = true;
                        flashed[row][col] = true;
                        num_flashed += 1;

                        //power neighbors
                        for ydiff in -1..=1 {
                            let y = row as i32 + ydiff;
                            if !(0..10).contains(&y) {
                                continue;
                            }
                            for xdiff in -1..=1 {
                                let x = col as i32 + xdiff;
                                if (0..10).contains(&x) && !(ydiff == 0 && xdiff == 0) {
                                    input[y as usize][x as usize] += 1;
                                }
                            }
                        }

                        break 'flash_scanner;
                    }
                }
            }

            if !flash {
                break;
            }
        }

        // reset flashed
        for row in input.iter_mut() {
            for col in row.iter_mut() {
                if *col > 9 {
                    *col = 0;
                }
            }
        }

        step += 1;

        if num_flashed == 100 {
            break;
        }
    }

    step
}

fn main() {
    let input: Vec<Vec<u32>> = include_str!("input.txt")
        .trim()
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    println!("part 1: {}", solve1(input.clone()));
    println!("part 2: {}", solve2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[&[u32]] = &[
        &[5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
        &[2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
        &[5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
        &[6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
        &[6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
        &[4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
        &[2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
        &[6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
        &[4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
        &[5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
    ];

    #[test]
    fn test_solve1() {
        let mut input: Vec<Vec<u32>> = Vec::new();
        for row in INPUT.iter() {
            input.push(Vec::from(*row));
        }

        assert_eq!(solve1(input.clone()), 1656);
    }

    #[test]
    fn test_solve2() {
        let mut input: Vec<Vec<u32>> = Vec::new();
        for row in INPUT.iter() {
            input.push(Vec::from(*row));
        }

        assert_eq!(solve2(input.clone()), 195);
    }
}
