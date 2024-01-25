use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[derive(Debug, PartialEq, PartialOrd)]
struct Event {
    time: f64,
}

impl Eq for Event {}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

struct Environment {
    event_queue: BinaryHeap<Reverse<Event>>,
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
        self.event_queue.push(Reverse(event));
    }

    fn run(&mut self, end_time: f64) {
        while let Some(Reverse(current_event)) = self.event_queue.pop() {
            if current_event.time + f64::EPSILON  < end_time {
                self.clock = current_event.time;
                current_event.execute();
            } else {
                self.clock = end_time;
                break;
            }
        }

		if self.event_queue.is_empty() {
			self.clock = end_time;
		}
    }

    fn now(&self) -> f64 {
        self.clock
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

fn main() {
    let mut env = Environment::new();

    // Schedule events
    env.schedule_event(Event::new(5.0));
    env.schedule_event(Event::new(2.0));
    env.schedule_event(Event::new(4.0));
    env.schedule_event(Event::new(3.0));
    env.schedule_event(Event::new(1.0));
    env.schedule_event(Event::new(0.0));
    env.schedule_event(Event::new(10.0));
    env.schedule_event(Event::new(8.0));
    env.schedule_event(Event::new(7.0));
    env.schedule_event(Event::new(6.0));
   
   // Simulate until max time
    env.run(100.0);

    // Print the current time after simulation
    println!("Simulation ended at time: {}", env.now());
}

