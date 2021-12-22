use std::collections::HashSet;

#[derive(Clone)]
struct CompressedImage {
    enhancement_algorithm: Vec<bool>,
    image: HashSet<(isize, isize)>,
    min_x: isize,
    min_y: isize,
    max_x: isize,
    max_y: isize,
    background: bool,
}

impl CompressedImage {
    fn from_str(input: &str) -> Self {
        let mut input_iter = input.trim().lines();
        let enhancement_algorithm: Vec<bool> = input_iter
            .next()
            .unwrap()
            .chars()
            .map(|c| c == '#')
            .collect();
        let mut image = HashSet::new();
        input_iter.next(); // skip blank
        let mut min_x = isize::MAX;
        let mut min_y = isize::MAX;
        let mut max_x = isize::MIN;
        let mut max_y = isize::MIN;
        for (y, row) in input_iter.enumerate() {
            for (x, col) in row.trim().chars().enumerate() {
                let x = x as isize;
                let y = y as isize;
                min_x = min_x.min(x);
                min_y = min_y.min(y);
                max_x = max_x.max(x);
                max_y = max_y.max(y);
                if col == '#' {
                    image.insert((x, y));
                }
            }
        }

        CompressedImage {
            enhancement_algorithm,
            image,
            min_x,
            min_y,
            max_x,
            max_y,
            background: false,
        }
    }

    fn lookup(&self, x: isize, y: isize) -> usize {
        let mut output = 0;
        for ydiff in -1..=1 {
            for xdiff in -1..=1 {
                let x = x + xdiff;
                let y = y + ydiff;
                let lit = self.image.contains(&(x, y))
                    || (self.background
                        && (x < self.min_x || x > self.max_y || y < self.min_y || y > self.max_y));
                if lit {
                    output |= 1;
                }
                output <<= 1;
            }
        }
        output >> 1
    }

    fn decompress(&mut self, iterations: usize) {
        for _ in 0..iterations {
            let mut turn_on = Vec::new();
            let mut turn_off = Vec::new();
            let mut new_min_x = self.min_x;
            let mut new_min_y = self.min_y;
            let mut new_max_x = self.max_x;
            let mut new_max_y = self.max_y;
            for y in self.min_y - 1..=self.max_y + 1 {
                for x in self.min_x - 1..=self.max_x + 1 {
                    if self.enhancement_algorithm[self.lookup(x, y)] {
                        turn_on.push((x, y));
                        new_min_x = new_min_x.min(x);
                        new_min_y = new_min_y.min(y);
                        new_max_x = new_max_x.max(x);
                        new_max_y = new_max_y.max(y);
                    } else {
                        turn_off.push((x, y));
                    }
                }
            }
            if self.background {
                self.background = self.enhancement_algorithm[0b111111111];
            } else {
                self.background = self.enhancement_algorithm[0];
            }

            for set_on in turn_on {
                self.image.insert(set_on);
            }
            for set_off in turn_off {
                self.image.remove(&set_off);
            }
            self.min_x = new_min_x;
            self.min_y = new_min_y;
            self.max_x = new_max_x;
            self.max_y = new_max_y;
        }
    }

    #[allow(dead_code)]
    fn display(&self) {
        for y in self.min_y - 1..self.max_y + 1 {
            for x in self.min_x - 1..self.max_x + 1 {
                if self.image.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

fn solve1(mut image: CompressedImage) -> usize {
    image.decompress(2);
    image.image.len()
}

fn solve2(mut image: CompressedImage) -> usize {
    image.decompress(50);
    image.image.len()
}

fn main() {
    let input = CompressedImage::from_str(include_str!("input.txt").trim());

    println!("part 1: {}", solve1(input.clone()));
    println!("part 2: {}", solve2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#",

    #..#.
    #....
    ##..#
    ..#..
    ..###"#;

    #[test]
    fn test_lookup() {
        let cimage = CompressedImage::from_str(&INPUT);
        assert_eq!(cimage.lookup(2, 2), 34);
    }

    #[test]
    fn test_solve1() {
        let cimage = CompressedImage::from_str(&INPUT);
        assert_eq!(solve1(cimage), 35);
    }

    #[test]
    fn test_solve2() {
        let cimage = CompressedImage::from_str(&INPUT);
        assert_eq!(solve2(cimage), 3351);
    }
}
