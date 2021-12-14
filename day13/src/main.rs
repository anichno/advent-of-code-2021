use std::collections::HashSet;

enum Fold {
    X(usize),
    Y(usize),
}

fn parse_input(input: &[&str]) -> (HashSet<(usize, usize)>, Vec<Fold>) {
    let mut map: HashSet<(usize, usize)> = HashSet::new();

    let mut lines = input.iter();
    for line in &mut lines {
        // break when get to folds
        if line.is_empty() {
            break;
        }

        let (left, right) = line.split_once(',').unwrap();
        let left = left.parse().unwrap();
        let right = right.parse().unwrap();
        map.insert((left, right));
    }

    let mut folds = Vec::new();
    for line in lines {
        let (left, right) = line.split_once('=').unwrap();
        let left = left.chars().last().unwrap();
        let right = right.parse().unwrap();

        match left {
            'x' => folds.push(Fold::X(right)),
            'y' => folds.push(Fold::Y(right)),
            _ => panic!(),
        }
    }

    (map, folds)
}

fn display(input: &HashSet<(usize, usize)>) {
    let mut max_x = 0;
    let mut max_y = 0;
    for (x, y) in input.iter() {
        if *x > max_x {
            max_x = *x;
        }

        if *y > max_y {
            max_y = *y;
        }
    }

    let mut grid = vec![vec![false; max_x + 1]; max_y + 1];
    for (x, y) in input.iter() {
        grid[*y][*x] = true;
    }
    for row in grid {
        for col in row {
            if col {
                print!("#");
            } else {
                print!(".");
            }
        }

        println!();
    }

    println!();
}

fn solve1(mut input: HashSet<(usize, usize)>, folds: &[Fold]) -> usize {
    match folds[0] {
        Fold::X(xline) => {
            let mut remove_list = Vec::new();
            let mut add_list = Vec::new();
            for (x, y) in input.iter() {
                if *x > xline {
                    let new_x = xline - (*x - xline);
                    remove_list.push((*x, *y));
                    add_list.push((new_x, *y));
                }
            }

            for remove in remove_list {
                input.remove(&remove);
            }

            for add in add_list {
                input.insert(add);
            }
        }
        Fold::Y(yline) => {
            let mut remove_list = Vec::new();
            let mut add_list = Vec::new();
            for (x, y) in input.iter() {
                if *y > yline {
                    let new_y = yline - (*y - yline);
                    remove_list.push((*x, *y));
                    add_list.push((*x, new_y));
                }
            }

            for remove in remove_list {
                input.remove(&remove);
            }

            for add in add_list {
                input.insert(add);
            }
        }
    }

    input.len()
}

fn solve2(mut input: HashSet<(usize, usize)>, folds: &[Fold]) {
    for fold in folds {
        match *fold {
            Fold::X(xline) => {
                let mut remove_list = Vec::new();
                let mut add_list = Vec::new();
                for (x, y) in input.iter() {
                    if *x > xline {
                        let new_x = xline - (*x - xline);
                        remove_list.push((*x, *y));
                        add_list.push((new_x, *y));
                    }
                }

                for remove in remove_list {
                    input.remove(&remove);
                }

                for add in add_list {
                    input.insert(add);
                }
            }
            Fold::Y(yline) => {
                let mut remove_list = Vec::new();
                let mut add_list = Vec::new();
                for (x, y) in input.iter() {
                    if *y > yline {
                        let new_y = yline - (*y - yline);
                        remove_list.push((*x, *y));
                        add_list.push((*x, new_y));
                    }
                }

                for remove in remove_list {
                    input.remove(&remove);
                }

                for add in add_list {
                    input.insert(add);
                }
            }
        }
    }

    display(&input);
}

fn main() {
    let input: Vec<&str> = include_str!("input.txt").trim().lines().collect();
    let (map, folds) = parse_input(&input);

    println!("part 1: {}", solve1(map.clone(), &folds));
    println!("part 2:");
    solve2(map, &folds);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[&str] = &[
        "6,10",
        "0,14",
        "9,10",
        "0,3",
        "10,4",
        "4,11",
        "6,0",
        "6,12",
        "4,1",
        "0,13",
        "10,12",
        "3,4",
        "3,0",
        "8,4",
        "1,10",
        "2,14",
        "8,10",
        "9,0",
        "",
        "fold along y=7",
        "fold along x=5",
    ];

    #[test]
    fn test_solve1() {
        let (grid, folds) = parse_input(&INPUT);
        assert_eq!(solve1(grid, &folds), 17);
    }
}
