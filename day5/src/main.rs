use std::collections::HashMap;

struct Line {
    start_x: i32,
    start_y: i32,
    end_x: i32,
    end_y: i32,
}

fn parse_input(input: &[&str]) -> Vec<Line> {
    let mut lines = Vec::new();

    for line in input {
        let (left, right) = line.split_once(" -> ").unwrap();
        let (start_x, start_y) = left.split_once(",").unwrap();
        let (end_x, end_y) = right.split_once(",").unwrap();
        let start_x = start_x.parse().unwrap();
        let start_y = start_y.parse().unwrap();
        let end_x = end_x.parse().unwrap();
        let end_y = end_y.parse().unwrap();

        lines.push(Line {
            start_x,
            start_y,
            end_x,
            end_y,
        });
    }

    lines
}

fn solve1(lines: &[Line]) -> i32 {
    let mut grid: HashMap<(i32, i32), i32> = HashMap::new();

    for line in lines {
        // only horizontal or vertical lines
        if line.start_x == line.end_x || line.start_y == line.end_y {
            let mut x = line.start_x;
            let x_step = if line.start_x > line.end_x { -1 } else { 1 };

            let mut y = line.start_y;
            let y_step = if line.start_y > line.end_y { -1 } else { 1 };

            loop {
                loop {
                    let point = grid.entry((x, y)).or_default();
                    *point += 1;

                    if y == line.end_y {
                        break;
                    }
                    y += y_step;
                }

                if x == line.end_x {
                    break;
                }
                x += x_step;
            }
        }
    }

    // get score
    let mut score = 0;
    for val in grid.values() {
        if *val >= 2 {
            score += 1;
        }
    }

    score
}

fn solve2(lines: &[Line]) -> i32 {
    let mut grid: HashMap<(i32, i32), i32> = HashMap::new();

    for line in lines {
        let mut x = line.start_x;
        let x_step = if line.start_x > line.end_x { -1 } else { 1 };

        let mut y = line.start_y;
        let y_step = if line.start_y > line.end_y { -1 } else { 1 };

        // only horizontal or vertical lines
        if line.start_x == line.end_x || line.start_y == line.end_y {
            loop {
                loop {
                    let point = grid.entry((x, y)).or_default();
                    *point += 1;

                    if y == line.end_y {
                        break;
                    }
                    y += y_step;
                }

                if x == line.end_x {
                    break;
                }
                x += x_step;
            }
        } else {
            // diagonal
            loop {
                let point = grid.entry((x, y)).or_default();
                *point += 1;

                if x == line.end_x && y == line.end_y {
                    break;
                }
                x += x_step;
                y += y_step;
            }
        }
    }

    // get score
    let mut score = 0;
    for val in grid.values() {
        if *val >= 2 {
            score += 1;
        }
    }

    score
}

fn main() {
    let input: Vec<&str> = include_str!("input.txt").lines().collect();
    let parsed = parse_input(&input);

    println!("part 1: {}", solve1(&parsed));
    println!("part 2: {}", solve2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[&str] = &[
        "0,9 -> 5,9",
        "8,0 -> 0,8",
        "9,4 -> 3,4",
        "2,2 -> 2,1",
        "7,0 -> 7,4",
        "6,4 -> 2,0",
        "0,9 -> 2,9",
        "3,4 -> 1,4",
        "0,0 -> 8,8",
        "5,5 -> 8,2",
    ];

    #[test]
    fn test_solve1() {
        let parsed = parse_input(&INPUT);
        assert_eq!(solve1(&parsed), 5);
    }

    #[test]
    fn test_solve2() {
        let parsed = parse_input(&INPUT);
        assert_eq!(solve2(&parsed), 12);
    }
}
