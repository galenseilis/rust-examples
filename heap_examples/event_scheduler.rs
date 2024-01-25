use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, PartialEq, PartialOrd)]
struct Event {
    time: f64,
}

impl Eq for Event {}

// Implement Ord trait for Event
impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.partial_cmp(&other.time).unwrap_or(Ordering::Equal)
    }
}

impl Event {
    fn new(time: f64) -> Self {
        Self { time }
    }

    fn execute(&self) {
        println!("Event executed at time: {}", self.time);
    }
}

struct Environment {
    event_queue: BinaryHeap<Event>,
    clock: f64,
}

impl Environment {
    fn new() -> Self {
        Self {
            event_queue: BinaryHeap::new(),
            clock: 0.0,
        }
    }

    fn schedule_event(&mut self, event: Event) {
        self.event_queue.push(event);
    }

    fn run(&mut self, end_time: f64) {
        while let Some(current_event) = self.event_queue.pop() {
            if current_event.time < end_time {
                self.clock = current_event.time;
                current_event.execute();
            } else {
                self.clock = end_time;
                break;
            }
        }
    }

    fn now(&self) -> f64 {
        self.clock
    }
}

fn main() {
    let mut env = Environment::new();

    // Schedule events
    env.schedule_event(Event::new(5.0));
    env.schedule_event(Event::new(2.0));
    env.schedule_event(Event::new(8.0));

    // Run the simulation until time 10.0
    env.run(10.0);

    // Print the current time after simulation
    println!("Simulation ended at time: {}", env.now());
}

