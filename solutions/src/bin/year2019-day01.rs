//! Day 1: The Tyranny of the Rocket Equation
//!
//! https://adventofcode.com/2019/day/1

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| Mass(line.parse().unwrap()))
        .map(|mass| mass.simple_fuel())
        .map(|fuel| fuel.0)
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| Mass(line.parse().unwrap()))
        .map(|mass| mass.recursive_fuel())
        .map(|fuel| fuel.0)
        .sum()
}

aoc_macro::aoc_main!();

#[derive(Debug, PartialEq)]
struct Mass(usize);

impl Mass {
    fn simple_fuel(&self) -> Fuel {
        Fuel((((self.0 as f32) / 3.0).floor() - 2.0).max(0.0) as usize)
    }

    fn recursive_fuel(&self) -> Fuel {
        let fuel = self.simple_fuel();
        if fuel.0 <= 0 {
            return fuel;
        }

        Fuel(fuel.0 + Mass::from(fuel).recursive_fuel().0)
    }
}

#[derive(Debug, PartialEq)]
struct Fuel(usize);

impl From<Fuel> for Mass {
    fn from(fuel: Fuel) -> Self {
        Mass(fuel.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"12
14
1969
100756
"#;

    #[test]
    fn part1_example() {
        assert_eq!(34241, part1(INPUT));
    }

    #[test]
    fn simple_fuel() {
        assert_eq!(Fuel(2), Mass(12).simple_fuel());
        assert_eq!(Fuel(2), Mass(14).simple_fuel());
        assert_eq!(Fuel(654), Mass(1969).simple_fuel());
        assert_eq!(Fuel(33583), Mass(100756).simple_fuel());
    }

    #[test]
    fn recursive_fuel() {
        assert_eq!(Fuel(2), Mass(14).recursive_fuel());
        assert_eq!(Fuel(966), Mass(1969).recursive_fuel());
        assert_eq!(Fuel(50346), Mass(100756).recursive_fuel());
    }
}
