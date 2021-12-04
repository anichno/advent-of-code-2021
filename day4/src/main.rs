use std::collections::HashSet;

fn solve1(numbers: &[i32], boards: &[Vec<Vec<i32>>]) -> i32 {
    let mut board_tracker = vec![vec![vec![false; 5]; 5]; boards.len()];

    for num in numbers {
        for (board_id, board) in boards.iter().enumerate() {
            for (row_id, row) in board.iter().enumerate() {
                for (col_id, col) in row.iter().enumerate() {
                    if col == num {
                        board_tracker[board_id][row_id][col_id] = true;

                        // evaluate if bingo
                        let mut bingo = false;
                        // scan rows
                        for tracker_row in &board_tracker[board_id] {
                            let mut found_bingo = true;
                            for col in tracker_row.iter() {
                                if !col {
                                    found_bingo = false;
                                    break;
                                }
                            }
                            if found_bingo {
                                bingo = true;
                                break;
                            }
                        }

                        if !bingo {
                            // scan cols
                            for x in 0..5 {
                                let mut found_bingo = true;
                                for y in 0..5 {
                                    if !board_tracker[board_id][y][x] {
                                        found_bingo = false;
                                        break;
                                    }
                                }
                                if found_bingo {
                                    bingo = true;
                                    break;
                                }
                            }
                        }

                        if bingo {
                            // get score
                            let wining_board = &boards[board_id];
                            let wining_tracker = &board_tracker[board_id];
                            let mut score = 0;

                            for (row, tracker_row) in wining_board.iter().zip(wining_tracker.iter())
                            {
                                for (col, tracker_col) in row.iter().zip(tracker_row.iter()) {
                                    if !tracker_col {
                                        score += col;
                                    }
                                }
                            }

                            score *= num;

                            return score;
                        }
                    }
                }
            }
        }
    }

    0
}

fn solve2(numbers: &[i32], boards: &[Vec<Vec<i32>>]) -> i32 {
    let mut boards_left: HashSet<_> = HashSet::from_iter(0..boards.len());
    let mut board_tracker = vec![vec![vec![false; 5]; 5]; boards.len()];

    for num in numbers {
        for (board_id, board) in boards.iter().enumerate() {
            if !boards_left.contains(&board_id) {
                continue;
            }
            for (row_id, row) in board.iter().enumerate() {
                for (col_id, col) in row.iter().enumerate() {
                    if col == num {
                        board_tracker[board_id][row_id][col_id] = true;

                        // evaluate if bingo
                        let mut bingo = false;
                        // scan rows
                        for tracker_row in &board_tracker[board_id] {
                            let mut found_bingo = true;
                            for col in tracker_row.iter() {
                                if !col {
                                    found_bingo = false;
                                    break;
                                }
                            }
                            if found_bingo {
                                bingo = true;
                                break;
                            }
                        }

                        if !bingo {
                            // scan cols
                            for x in 0..5 {
                                let mut found_bingo = true;
                                for y in 0..5 {
                                    if !board_tracker[board_id][y][x] {
                                        found_bingo = false;
                                        break;
                                    }
                                }
                                if found_bingo {
                                    bingo = true;
                                    break;
                                }
                            }
                        }

                        if bingo && boards_left.len() == 1 {
                            // get score
                            let wining_board = &boards[board_id];
                            let wining_tracker = &board_tracker[board_id];
                            let mut score = 0;

                            for (row, tracker_row) in wining_board.iter().zip(wining_tracker.iter())
                            {
                                for (col, tracker_col) in row.iter().zip(tracker_row.iter()) {
                                    if !tracker_col {
                                        score += col;
                                    }
                                }
                            }

                            score *= num;

                            return score;
                        } else if bingo {
                            boards_left.remove(&board_id);
                        }
                    }
                }
            }
        }
    }

    0
}

fn main() {
    let mut input = include_str!("input.txt").lines();
    let numbers: Vec<i32> = input
        .next()
        .unwrap()
        .split(',')
        .map(|txt_num| txt_num.parse::<i32>().unwrap())
        .collect();

    input.next().unwrap();

    // parse boards
    let mut boards = Vec::new();
    loop {
        let mut board = Vec::new();
        for _ in 0..5 {
            let row: Vec<i32> = input
                .next()
                .unwrap()
                .split_whitespace()
                .map(|val| val.parse::<i32>().unwrap())
                .collect();
            board.push(row);
        }
        boards.push(board);

        if input.next().is_none() {
            break;
        }
    }

    println!("part 1: {}", solve1(&numbers, &boards));
    println!("part 2: {}", solve2(&numbers, &boards));
}

#[cfg(test)]
mod tests {
    use super::*;

    const NUMBERS: &[i32] = &[
        7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19, 3,
        26, 1,
    ];
    const BOARDS: &[&[&[i32]]] = &[
        &[
            &[22, 13, 17, 11, 0],
            &[8, 2, 23, 4, 24],
            &[21, 9, 14, 16, 7],
            &[6, 10, 3, 18, 5],
            &[1, 12, 20, 15, 19],
        ],
        &[
            &[3, 15, 0, 2, 22],
            &[9, 18, 13, 17, 5],
            &[19, 8, 7, 25, 23],
            &[20, 11, 10, 24, 4],
            &[14, 21, 16, 12, 6],
        ],
        &[
            &[14, 21, 17, 24, 4],
            &[10, 16, 15, 9, 19],
            &[18, 8, 23, 26, 20],
            &[22, 11, 13, 6, 5],
            &[2, 0, 12, 3, 7],
        ],
    ];

    fn boards_to_vec() -> Vec<Vec<Vec<i32>>> {
        let mut boards = Vec::new();
        for board_data in BOARDS.iter() {
            let mut board = Vec::new();
            for row in board_data.iter() {
                board.push(row.to_vec());
            }

            boards.push(board);
        }

        boards
    }

    #[test]
    fn test_solve1() {
        let boards = boards_to_vec();
        assert_eq!(solve1(NUMBERS, &boards), 4512);
    }

    #[test]
    fn test_solve2() {
        let boards = boards_to_vec();
        assert_eq!(solve2(NUMBERS, &boards), 1924);
    }
}
