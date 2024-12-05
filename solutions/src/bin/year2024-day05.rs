//! Day 5: Print Queue

/// Satisfied with their search on Ceres, the squadron of scholars suggests subsequently scanning the stationery stacks of
/// sub-basement 17.
///
/// The North Pole printing department is busier than ever this close to Christmas, and while The Historians continue their
/// search of this historically significant facility, an Elf operating a [very familiar printer][1] beckons you over.
///
/// The Elf must recognize you, because they waste no time explaining that the new *sleigh launch safety manual* updates
/// won't print correctly. Failure to update the safety manuals would be dire indeed, so you offer your services.
///
/// Safety protocols clearly indicate that new pages for the safety manuals must be printed in a *very specific order*. The
/// notation `X|Y` means that if both page number `X` and page number `Y` are to be produced as part of an update, page
/// number `X` *must* be printed at some point before page number `Y`.
///
/// The Elf has for you both the *page ordering rules* and the *pages to produce in each update* (your puzzle input), but
/// can't figure out whether each update has the pages in the right order.
///
/// For example:
///
/// ```text
/// 47|53
/// 97|13
/// 97|61
/// 97|47
/// 75|29
/// 61|13
/// 75|53
/// 29|13
/// 97|29
/// 53|29
/// 61|53
/// 97|53
/// 61|29
/// 47|13
/// 75|47
/// 97|75
/// 47|61
/// 75|61
/// 47|29
/// 75|13
/// 53|13
///
/// 75,47,61,53,29
/// 97,61,53,29,13
/// 75,29,13
/// 75,97,47,61,53
/// 61,13,29
/// 97,13,75,29,47
/// ```
///
/// The first section specifies the *page ordering rules*, one per line. The first rule, `47|53`, means that if an update
/// includes both page number 47 and page number 53, then page number 47 *must* be printed at some point before page number
/// 53. (47 doesn't necessarily need to be *immediately* before 53; other pages are allowed to be between them.)
///
/// The second section specifies the page numbers of each *update*. Because most safety manuals are different, the pages
/// needed in the updates are different too. The first update, `75,47,61,53,29`, means that the update consists of page
/// numbers 75, 47, 61, 53, and 29.
///
/// To get the printers going as soon as possible, start by identifying *which updates are already in the right order*.
///
/// In the above example, the first update (`75,47,61,53,29`) is in the right order:
///
/// * `75` is correctly first because there are rules that put each other page after it: `75|47`, `75|61`, `75|53`, and
///   `75|29`.
/// * `47` is correctly second because 75 must be before it (`75|47`) and every other page must be after it according to
///   `47|61`, `47|53`, and `47|29`.
/// * `61` is correctly in the middle because 75 and 47 are before it (`75|61` and `47|61`) and 53 and 29 are after it
///   (`61|53` and `61|29`).
/// * `53` is correctly fourth because it is before page number 29 (`53|29`).
/// * `29` is the only page left and so is correctly last.
///
/// Because the first update does not include some page numbers, the ordering rules involving those missing page numbers are
/// ignored.
///
/// The second and third updates are also in the correct order according to the rules. Like the first update, they also do
/// not include every page number, and so only some of the ordering rules apply - within each update, the ordering rules
/// that involve missing page numbers are not used.
///
/// The fourth update, `75,97,47,61,53`, is *not* in the correct order: it would print 75 before 97, which violates the rule
/// `97|75`.
///
/// The fifth update, `61,13,29`, is also *not* in the correct order, since it breaks the rule `29|13`.
///
/// The last update, `97,13,75,29,47`, is *not* in the correct order due to breaking several rules.
///
/// For some reason, the Elves also need to know the *middle page number* of each update being printed. Because you are
/// currently only printing the correctly-ordered updates, you will need to find the middle page number of each
/// correctly-ordered update. In the above example, the correctly-ordered updates are:
///
/// ```text
/// 75,47,*61*,53,29
/// 97,61,*53*,29,13
/// 75,*29*,13
/// ```
///
/// These have middle page numbers of `61`, `53`, and `29` respectively. Adding these page numbers together gives `*143*`.
///
/// Of course, you'll need to be careful: the actual list of *page ordering rules* is bigger and more complicated than the
/// above example.
///
/// Determine which updates are already in the correct order. *What do you get if you add up the middle page number from
/// those correctly-ordered updates?*
///
/// [1]: https://adventofcode.com/2017/day/1
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
        let mut rulesMatrix = [[false; N]; N];
        for rule in rules {
            rulesMatrix[rule.0][rule.1] = true;
        }

        Self { rules: rulesMatrix }
    }

    fn is_correct_order(&self, update: &Update) -> bool {
        let pages = &update.0;
        for i in 0..pages.len() {
            for j in i + 1..pages.len() {
                if self.rules[pages[j]][pages[i]] {
                    return false;
                }
            }
        }

        true
    }

    /// Corrects the order of the update according to the rules.
    fn correct(&self, update: &Update) -> Update {
        let pages = &update.0;
        let mut corrected = pages.to_vec();
        for i in 0..pages.len() {
            for j in i + 1..pages.len() {
                if self.rules[pages[j]][pages[i]] {
                    corrected.swap(i, j);
                }
            }
        }

        Update(corrected)
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

/// While the Elves get to work printing the correctly-ordered updates, you have a little time to fix the rest of them.
///
/// For each of the *incorrectly-ordered updates*, use the page ordering rules to put the page numbers in the right order.
/// For the above example, here are the three incorrectly-ordered updates and their correct orderings:
///
/// * `75,97,47,61,53` becomes `97,75,*47*,61,53`.
/// * `61,13,29` becomes `61,*29*,13`.
/// * `97,13,75,29,47` becomes `97,75,*47*,29,13`.
///
/// After taking *only the incorrectly-ordered updates* and ordering them correctly, their middle page numbers are `47`,
/// `29`, and `47`. Adding these together produces `*123*`.
///
/// Find the updates which are not in the correct order. *What do you get if you add up the middle page numbers after
/// correctly ordering just those updates?*
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

aoc_macro::aoc_main!();

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
