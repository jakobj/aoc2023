use std::{
    collections::{HashMap, VecDeque},
    fs,
    ops::Not,
};

fn main() {
    let filename = "inputs/20.txt";
    let content =
        fs::read_to_string(filename).expect("Should have been able to read the input file");

    let mut modules = content
        .lines()
        .map(|l| {
            let module = Module::from(l);
            (module.name.clone(), module)
        })
        .collect::<HashMap<String, Module>>();

    let mut input_map = HashMap::new();
    for (source, module) in modules.iter() {
        for destination in module.destinations.iter() {
            input_map
                .entry(destination.to_string())
                .or_insert(Vec::new())
                .push(source.to_string());
        }
    }

    for (name, inputs) in input_map.iter() {
        if let Some(module) = modules.get_mut(name) {
            module.set_inputs(inputs, Level::Low);
        }
    }

    let mut n_events = (0, 0);
    for _ in 0..1000 {
        let (n, _) = press_button(&mut modules, None);
        n_events.0 += n.0;
        n_events.1 += n.1;
    }
    println!("If you multiply the total number of low pulses sent by the total number of high pulses sent you get {}.", n_events.0 * n_events.1);

    // idea: look at module graph, determine which (non-recurrent) nodes need
    // transitions, then figure out periods of those, then find least common
    // multiple of periods
    let periods = [
        ("kk", Level::Low),
        ("sk", Level::Low),
        ("xc", Level::Low),
        ("vt", Level::Low),
    ]
    .into_iter()
    .map(|break_on| {
        for module in modules.values_mut() {
            module.reset();
        }
        let mut n_button_presses: usize = 0;
        loop {
            let (_, done) = press_button(&mut modules, Some(break_on));
            n_button_presses += 1;
            if done {
                break;
            }
        }
        n_button_presses
    })
    .collect::<Vec<usize>>();
    let n_button_presses = find_lcm(&periods);
    println!("The fewest number of button presses required to deliver a single low pulse to the module named rx is {n_button_presses}.");
}

fn press_button(
    modules: &mut HashMap<String, Module>,
    break_on: Option<(&str, Level)>,
) -> ((usize, usize), bool) {
    let mut event_queue = VecDeque::new();
    event_queue.push_back(Event {
        source: String::from("button"),
        destination: String::from("broadcaster"),
        level: Level::Low,
    });
    let mut n_events = (0, 0);
    while let Some(event) = event_queue.pop_front() {
        // println!("  {event:?}");
        if event.level == Level::Low {
            n_events.0 += 1;
        } else {
            n_events.1 += 1;
        }
        if let Some((name, level)) = break_on {
            if event.destination == name && event.level == level {
                return (n_events, true);
            }
        }
        if let Some(module) = modules.get_mut(&event.destination) {
            module.process_event(event, &mut event_queue);
        }
    }
    (n_events, false)
}

#[derive(Clone, Debug)]
struct Module {
    name: String,
    module_type: ModuleType,
    destinations: Vec<String>,
}

impl Module {
    fn process_event(&mut self, event: Event, event_queue: &mut VecDeque<Event>) {
        let out_level = match &mut self.module_type {
            ModuleType::Broadcaster => Some(event.level),
            ModuleType::FlipFlop(ref mut level) => {
                if event.level == Level::Low {
                    *level = !*level;
                    Some(*level)
                } else {
                    None
                }
            }
            ModuleType::Conjunction(ref mut input_levels) => {
                *input_levels.get_mut(&event.source).unwrap() = event.level;
                if input_levels.values().all(|&l| l == Level::High) {
                    Some(Level::Low)
                } else {
                    Some(Level::High)
                }
            }
        };
        if let Some(out_level) = out_level {
            for destination in self.destinations.iter() {
                event_queue.push_back(Event {
                    source: self.name.clone(),
                    destination: destination.to_string(),
                    level: out_level,
                });
            }
        }
    }

    fn set_inputs(&mut self, inputs: &[String], level: Level) {
        if let ModuleType::Conjunction(ref mut levels) = &mut self.module_type {
            for input in inputs.iter() {
                levels.insert(input.to_string(), level);
            }
        }
    }

    fn reset(&mut self) {
        match self.module_type {
            ModuleType::Broadcaster => (),
            ModuleType::FlipFlop(ref mut level) => *level = Level::Low,
            ModuleType::Conjunction(ref mut input_levels) => {
                input_levels.values_mut().map(|l| *l = Level::Low).count();
            }
        }
    }
}

impl From<&str> for Module {
    fn from(s: &str) -> Self {
        let s = s.split(" -> ").collect::<Vec<&str>>();
        let (name, module_type) = if s[0] == "broadcaster" {
            ("broadcaster", ModuleType::Broadcaster)
        } else if &s[0][..1] == "%" {
            (&s[0][1..], ModuleType::FlipFlop(Level::Low))
        } else {
            assert!(&s[0][..1] == "&");
            (&s[0][1..], ModuleType::Conjunction(HashMap::new()))
        };
        let destinations = s[1]
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        Self {
            name: name.to_string(),
            module_type,
            destinations,
        }
    }
}

#[derive(Clone, Debug)]
enum ModuleType {
    Broadcaster,
    FlipFlop(Level),
    Conjunction(HashMap<String, Level>),
}

#[derive(Clone, Debug)]
struct Event {
    source: String,
    destination: String,
    level: Level,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Level {
    Low,
    High,
}

impl Not for Level {
    type Output = Level;

    fn not(self) -> Self::Output {
        match self {
            Self::Low => Self::High,
            Self::High => Self::Low,
        }
    }
}

fn find_lcm(x: &[usize]) -> usize {
    let mut lcm = x[0];
    for item in x.iter().skip(1) {
        lcm = compute_lcm(lcm, *item);
    }
    lcm
}

fn compute_lcm(a: usize, b: usize) -> usize {
    let gcd = compute_gcd(a, b);
    a * (b / gcd)
}

fn compute_gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
