use std::collections::{HashMap, VecDeque};
use num::integer::lcm;
advent_of_code::solution!(20);

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Pulse {
    High,
    Low,
}

trait State {
    fn get_state(&self) -> u32;
}

trait ReceivePulse<'a> {
    fn receive_pulse(&mut self, pulse: Pulse, from: &'a str) -> Option<(Pulse, &Vec<&'a str>)>;
}

#[derive(Debug)]
struct FlipFlopModule<'a> {
    state: bool,
    // on / off
    outputs: Vec<&'a str>,
}

impl<'a> FlipFlopModule<'a> {
    fn new(outputs: Vec<&'a str>) -> Self {
        Self {
            state: false,
            outputs,
        }
    }
}

impl<'a> State for FlipFlopModule<'a> {
    fn get_state(&self) -> u32 {
        todo!()
    }
}

impl<'a> ReceivePulse<'a> for FlipFlopModule<'a> {
    fn receive_pulse(&mut self, pulse: Pulse, _: &'a str) -> Option<(Pulse, &Vec<&'a str>)> {
        if pulse == Pulse::High {
            return None;
        }

        self.state = !self.state;

        match self.state {
            true => Some((Pulse::High, &self.outputs)),
            false => Some((Pulse::Low, &self.outputs))
        }
    }
}

#[derive(Debug)]
struct ConjunctionModule<'a> {
    most_recent: HashMap<&'a str, Pulse>,
    outputs: Vec<&'a str>,
}

impl<'a> ConjunctionModule<'a> {
    fn new(outputs: Vec<&'a str>) -> Self {
        Self {
            most_recent: HashMap::new(),
            outputs,
        }
    }

    fn add_sender(&mut self, sender: &'a str) {
        self.most_recent.insert(sender, Pulse::Low);
    }
}

impl<'a> State for ConjunctionModule<'a> {
    fn get_state(&self) -> u32 {
        todo!()
    }
}

impl<'a> ReceivePulse<'a> for ConjunctionModule<'a> {
    fn receive_pulse(&mut self, pulse: Pulse, from: &'a str) -> Option<(Pulse, &Vec<&'a str>)> {
        self.most_recent.insert(from, pulse);

        let sending_pulse =
            match self.most_recent
                .iter()
                .all(|(_, b)| b == &Pulse::High)
            {
                true => Pulse::Low,
                false => Pulse::High
            };

        // println!("{:?} {:?}", self.most_recent, sending_pulse);

        Some((sending_pulse, &self.outputs))
    }
}

#[derive(Debug)]
struct BroadcastModule<'a> {
    outputs: Vec<&'a str>,
}

impl<'a> BroadcastModule<'a> {
    fn new(outputs: Vec<&'a str>) -> Self {
        Self {
            outputs,
        }
    }
}

impl<'a> ReceivePulse<'a> for BroadcastModule<'a> {
    fn receive_pulse(&mut self, pulse: Pulse, _: &'a str) -> Option<(Pulse, &Vec<&'a str>)> {
        Some((pulse, &self.outputs))
    }
}

#[derive(Debug)]
enum Module<'a> {
    FlipFlop(FlipFlopModule<'a>),
    Conjunction(ConjunctionModule<'a>),
    Broadcast(BroadcastModule<'a>),
    Output,
}

fn parse_input(input: &str) -> HashMap<&str, Module> {

    // Connect conjunction modules
    let mut connections: HashMap<&str, Vec<&str>> = HashMap::new();

    let mut modules: HashMap<&str, Module> = input
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.split("->");
            let name_with_type = parts.next().unwrap().trim();
            let outputs: Vec<_> = parts.next().unwrap().split(',').map(str::trim).collect();


            match &name_with_type[..1] {
                "%" => {
                    connections.insert(&name_with_type[1..], outputs.clone());
                    (&name_with_type[1..], Module::FlipFlop(FlipFlopModule::new(outputs)))
                }
                "&" => {
                    connections.insert(&name_with_type[1..], outputs.clone());
                    (&name_with_type[1..], Module::Conjunction(ConjunctionModule::new(outputs)))
                }
                _ => {
                    connections.insert(&name_with_type[1..], outputs.clone());
                    (name_with_type, Module::Broadcast(BroadcastModule::new(outputs)))
                }
            }
        })
        .collect();

    for (from, to) in connections.iter() {
        for to in to.iter() {
            if !modules.contains_key(to) {
                modules.insert(to, Module::Output);
            }

            if let Some(Module::Conjunction(conjunction_module)) = modules.get_mut(to) {
                conjunction_module.add_sender(from);
            }
        }
    }

    modules
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut modules = parse_input(input);

    let mut low_pulse_count: u32 = 0;
    let mut high_pulse_count: u32 = 0;

    let mut pulse_stack: VecDeque<(Pulse, &str, Vec<&str>)> = VecDeque::new();
    for _ in 0..1000 {
        pulse_stack.push_back((Pulse::Low, "button", vec!["broadcaster"]));

        while let Some((pulse, from, modules_names)) = pulse_stack.pop_front() {
            match pulse {
                Pulse::High => high_pulse_count += modules_names.len() as u32,
                Pulse::Low => low_pulse_count += modules_names.len() as u32
            }

            for module_name in modules_names {
                // println!("{from} -{:?}-> {module_name}", pulse);

                let module = modules.get_mut(module_name).unwrap_or_else(|| panic!("No module {module_name}"));

                let outputs = match module {
                    Module::FlipFlop(f) => f.receive_pulse(pulse, from),
                    Module::Conjunction(c) => c.receive_pulse(pulse, from),
                    Module::Broadcast(b) => b.receive_pulse(pulse, from),
                    _ => None
                };

                if let Some((pulse, outputs)) = outputs {
                    pulse_stack.push_back((pulse, module_name, outputs.to_owned()));
                }
            }
        }
    }

    Some(low_pulse_count * high_pulse_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut modules = parse_input(input);

    let mut pulse_stack: VecDeque<(Pulse, &str, Vec<&str>)> = VecDeque::new();

    let mut n = 0u64;

    let mut zz = false;
    let mut cm = false;
    let mut mh = false;
    let mut kd = false;

    let mut values: Vec<u64> = vec![];

    loop {
        n += 1;
        pulse_stack.push_back((Pulse::Low, "button", vec!["broadcaster"]));

        while let Some((pulse, from, modules_names)) = pulse_stack.pop_front() {
            for module_name in modules_names {
                let module = modules.get_mut(module_name).unwrap_or_else(|| panic!("No module {module_name}"));

                let outputs = match module {
                    Module::FlipFlop(f) => f.receive_pulse(pulse, from),
                    Module::Conjunction(c) => c.receive_pulse(pulse, from),
                    Module::Broadcast(b) => b.receive_pulse(pulse, from),
                    _ => {
                        if pulse == Pulse::Low {
                            return Some(n);
                        } else {
                            None
                        }
                    }
                };

                if let Some((pulse, outputs)) = outputs {
                    pulse_stack.push_back((pulse, module_name, outputs.to_owned()));
                }
            }
        }


        if !zz {
            if let Some(Module::Conjunction(c)) = modules.get("zz") {
                if c.most_recent.iter().all(|(_, p)| p.eq(&Pulse::Low)) && n > 1000 {
                    zz = true;
                    values.push(n);
                }
            }
        }
        if !cm {
            if let Some(Module::Conjunction(c)) = modules.get("cm") {
                if c.most_recent.iter().all(|(_, p)| p.eq(&Pulse::Low)) && n > 1000 {
                    cm = true;
                    values.push(n);
                }
            }
        }
        if !mh {
            if let Some(Module::Conjunction(c)) = modules.get("mh") {
                if c.most_recent.iter().all(|(_, p)| p.eq(&Pulse::Low)) && n > 1000 {
                    mh = true;
                    values.push(n);
                }
            }
        }
        if !kd {
            if let Some(Module::Conjunction(c)) = modules.get("kd") {
                if c.most_recent.iter().all(|(_, p)| p.eq(&Pulse::Low)) && n > 1000 {
                    kd = true;
                    values.push(n);
                }
            }
        }
        if zz && cm && mh && kd {
            break;
        }
    }

    Some(
        values
            .into_iter()
            .fold(1u64, lcm),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11_687_500));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }
}
