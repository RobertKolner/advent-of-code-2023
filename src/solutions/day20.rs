use std::collections::{HashMap, VecDeque};
use std::convert::identity;
use nom::bytes::streaming::{is_a, take_while1};
use nom::{AsChar, IResult};
use nom::character::complete::{alpha1, multispace0};
use nom::combinator::opt;
use nom::multi::many1;

const EXAMPLE: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

pub fn solve(input_data: Option<String>, _advanced: bool) -> String {
    let data = input_data.unwrap_or(String::from(EXAMPLE)).trim().to_string();

    let mut modules_map = parse_input(&data)
        .into_iter()
        .fold(HashMap::new(), |mut acc, m| {
            acc.insert(m.id(), m);
            acc
        });
    let mut broadcaster = modules_map
        .remove("broadcaster")
        .expect("No broadcaster module found");

    let mut signal_bus = VecDeque::<Signal>::new();
    for _ in 0..10 {
        let click = broadcaster.receive_signal(SignalType::High, String::from("")).unwrap();
        signal_bus.push_back(click);
        while !signal_bus.is_empty() {
            let signal = signal_bus.pop_front().unwrap();
            println!("Handling signal: {:?}", signal);
            let new_signals = signal.destinations.iter().map(|d| {
                let module = modules_map.get_mut(d).unwrap();
                module.receive_signal(signal.signal_type, signal.from_id.clone())
            }).filter_map(identity).collect::<Vec<_>>();
            signal_bus.extend(new_signals);
        }
    }

    format!("{}", 0)
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum SignalType {
    High,
    Low,
}

#[derive(Debug)]
struct Signal {
    from_id: String,
    destinations: Vec<String>,
    signal_type: SignalType,
}

trait Module: std::fmt::Debug {
    fn id(&self) -> String;
    fn receive_signal(&mut self, signal_type: SignalType, from_id: String) -> Option<Signal>;
}

#[derive(Debug, Clone)]
struct Broadcaster {
    id: String,
    connected_modules: Vec<String>,
}

impl Module for Broadcaster {
    fn id(&self) -> String {
        self.id.clone()
    }
    fn receive_signal(&mut self, signal_type: SignalType, _: String) -> Option<Signal> {
        Some(Signal {
            from_id: self.id.clone(),
            destinations: self.connected_modules.clone(),
            signal_type,
        })
    }
}

#[derive(Debug)]
struct FlipFlop {
    id: String,
    enabled: bool,
    connected_modules: Vec<String>,
}

impl Module for FlipFlop {
    fn id(&self) -> String {
        self.id.clone()
    }
    fn receive_signal(&mut self, signal_type: SignalType, _: String) -> Option<Signal> {
        if signal_type == SignalType::Low {
            self.enabled = !self.enabled;
            return Some(Signal {
                from_id: self.id.clone(),
                destinations: self.connected_modules.clone(),
                signal_type: if self.enabled { SignalType::High } else { SignalType::Low },
            });
        }
        return None;
    }
}

#[derive(Debug)]
struct Conjunction {
    id: String,
    connected_modules: Vec<String>,
    last_signals: HashMap<String, SignalType>
}
impl Module for Conjunction {
    fn id(&self) -> String {
        self.id.clone()
    }
    fn receive_signal(&mut self, signal_type: SignalType, from_id: String) -> Option<Signal> {
        let &previous_signal = self.last_signals.get(&from_id).unwrap_or(&SignalType::Low);
        self.last_signals.entry(from_id).and_modify(|e| *e = signal_type);
        return if self.last_signals.values().all(|&s| s == SignalType::High) {
            self.last_signals.iter_mut().for_each(|(_, s)| *s = SignalType::Low);
            Some(Signal {
                from_id: self.id.clone(),
                destinations: self.connected_modules.clone(),
                signal_type: SignalType::Low,
            })
        } else {
            Some(Signal {
                from_id: self.id.clone(),
                destinations: self.connected_modules.clone(),
                signal_type: SignalType::High,
            })
        }
    }
}

fn parse_input(input: &str) -> Vec<Box<dyn Module>> {
    input
        .lines()
        .map(|l| parse_line(l).unwrap().1)
        .collect()
}

fn parse_line(input: &str) -> IResult<&str, Box<dyn Module>> {
    let (input, flip_flop_mod) = opt(nom::character::complete::char('%'))(input)?;
    let (input, conjunction_mod) = opt(nom::character::complete::char('&'))(input)?;
    let (input, module_name) = take_while1(AsChar::is_alpha)(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = is_a("->")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, destinations) = many1(parse_module)(input)?;
    if flip_flop_mod.is_some() {
        return Ok((input, Box::new(FlipFlop {
            enabled: false,
            id: module_name.to_string(),
            connected_modules: destinations,
        })));
    }
    if conjunction_mod.is_some() {
        return Ok((input, Box::new(Conjunction {
            id: module_name.to_string(),
            connected_modules: destinations,
            last_signals: HashMap::new(),
        })));
    }
    if module_name == "broadcaster" {
        return Ok((input, Box::new(Broadcaster {
            id: module_name.to_string(),
            connected_modules: destinations,
        })));
    }
    panic!("Unknown module: {}", module_name);
}

fn parse_module(input: &str) -> IResult<&str, String> {
    let (input, _) = multispace0(input)?;
    let (input, module_name) = alpha1(input)?;
    let (input, _) = opt(nom::character::complete::char(','))(input)?;
    Ok((input, module_name.to_string()))
}
