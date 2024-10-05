use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Clone)]
enum EventType {
    Arrival,
    ServiceStart,
    ServiceEnd,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Event {
    event_type: EventType,
    entity_id: u32,
}

impl Event {
    fn new(event_type: EventType, entity_id: u32) -> Self {
        Event {
            event_type,
            entity_id,
        }
    }
}

// Wrapper for f64 to make it sortable
#[derive(Debug, PartialOrd, PartialEq)]
struct SortableF64(f64);

impl Eq for SortableF64 {}

impl Ord for SortableF64 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ScheduledEvent {
    time: SortableF64,
    event: Event,
}

impl Ord for ScheduledEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        other.time.cmp(&self.time)  // Reverse to simulate min-heap behavior
    }
}

impl PartialOrd for ScheduledEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Scheduler {
    event_queue: BinaryHeap<ScheduledEvent>,
}

impl Scheduler {
    fn new() -> Self {
        Scheduler {
            event_queue: BinaryHeap::new(),
        }
    }

    fn schedule_event(&mut self, event: Event, time: f64) {
        self.event_queue.push(ScheduledEvent { time: SortableF64(time), event });
    }

    fn run(&mut self) {
        while let Some(scheduled_event) = self.event_queue.pop() {
            println!(
                "Processing event {:?} at time {} for entity {}",
                scheduled_event.event.event_type,
                scheduled_event.time.0,
                scheduled_event.event.entity_id
            );
            // Further logic for handling different event types can be added here
        }
    }
}

fn main() {
    let mut scheduler = Scheduler::new();

    scheduler.schedule_event(Event::new(EventType::Arrival, 1), 1.0);
    scheduler.schedule_event(Event::new(EventType::Arrival, 2), 2.0);
    scheduler.schedule_event(Event::new(EventType::Arrival, 3), 3.0);

    scheduler.run();
}

