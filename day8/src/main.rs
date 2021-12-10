use std::collections::HashMap;

struct Sample {
    signals: Vec<String>,
    output: Vec<String>,
}

fn parse_input(input: &[String]) -> Vec<Sample> {
    let mut parsed = Vec::new();

    for line in input {
        let (left, right) = line.split_once(" | ").unwrap();
        let signals = left.split_whitespace().map(|l| l.to_owned()).collect();
        let output = right.split_whitespace().map(|l| l.to_owned()).collect();
        parsed.push(Sample { signals, output });
    }

    parsed
}

fn solve1(input: &[Sample]) -> usize {
    let mut num_easy = 0;
    for sample in input {
        for digit in &sample.output {
            if let 2 | 4 | 3 | 7 = digit.len() {
                num_easy += 1;
            }
        }
    }

    num_easy
}

fn decode_segments(segments: &str, mapping: &HashMap<char, char>) -> i32 {
    let display: Vec<char> = segments.chars().collect();
    let mut segments: Vec<char> = display.iter().map(|c| *mapping.get(c).unwrap()).collect();
    segments.sort_unstable();
    let segments = String::from_iter(segments);

    match segments.as_str() {
        "abcefg" => 0,
        "cf" => 1,
        "acdeg" => 2,
        "acdfg" => 3,
        "bcdf" => 4,
        "abdfg" => 5,
        "abdefg" => 6,
        "acf" => 7,
        "abcdefg" => 8,
        "abcdfg" => 9,
        _ => panic!("invalid segments: {}", &segments),
    }
}

#[allow(clippy::many_single_char_names)]
fn solve2(input: &[Sample]) -> i32 {
    let mut sum = 0;

    for sample in input {
        let one: Vec<char> = sample
            .signals
            .iter()
            .find(|c| c.len() == 2)
            .unwrap()
            .chars()
            .collect();

        let a = sample
            .signals
            .iter()
            .find(|c| c.len() == 3)
            .unwrap()
            .chars()
            .find(|c| !one.contains(c))
            .unwrap();

        let four: Vec<char> = sample
            .signals
            .iter()
            .find(|c| c.len() == 4)
            .unwrap()
            .chars()
            .collect();

        // 5 segment numbers
        let mut occurences: HashMap<char, i32> = HashMap::new();
        for signals in sample.signals.iter().filter(|c| c.len() == 5) {
            for chr in signals.chars() {
                *occurences.entry(chr).or_default() += 1;
            }
        }

        let b = *occurences
            .iter()
            .find(|(k, v)| **v == 1 && four.contains(*k))
            .unwrap()
            .0;

        let d = *four.iter().find(|c| !one.contains(*c) && **c != b).unwrap();

        let five: Vec<char> = sample
            .signals
            .iter()
            .find(|c| c.len() == 5 && c.chars().any(|f| f == b))
            .unwrap()
            .chars()
            .collect();

        let g = *occurences
            .iter()
            .find(|(k, v)| **v == 3 && **k != d && **k != a)
            .unwrap()
            .0;

        let f = *five
            .iter()
            .find(|c| **c != a && **c != b && **c != d && **c != g)
            .unwrap();

        let c = *one.iter().find(|c| **c != f).unwrap();

        let e = *occurences
            .iter()
            .find(|(k, v)| **v == 1 && **k != b)
            .unwrap()
            .0;

        let mut mapping = HashMap::new();
        mapping.insert(a, 'a');
        mapping.insert(b, 'b');
        mapping.insert(c, 'c');
        mapping.insert(d, 'd');
        mapping.insert(e, 'e');
        mapping.insert(f, 'f');
        mapping.insert(g, 'g');

        let mut displayed_val = 0;
        for (idx, output) in sample.output.iter().rev().enumerate() {
            let number = decode_segments(output, &mapping);
            displayed_val += number * 10_i32.pow(idx as u32);
        }
        sum += displayed_val;
    }

    sum
}

fn main() {
    let input: Vec<String> = include_str!("input.txt")
        .lines()
        .map(|l| l.to_owned())
        .collect();
    let parsed = parse_input(&input);

    println!("part 1: {}", solve1(&parsed));
    println!("part 2: {}", solve2(&parsed));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[&str] = &[
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
        "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
        "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
        "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
        "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
        "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
        "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
        "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
        "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
        "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
    ];

    #[test]
    fn test_solve1() {
        let input: Vec<String> = INPUT.iter().map(|l| String::from(*l)).collect();
        let parsed = parse_input(&input);
        assert_eq!(solve1(&parsed), 26);
    }

    #[test]
    fn test_solve2_ex() {
        let sample = Sample {
            signals: vec![
                "acedgfb".to_owned(),
                "cdfbe".to_owned(),
                "gcdfa".to_owned(),
                "fbcad".to_owned(),
                "dab".to_owned(),
                "cefabd".to_owned(),
                "cdfgeb".to_owned(),
                "eafb".to_owned(),
                "cagedb".to_owned(),
                "ab".to_owned(),
            ],
            output: vec![
                "cdfeb".to_owned(),
                "fcadb".to_owned(),
                "cdfeb".to_owned(),
                "cdbaf".to_owned(),
            ],
        };
        assert_eq!(solve2(&[sample]), 5353);
    }

    #[test]
    fn test_solve2() {
        let input: Vec<String> = INPUT.iter().map(|l| String::from(*l)).collect();
        let parsed = parse_input(&input);
        assert_eq!(solve2(&parsed), 61229);
    }
}
