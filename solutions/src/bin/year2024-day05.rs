//! Day 5: Print Queue
//!
//! https://adventofcode.com/2024/day/5

use tracing::instrument;

#[instrument(skip(input), level = "debug")]
fn part1(input: &str) -> usize {
    let (rules, updates) = parse_input(input);
    let rules = Rules::<100>::from_rules(&rules);
    updates
        .iter()
        .filter(|update| rules.is_correct_order(update))
        .map(|update| update.middle())
        .sum()
}

fn parse_input(input: &str) -> (Vec<Rule>, Vec<Update>) {
    let mut sections = input.lines().map(str::trim);
    let rules = sections
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(Rule::from_str)
        .collect::<Vec<_>>();

    let updates = sections.map(Update::from_str).collect::<Vec<_>>();
    (rules, updates)
}

#[derive(Debug, Clone, Copy)]
struct Rule(usize, usize);

impl Rule {
    fn from_str(s: &str) -> Self {
        let mut parts = s.split('|').map(str::trim).map(str::parse);
        let a = parts.next().unwrap().unwrap();
        let b = parts.next().unwrap().unwrap();
        Self(a, b)
    }
}

struct Rules<const N: usize> {
    /// `rules[a][b]` is `true` if `a` must be printed before `b`.
    rules: [[bool; N]; N],
}

impl<const N: usize> Rules<N> {
    fn from_rules(rules: &[Rule]) -> Self {
        let mut rules_matrix = [[false; N]; N];
        for rule in rules {
            rules_matrix[rule.0][rule.1] = true;
        }

        Self {
            rules: rules_matrix,
        }
    }

    fn is_correct_order(&self, update: &Update) -> bool {
        update.0.is_sorted_by(|a, b| self.must_print_before(*a, *b))
    }

    /// Corrects the order of the update according to the rules.
    fn correct(&self, update: &Update) -> Update {
        let mut corrected = update.0.clone();
        corrected.sort_by(|a, b| {
            if self.must_print_before(*a, *b) {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        });

        Update(corrected)
    }

    fn must_print_before(&self, a: usize, b: usize) -> bool {
        self.rules[a][b]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Update(Vec<usize>);

impl Update {
    fn from_str(s: &str) -> Self {
        let parts = s.split(',').map(str::trim).map(str::parse);
        let pages = parts.map(Result::unwrap).collect();
        Self(pages)
    }

    fn middle(&self) -> usize {
        self.0[self.0.len() / 2]
    }
}

#[instrument(skip(input), level = "debug")]
fn part2(input: &str) -> usize {
    let (rules, updates) = parse_input(input);
    let rules = Rules::<100>::from_rules(&rules);
    updates
        .iter()
        .filter(|update| !rules.is_correct_order(update))
        .map(|update| rules.correct(update))
        .map(|update| update.middle())
        .sum()
}

fn main() {
    solutions::main(part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_rules() -> Rules<100> {
        Rules::<100>::from_rules(&[
            Rule(47, 53),
            Rule(97, 13),
            Rule(97, 61),
            Rule(97, 47),
            Rule(75, 29),
            Rule(61, 13),
            Rule(75, 53),
            Rule(29, 13),
            Rule(97, 29),
            Rule(53, 29),
            Rule(61, 53),
            Rule(97, 53),
            Rule(61, 29),
            Rule(47, 13),
            Rule(75, 47),
            Rule(97, 75),
            Rule(47, 61),
            Rule(75, 61),
            Rule(47, 29),
            Rule(75, 13),
            Rule(53, 13),
        ])
    }

    fn get_example_updates() -> Vec<Update> {
        vec![
            Update(vec![75, 47, 61, 53, 29]),
            Update(vec![97, 61, 53, 29, 13]),
            Update(vec![75, 29, 13]),
            Update(vec![75, 97, 47, 61, 53]),
            Update(vec![61, 13, 29]),
            Update(vec![97, 13, 75, 29, 47]),
        ]
    }

    #[test]
    fn part1_example() {
        let input = include_str!("../../data/examples/2024/05/1.txt");
        assert_eq!(143, part1(input));
    }

    #[test]
    fn is_correct_order() {
        let rules = get_example_rules();
        let updates = get_example_updates();
        assert!(rules.is_correct_order(&updates[0]));
        assert!(rules.is_correct_order(&updates[1]));
        assert!(rules.is_correct_order(&updates[2]));
        assert!(!rules.is_correct_order(&updates[3]));
        assert!(!rules.is_correct_order(&updates[4]));
        assert!(!rules.is_correct_order(&updates[5]));
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../../data/examples/2024/05/1.txt");
        assert_eq!(123, part2(input));
    }

    #[test]
    fn fix_incorrect_update() {
        let rules = get_example_rules();
        let updates = get_example_updates();
        assert_eq!(Update(vec![97, 75, 47, 61, 53]), rules.correct(&updates[3]));
        assert_eq!(Update(vec![61, 29, 13]), rules.correct(&updates[4]));
        assert_eq!(Update(vec![97, 75, 47, 29, 13]), rules.correct(&updates[5]));
    }
}
