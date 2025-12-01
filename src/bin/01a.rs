const INPUT: &str = include_str!("../../input/01.txt");

fn main() {
    let input = INPUT.lines().map(|line| parse_rotation(line.trim()));

    let count = count_zeros(input);

    println!("{count}")
}

fn parse_rotation(input: &str) -> isize {
    let direction = if input.starts_with('L') {
        -1
    } else if input.starts_with('R') {
        1
    } else {
        panic!("rotation must start with L or R")
    };

    let distance: isize = input[1..].parse().unwrap();

    direction * distance
}

fn count_zeros<T: Iterator<Item = isize>>(rotations: T) -> usize {
    let mut dial = 50;
    let mut count = 0;

    for rotation in rotations {
        dial = (dial + rotation).rem_euclid(100);
        if dial == 0 {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_rotation_parses_rotations() {
        assert_eq!(25, parse_rotation("R25"));
        assert_eq!(0, parse_rotation("R0"));
        assert_eq!(99, parse_rotation("R99"));

        assert_eq!(-25, parse_rotation("L25"));
        assert_eq!(-0, parse_rotation("L0"));
        assert_eq!(-99, parse_rotation("L99"));
    }

    #[test]
    fn example_test_case() {
        let input = [-68, -30, 48, -5, 60, -55, -1, -99, 14, -82];
        assert_eq!(3, count_zeros(input.into_iter()));
    }
}
