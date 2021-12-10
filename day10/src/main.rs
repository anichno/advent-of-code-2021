fn solve1(input: &[&str]) -> i32 {
    let mut score = 0;

    for line in input.iter() {
        let mut char_stack = Vec::new();

        for chr in line.chars() {
            if let '(' | '[' | '{' | '<' = chr {
                char_stack.push(chr);
            } else {
                let opener = char_stack.pop().unwrap();
                let bad_match_score = match chr {
                    ')' if opener != '(' => Some(3),
                    ']' if opener != '[' => Some(57),
                    '}' if opener != '{' => Some(1197),
                    '>' if opener != '<' => Some(25137),
                    _ => None,
                };

                if let Some(bad_match_score) = bad_match_score {
                    score += bad_match_score;
                    break;
                }
            }
        }
    }

    score
}

fn solve2(input: &[&str]) -> u64 {
    let mut line_scores = Vec::new();

    for line in input.iter() {
        let mut char_stack = Vec::new();
        let mut invalid_line = false;

        for chr in line.chars() {
            if let '(' | '[' | '{' | '<' = chr {
                char_stack.push(chr);
            } else {
                let opener = char_stack.pop().unwrap();

                invalid_line = match chr {
                    ')' if opener != '(' => true,
                    ']' if opener != '[' => true,
                    '}' if opener != '{' => true,
                    '>' if opener != '<' => true,
                    _ => false,
                };

                if invalid_line {
                    break;
                }
            }
        }

        if !invalid_line {
            let mut line_score = 0;
            for chr in char_stack.iter().rev() {
                match chr {
                    '(' => line_score = line_score * 5 + 1,
                    '[' => line_score = line_score * 5 + 2,
                    '{' => line_score = line_score * 5 + 3,
                    '<' => line_score = line_score * 5 + 4,
                    _ => (),
                }
            }

            line_scores.push(line_score);
        }
    }

    line_scores.sort_unstable();

    line_scores[line_scores.len() / 2]
}

fn main() {
    let input: Vec<&str> = include_str!("input.txt").trim().lines().collect();

    println!("part 1: {}", solve1(&input));
    println!("part 2: {}", solve2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[&str] = &[
        "[({(<(())[]>[[{[]{<()<>>",
        "[(()[<>])]({[<{<<[]>>(",
        "{([(<{}[<>[]}>{[]{[(<()>",
        "(((({<>}<{<{<>}{[]{[]{}",
        "[[<[([]))<([[{}[[()]]]",
        "[{[{({}]{}}([{[{{{}}([]",
        "{<[[]]>}<{[{[{[]{()[[[]",
        "[<(<(<(<{}))><([]([]()",
        "<{([([[(<>()){}]>(<<{{",
        "<{([{{}}[<[[[<>{}]]]>[]]",
    ];

    #[test]
    fn test_solve1() {
        assert_eq!(solve1(&INPUT), 26397);
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve2(&INPUT), 288957);
    }
}
