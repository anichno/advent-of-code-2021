use std::ops::RangeInclusive;

fn solve1(xrange: &RangeInclusive<i32>, yrange: &RangeInclusive<i32>) -> i32 {
    let mut max_y = 0;
    for xv in (1..*xrange.end()).rev() {
        for yv in 0..200 {
            let mut x_velocity = xv;
            let mut y_velocity = yv;
            let mut cur_x = 0;
            let mut cur_y = 0;
            let mut max_y_pos = 0;

            loop {
                if (x_velocity == 0 && cur_x < *xrange.start())
                    || cur_y < *yrange.start()
                    || cur_x > *xrange.end()
                {
                    break;
                }
                cur_x += x_velocity;
                if x_velocity > 0 {
                    x_velocity -= 1;
                }
                cur_y += y_velocity;
                y_velocity -= 1;

                max_y_pos = max_y_pos.max(cur_y);

                if xrange.contains(&cur_x) && yrange.contains(&cur_y) {
                    max_y = max_y.max(max_y_pos);
                    break;
                }
            }
        }
    }

    max_y
}

fn solve2(xrange: &RangeInclusive<i32>, yrange: &RangeInclusive<i32>) -> i32 {
    let mut valid_velocity = 0;
    for xv in 1..=*xrange.end() {
        for yv in -200..200 {
            let mut x_velocity = xv;
            let mut y_velocity = yv;
            let mut cur_x = 0;
            let mut cur_y = 0;

            loop {
                if (x_velocity == 0 && cur_x < *xrange.start())
                    || cur_y < *yrange.start()
                    || cur_x > *xrange.end()
                {
                    break;
                }
                cur_x += x_velocity;
                if x_velocity > 0 {
                    x_velocity -= 1;
                }
                cur_y += y_velocity;
                y_velocity -= 1;

                if xrange.contains(&cur_x) && yrange.contains(&cur_y) {
                    valid_velocity += 1;
                    break;
                }
            }
        }
    }

    valid_velocity
}

fn main() {
    let (left, right) = include_str!("input.txt")
        .trim()
        .split_once(": ")
        .unwrap()
        .1
        .split_once(", ")
        .unwrap();
    let (xleft, xright) = left[2..].split_once("..").unwrap();
    let (yleft, yright) = right[2..].split_once("..").unwrap();
    let xrange: RangeInclusive<i32> =
        RangeInclusive::new(xleft.parse().unwrap(), xright.parse().unwrap());
    let yrange: RangeInclusive<i32> =
        RangeInclusive::new(yleft.parse().unwrap(), yright.parse().unwrap());

    println!("part 1: {}", solve1(&xrange, &yrange));
    println!("part 2: {}", solve2(&xrange, &yrange));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::RangeInclusive;

    const INPUT: (RangeInclusive<i32>, RangeInclusive<i32>) = (20..=30, -10..=-5);

    #[test]
    fn test_solve1() {
        assert_eq!(solve1(&INPUT.0, &INPUT.1), 45);
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve2(&INPUT.0, &INPUT.1), 112);
    }
}
