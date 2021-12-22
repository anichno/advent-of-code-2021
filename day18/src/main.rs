use std::collections::VecDeque;
use std::fmt;
use std::ops::Add;

#[derive(Clone)]
enum Element {
    Value(u64),
    SnailNumber(Box<SnailNumber>),
}

impl Element {
    fn explode_right(&mut self, val: u64) {
        match self {
            Element::Value(cval) => {
                *cval += val;
            }
            Element::SnailNumber(num) => {
                num.right.explode_right(val);
            }
        }
    }

    fn explode_left(&mut self, val: u64) {
        match self {
            Element::Value(cval) => *cval += val,
            Element::SnailNumber(num) => {
                num.left.explode_left(val);
            }
        }
    }
}

#[derive(Clone)]
struct SnailNumber {
    left: Element,
    right: Element,
}

#[derive(Debug)]
enum ReduceResult {
    SplitLeft(u64),
    SplitRight(u64),
    Explode(Option<u64>, Option<u64>),
    Some, // operation concluded
    None,
}

impl SnailNumber {
    fn from_str(input: &str) -> Option<Self> {
        let mut depth = 0;
        for (i, chr) in input.chars().enumerate() {
            if chr == '[' {
                depth += 1;
            } else if chr == ']' {
                depth -= 1;
            } else if chr == ',' && depth == 1 {
                let left = &input[1..i];
                let right = &input[i + 1..input.len() - 1];
                let left = if left.len() == 1 {
                    Element::Value(left.parse().unwrap())
                } else {
                    Element::SnailNumber(Box::new(Self::from_str(left).unwrap()))
                };
                let right = if right.len() == 1 {
                    Element::Value(right.parse().unwrap())
                } else {
                    Element::SnailNumber(Box::new(Self::from_str(right).unwrap()))
                };
                return Some(Self { left, right });
            }
        }

        None
    }

    fn to_string_rec(&self, buffer: &mut String) {
        buffer.push('[');
        match &self.left {
            Element::Value(val) => buffer.push_str(val.to_string().as_str()),
            Element::SnailNumber(num) => num.to_string_rec(buffer),
        }
        buffer.push(',');
        match &self.right {
            Element::Value(val) => buffer.push_str(val.to_string().as_str()),
            Element::SnailNumber(num) => num.to_string_rec(buffer),
        }
        buffer.push(']');
    }

    fn as_string(&self) -> String {
        let mut output = String::new();
        self.to_string_rec(&mut output);
        output
    }

    fn reduce_rec(&mut self, depth: u32, explode_rule: bool) -> ReduceResult {
        if explode_rule && depth >= 4 {
            // explode case
            if let (Element::Value(left), Element::Value(right)) = (&self.left, &self.right) {
                return ReduceResult::Explode(Some(*left), Some(*right));
            }
            panic!("depth too great with non-value elements");
        }

        if !explode_rule {
            // split case
            if let Element::Value(left) = self.left {
                if left >= 10 {
                    return ReduceResult::SplitLeft(left);
                }
            }
        }

        // left recurse
        if let Element::SnailNumber(left) = &mut self.left {
            let result = left.reduce_rec(depth + 1, explode_rule);
            match result {
                ReduceResult::SplitLeft(val) => {
                    let rem = val % 2;
                    left.left = Element::SnailNumber(Box::new(SnailNumber {
                        left: Element::Value(val / 2),
                        right: Element::Value(val / 2 + rem),
                    }));
                    return ReduceResult::Some;
                }
                ReduceResult::SplitRight(val) => {
                    let rem = val % 2;
                    left.right = Element::SnailNumber(Box::new(SnailNumber {
                        left: Element::Value(val / 2),
                        right: Element::Value(val / 2 + rem),
                    }));
                    return ReduceResult::Some;
                }
                ReduceResult::Explode(Some(left), Some(right)) => {
                    self.left = Element::Value(0);
                    self.right.explode_left(right);
                    return ReduceResult::Explode(Some(left), None);
                }
                ReduceResult::Explode(None, Some(right)) => {
                    self.right.explode_left(right);
                    return ReduceResult::Some;
                }
                ReduceResult::None => (),
                _ => return result,
            }
        }

        if !explode_rule {
            if let Element::Value(right) = self.right {
                if right >= 10 {
                    return ReduceResult::SplitRight(right);
                }
            }
        }

        // right recurse
        if let Element::SnailNumber(right) = &mut self.right {
            let result = right.reduce_rec(depth + 1, explode_rule);
            match result {
                ReduceResult::SplitLeft(val) => {
                    let rem = val % 2;
                    right.left = Element::SnailNumber(Box::new(SnailNumber {
                        left: Element::Value(val / 2),
                        right: Element::Value(val / 2 + rem),
                    }));
                    return ReduceResult::Some;
                }
                ReduceResult::SplitRight(val) => {
                    let rem = val % 2;
                    right.right = Element::SnailNumber(Box::new(SnailNumber {
                        left: Element::Value(val / 2),
                        right: Element::Value(val / 2 + rem),
                    }));
                    return ReduceResult::Some;
                }
                ReduceResult::Explode(Some(left), Some(right)) => {
                    self.right = Element::Value(0);
                    self.left.explode_right(left);
                    return ReduceResult::Explode(None, Some(right));
                }
                ReduceResult::Explode(Some(left), None) => {
                    self.left.explode_right(left);
                    return ReduceResult::Some;
                }
                ReduceResult::None => (),
                _ => return result,
            }
        }

        ReduceResult::None
    }

    fn reduce(&mut self) {
        loop {
            let mut result = self.reduce_rec(0, true);
            if let ReduceResult::None = result {
                result = self.reduce_rec(0, false);
                if let ReduceResult::None = result {
                    break;
                }
            }
        }
    }

    fn magnitude(&self) -> u64 {
        let mut tot = 0;
        match &self.left {
            Element::Value(val) => tot += val * 3,
            Element::SnailNumber(node) => tot += node.magnitude() * 3,
        }
        match &self.right {
            Element::Value(val) => tot += val * 2,
            Element::SnailNumber(node) => tot += node.magnitude() * 2,
        }

        tot
    }
}

impl fmt::Display for SnailNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

impl Add for SnailNumber {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut new = Self {
            left: Element::SnailNumber(Box::new(self)),
            right: Element::SnailNumber(Box::new(other)),
        };
        new.reduce();
        new
    }
}

impl Add for &SnailNumber {
    type Output = SnailNumber;

    fn add(self, other: Self) -> SnailNumber {
        let mut new = SnailNumber {
            left: Element::SnailNumber(Box::new(self.clone())),
            right: Element::SnailNumber(Box::new(other.clone())),
        };
        new.reduce();
        new
    }
}

fn solve1(mut input: VecDeque<SnailNumber>) -> u64 {
    let mut sum = input.pop_front().unwrap();
    while !input.is_empty() {
        let right = input.pop_front().unwrap();
        sum = sum + right;
    }

    sum.magnitude()
}

fn solve2(input: &VecDeque<SnailNumber>) -> u64 {
    let mut max = 0;
    for i in 0..input.len() {
        for j in 0..input.len() {
            if i == j {
                continue;
            }
            let tot = (&input[i] + &input[j]).magnitude();
            max = max.max(tot);
        }
    }

    max
}

fn main() {
    let input: VecDeque<SnailNumber> = include_str!("input.txt")
        .trim()
        .lines()
        .map(|l| SnailNumber::from_str(l).unwrap())
        .collect();

    println!("part 1: {}", solve1(input.clone()));
    println!("part 2: {}", solve2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small() {
        let input = SnailNumber::from_str("[[[[4,3],4],4],[7,[[8,4],9]]]").unwrap();
        let result = input + SnailNumber::from_str("[1,1]").unwrap();
        assert_eq!(
            result.to_string().as_str(),
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
        )
    }

    #[test]
    fn test_last_example() {
        let input = &[
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
            "[[[5,[2,8]],4],[5,[[9,9],0]]]",
            "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
            "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
            "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
            "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
            "[[[[5,4],[7,7]],8],[[8,3],8]]",
            "[[9,3],[[9,9],[6,[4,9]]]]",
            "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
            "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
        ];

        let parsed: VecDeque<SnailNumber> = input
            .iter()
            .map(|l| SnailNumber::from_str(*l).unwrap())
            .collect();

        assert_eq!(solve1(parsed), 4140);
    }

    #[test]
    fn test_solve2() {
        let input = &[
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
            "[[[5,[2,8]],4],[5,[[9,9],0]]]",
            "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
            "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
            "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
            "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
            "[[[[5,4],[7,7]],8],[[8,3],8]]",
            "[[9,3],[[9,9],[6,[4,9]]]]",
            "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
            "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
        ];

        let parsed: VecDeque<SnailNumber> = input
            .iter()
            .map(|l| SnailNumber::from_str(*l).unwrap())
            .collect();

        assert_eq!(solve2(&parsed), 3993);
    }
}
