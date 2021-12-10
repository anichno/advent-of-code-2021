fn solve1(input: &[Vec<i32>]) -> i32 {
    let mut sum = 0;
    let cols = input[0].len();
    for row in 0..input.len() {
        for col in 0..cols {
            let val = input[row][col];

            // left
            if col > 0 && input[row][col - 1] <= val {
                continue;
            }
            // up
            if row > 0 && input[row - 1][col] <= val {
                continue;
            }
            // right
            if col < cols - 1 && input[row][col + 1] <= val {
                continue;
            }
            // down
            if row < input.len() - 1 && input[row + 1][col] <= val {
                continue;
            }

            sum += input[row][col] + 1;
        }
    }

    sum
}

fn rec_fill_basin(
    cur_pos: (usize, usize),
    basin_id: i32,
    input: &[Vec<i32>],
    basin_tracker: &mut Vec<Vec<i32>>,
) {
    let rows = input.len();
    let cols = input[0].len();

    basin_tracker[cur_pos.1][cur_pos.0] = basin_id;

    // left
    if cur_pos.0 > 0
        && input[cur_pos.1][cur_pos.0 - 1] != 9
        && basin_tracker[cur_pos.1][cur_pos.0 - 1] == 0
    {
        rec_fill_basin((cur_pos.0 - 1, cur_pos.1), basin_id, input, basin_tracker);
    }
    // up
    if cur_pos.1 > 0
        && input[cur_pos.1 - 1][cur_pos.0] != 9
        && basin_tracker[cur_pos.1 - 1][cur_pos.0] == 0
    {
        rec_fill_basin((cur_pos.0, cur_pos.1 - 1), basin_id, input, basin_tracker);
    }
    // right
    if cur_pos.0 < cols - 1
        && input[cur_pos.1][cur_pos.0 + 1] != 9
        && basin_tracker[cur_pos.1][cur_pos.0 + 1] == 0
    {
        rec_fill_basin((cur_pos.0 + 1, cur_pos.1), basin_id, input, basin_tracker);
    }
    // down
    if cur_pos.1 < rows - 1
        && input[cur_pos.1 + 1][cur_pos.0] != 9
        && basin_tracker[cur_pos.1 + 1][cur_pos.0] == 0
    {
        rec_fill_basin((cur_pos.0, cur_pos.1 + 1), basin_id, input, basin_tracker);
    }
}

fn solve2(input: &[Vec<i32>]) -> i32 {
    let rows = input.len();
    let cols = input[0].len();
    let mut basin_tracker: Vec<Vec<i32>> = vec![vec![0; cols]; rows];
    let mut basin_id = 1;
    for row in 0..rows {
        for col in 0..cols {
            if input[row][col] == 9 || basin_tracker[row][col] > 0 {
                continue;
            }
            rec_fill_basin((col, row), basin_id, input, &mut basin_tracker);
            basin_id += 1;
        }
    }

    let mut basins = Vec::new();
    for x in 1..basin_id {
        let mut tot = 0;
        for row in basin_tracker.iter() {
            tot += row.iter().filter(|r| **r == x).count();
        }
        basins.push(tot);
    }

    basins.sort_unstable_by(|a, b| b.cmp(a));

    basins.iter().take(3).fold(1, |a, x| a * *x as i32)
}

fn main() {
    let mut parsed: Vec<Vec<i32>> = Vec::new();
    for line in include_str!("input.txt").trim().lines() {
        parsed.push(Vec::from_iter(
            line.chars().map(|d| d.to_digit(10).unwrap() as i32),
        ));
    }

    println!("part 1: {}", solve1(&parsed));
    println!("part 2: {}", solve2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[&[i32]] = &[
        &[2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
        &[3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
        &[9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
        &[8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
        &[9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
    ];

    #[test]
    fn test_solve1() {
        let mut input: Vec<Vec<i32>> = Vec::new();
        for row in INPUT {
            input.push(Vec::from(*row));
        }
        assert_eq!(solve1(&input), 15);
    }

    #[test]
    fn test_solve2() {
        let mut input: Vec<Vec<i32>> = Vec::new();
        for row in INPUT {
            input.push(Vec::from(*row));
        }
        assert_eq!(solve2(&input), 1134);
    }
}
