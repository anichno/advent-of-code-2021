#[derive(Debug)]
enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

fn parse_input(input: &[&str]) -> Vec<Command> {
    let mut parsed = Vec::new();
    for line in input {
        let mut split = line.split_whitespace();
        let dir = split.next().unwrap();
        let num: i32 = split.next().unwrap().parse().unwrap();

        let command = match dir {
            "forward" => Command::Forward(num),
            "down" => Command::Down(num),
            "up" => Command::Up(num),
            _ => panic!("parse error: {}", dir),
        };
        parsed.push(command);
    }

    parsed
}

fn solve1(input: &[Command]) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;

    for command in input {
        match command {
            Command::Forward(val) => horizontal += val,
            Command::Down(val) => depth += val,
            Command::Up(val) => depth -= val,
        }
    }

    horizontal * depth
}

fn solve2(input: &[Command]) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for command in input {
        match command {
            Command::Forward(val) => {
                horizontal += val;
                depth += val * aim;
            }
            Command::Down(val) => aim += val,
            Command::Up(val) => aim -= val,
        }
    }

    horizontal * depth
}

fn main() {
    let input: Vec<&str> = include_str!("input.txt").lines().collect();
    let parsed = parse_input(&input);

    println!("part 1: {}", solve1(&parsed));
    println!("part 2: {}", solve2(&parsed));
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &[&str] = &[
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2",
    ];

    #[test]
    fn test_solve1() {
        let parsed = parse_input(INPUT);
        assert_eq!(solve1(&parsed), 150);
    }

    #[test]
    fn test_solve2() {
        let parsed = parse_input(INPUT);
        assert_eq!(solve2(&parsed), 900);
    }
}
