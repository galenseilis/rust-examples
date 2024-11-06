use desru::{Event, EventScheduler};

fn clock(scheduler: &mut EventScheduler, name: String, tick: f64) {
    // Function to handle the clock's actions and schedule the next tick
    fn action(scheduler: &mut EventScheduler, name: String, tick: f64) {
        // Print the name of the clock and the current simulation time
        println!("{}: {}", name, scheduler.current_time);

        // Schedule the next tick of the clock
        let next_time = scheduler.current_time + tick;
        let event = Event::new(
            next_time,
            Some(Box::new(move |scheduler: &mut EventScheduler| {
                action(scheduler, name.clone(), tick);
                None
            })),
            None,
        );
        scheduler.schedule(event);
    }

    // Schedule the first event for the clock at time 0
    scheduler.schedule(Event::new(
        0.0,
        Some(Box::new(move |scheduler: &mut EventScheduler| {
            action(scheduler, name.clone(), tick);
            None
        })),
        None,
    ));
}

fn main() {
    // Initialize the event scheduler
    let mut scheduler = EventScheduler::new();

    // Schedule initial clock processes
    clock(&mut scheduler, "fast".to_string(), 0.5);
    clock(&mut scheduler, "slow".to_string(), 1.0);

    // Run the scheduler until the maximum simulation time
    scheduler.run_until_max_time(2.0);
}

