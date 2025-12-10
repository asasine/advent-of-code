//! Day 10: Factory
//!
//! https://adventofcode.com/2025/day/10

use core::{fmt::Display, num::ParseIntError, str::FromStr};
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use tracing::{instrument, trace, warn};
use z3::Optimize;

#[instrument(skip(input), level = "debug")]
fn part1(input: &str) -> usize {
    let machines: Vec<Machine> = input.lines().map(str::parse).try_collect().unwrap();
    machines
        .iter()
        .map(|machine| {
            machine
                .find_shortest_button_sequence_for_lights()
                .map(|seq| seq.len())
                .unwrap_or(0)
        })
        .sum::<usize>()
}

#[instrument(skip(input), level = "debug")]
fn part2(input: &str) -> usize {
    let machines: Vec<Machine> = input.lines().map(str::parse).try_collect().unwrap();
    machines
        .iter()
        .map(|machine| {
            machine
                .find_shortest_button_sequence_for_joltages()
                .map(|seq| seq.len())
                .unwrap_or(0)
        })
        .sum::<usize>()
}

fn main() {
    solutions::main(part1, part2);
}

/// An individual indicator light, either on or off.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum IndicatorLight {
    /// `#`
    On,

    /// `.`
    Off,
}

impl IndicatorLight {
    /// Toggle the indicator light.
    fn toggle(&mut self) {
        *self = match self {
            IndicatorLight::On => IndicatorLight::Off,
            IndicatorLight::Off => IndicatorLight::On,
        };
    }
}

impl TryFrom<char> for IndicatorLight {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(IndicatorLight::On),
            '.' => Ok(IndicatorLight::Off),
            _ => Err(()),
        }
    }
}

impl Display for IndicatorLight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IndicatorLight::On => write!(f, "#"),
            IndicatorLight::Off => write!(f, "."),
        }
    }
}

/// The state of all indicator lights on a machine.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct IndicatorLights(Vec<IndicatorLight>);

impl IndicatorLights {
    /// Get an all-off indicator lights vector of the given length.
    fn all_off(len: usize) -> Self {
        IndicatorLights(vec![IndicatorLight::Off; len])
    }

    /// Toggle the indicator light at the given index.
    fn toggle(&mut self, index: usize) {
        self.0[index].toggle();
    }

    /// Create an iterator of all possible permutations of indicator lights of the given length.
    fn all_permutations(len: usize) -> impl Iterator<Item = IndicatorLights> {
        (0..(1 << len)).map(move |state| {
            let mut lights = IndicatorLights::all_off(len);
            for i in 0..len {
                if (state & (1 << i)) != 0 {
                    lights.0[i] = IndicatorLight::On;
                }
            }

            lights
        })
    }
}

impl FromStr for IndicatorLights {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().trim_start_matches('[').trim_end_matches(']');
        let lights = s.chars().map(TryFrom::try_from).try_collect()?;
        Ok(IndicatorLights(lights))
    }
}

impl Display for IndicatorLights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for light in &self.0 {
            write!(f, "{}", light)?;
        }

        write!(f, "]")
    }
}

/// A button has two functions:
/// 1. It toggles the indicator lights at specific indices.
/// 2. It increases the joltage of that machine's numeric counter by 1.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Button(Vec<usize>);

impl Button {
    /// Press the button, toggling the indicator lights at the button's indices.
    fn press(&self, lights: &mut IndicatorLights) {
        for &index in &self.0 {
            lights.toggle(index);
        }
    }

    /// Whether this button increments the numeric `counter`.
    fn increments(&self, counter: usize) -> bool {
        self.0.contains(&counter)
    }
}

impl FromStr for Button {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_start_matches('(').trim_end_matches(')');
        let indices = s.split(',').map(str::parse).try_collect()?;
        Ok(Button(indices))
    }
}

impl Display for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        for (i, index) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ",")?;
            }

            write!(f, "{}", index)?;
        }

        write!(f, ")")
    }
}

/// All of a machine's buttons.
#[derive(Debug)]
struct Buttons(Vec<Button>);

impl FromStr for Buttons {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let buttons = s.trim().split(' ').map(str::parse).try_collect()?;
        Ok(Buttons(buttons))
    }
}

impl Display for Buttons {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, button) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }

            write!(f, "{}", button)?;
        }

        Ok(())
    }
}

/// A machine's numeric joltage counters.
#[derive(Debug)]
struct Joltages(Vec<u8>);

impl FromStr for Joltages {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_start_matches('{').trim_end_matches('}');
        let joltages = s.split(',').map(str::parse).try_collect()?;
        Ok(Joltages(joltages))
    }
}

impl Display for Joltages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        for (i, joltage) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ",")?;
            }

            write!(f, "{}", joltage)?;
        }

        write!(f, "}}")
    }
}

/// A machine has indicator lights, buttons, and numeric joltage counters.
///
/// In part 1, the buttons toggle the indicator lights at their indices.
/// In part 2, the buttons increment the joltage of their index's numeric counter.
#[derive(Debug)]
struct Machine {
    /// The required state of the indicator lights to start the machine.
    required_indicator_lights: IndicatorLights,

    /// The buttons on the machine.
    buttons: Buttons,

    /// The required joltage values for each counter to start the machine.
    joltages_requirements: Joltages,
}

impl Machine {
    /// Find the shortest sequence of button presses that will leave the indicator lights in the required state.
    fn find_shortest_button_sequence_for_lights(&self) -> Option<Vec<Button>> {
        // each button press toggles the lights at the button's indices
        // represent each combination of lights as a node in a graph, with edges between nodes for each button press
        // use BFS to find the shortest path from the initial state (all lights off) to the required state

        let mut adjacency_list: HashMap<IndicatorLights, HashSet<(Button, IndicatorLights)>> =
            HashMap::new();

        for button in &self.buttons.0 {
            // for each possible light state, compute the resulting light state after pressing this button
            // there are 2^n possible light states, where n is the number of lights
            let num_lights = self.required_indicator_lights.0.len();
            let all_lights = IndicatorLights::all_permutations(num_lights);
            for current_lights in all_lights {
                let mut new_lights = current_lights.clone();
                button.press(&mut new_lights);
                trace!(%current_lights, %button, %new_lights, "button press");
                adjacency_list
                    .entry(current_lights)
                    .or_default()
                    .insert((button.clone(), new_lights));
            }
        }

        // search for the shortest path from all-off to required lights
        let start_lights = IndicatorLights::all_off(self.required_indicator_lights.0.len());
        let target_lights = &self.required_indicator_lights;
        let mut queue = VecDeque::new();
        let mut visited: HashSet<IndicatorLights> = HashSet::new();
        let mut parents: HashMap<IndicatorLights, (Button, IndicatorLights)> = HashMap::new();
        queue.push_back(start_lights.clone());
        visited.insert(start_lights);
        while let Some(current_lights) = queue.pop_front() {
            if &current_lights == target_lights {
                // found a path to the target lights
                let mut path = Vec::new();
                let mut lights = current_lights;
                while let Some((button, parent_lights)) = parents.get(&lights) {
                    let button_index = self.buttons.0.iter().position(|b| b == button).unwrap();
                    path.push(button_index);
                    lights = parent_lights.clone();
                }
                path.reverse();
                let buttons = Buttons(
                    path.into_iter()
                        .map(|index| self.buttons.0[index].clone())
                        .collect(),
                );
                trace!(machine = %self, %buttons, "found button sequence");
                return Some(buttons.0);
            }

            if let Some(neighbors) = adjacency_list.get(&current_lights) {
                for (button, neighbor_lights) in neighbors {
                    if !visited.contains(neighbor_lights) {
                        visited.insert(neighbor_lights.clone());
                        parents.insert(
                            neighbor_lights.clone(),
                            (button.clone(), current_lights.clone()),
                        );
                        queue.push_back(neighbor_lights.clone());
                    }
                }
            }
        }

        None
    }

    /// Find the shortest sequence of button presses that will set the joltage counters to their required values.
    fn find_shortest_button_sequence_for_joltages(&self) -> Option<Vec<Button>> {
        let optimizer = Optimize::new();

        // variables to count how many times button i was pressed
        let button_presses = (0..self.buttons.0.len())
            .map(|i| z3::ast::Int::new_const(i as u32))
            .collect_vec();

        // constraint: each button press count must be non-negative
        for press_count in &button_presses {
            optimizer.assert(&press_count.ge(0));
        }

        // constraint: the joltage level for each numeric counter must equal the sum of all button presses that
        // increment that counter
        for (joltage_i, &required_joltage) in self.joltages_requirements.0.iter().enumerate() {
            let joltage_level: z3::ast::Int = self
                .buttons
                .0
                .iter()
                .enumerate()
                .filter(|(_, button)| button.increments(joltage_i))
                .map(|(button_i, _)| &button_presses[button_i])
                .sum();

            optimizer.assert(&joltage_level.eq(required_joltage));
        }

        // Goal: minimize the total number of button presses
        let total_presses: z3::ast::Int = button_presses.iter().sum();
        optimizer.minimize(&total_presses);

        if optimizer.check(&[]) == z3::SatResult::Sat
            && let Some(model) = optimizer.get_model()
        {
            // Build the button sequence from the solution
            let mut sequence = Vec::new();
            for (button_i, press_count_var) in button_presses.iter().enumerate() {
                if let Some(press_count) = model.eval(press_count_var, true) {
                    let count = press_count.as_u64().unwrap() as usize;
                    sequence.extend(std::iter::repeat_n(self.buttons.0[button_i].clone(), count));
                }
            }

            trace!(machine = %self, ?sequence, "found solution");
            Some(sequence)
        } else {
            warn!(machine = %self, "no solution exists");
            None
        }
    }
}

impl FromStr for Machine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let buttons_start = s.find('(').ok_or(())?;
        let joltages_start = s.find('{').ok_or(())?;

        let lights = &s[..buttons_start];
        let buttons = &s[buttons_start..joltages_start];
        let joltages = &s[joltages_start..];

        Ok(Machine {
            required_indicator_lights: lights.parse()?,
            buttons: buttons.parse().map_err(|_| ())?,
            joltages_requirements: joltages.parse().map_err(|_| ())?,
        })
    }
}

impl Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.required_indicator_lights, self.buttons, self.joltages_requirements
        )
    }
}

#[cfg(test)]
mod tests {
    use solutions::setup_tracing;

    use super::*;

    #[test]
    fn part1_example() {
        setup_tracing();
        let input = include_str!("../../data/examples/2025/10/1.txt");
        assert_eq!(7, part1(input));
    }

    #[test]
    fn part2_example() {
        let input = include_str!("../../data/examples/2025/10/1.txt");
        assert_eq!(33, part2(input));
    }
}
