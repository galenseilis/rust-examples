 use desru::{Event, EventScheduler};

 const MAX_TIME: f64 = 15.0;
 const PARK_DURATION: f64 =  5.0;
 const DRIVE_DURATION: f64 =  2.0;

 fn car(scheduler: &mut EventScheduler) {
    park(scheduler);
}

fn park(scheduler: &mut EventScheduler) {
    println!("Start parking at {}", scheduler.current_time);
    let delay = scheduler.current_time + PARK_DURATION;
    let event = Event::new(
        delay,
        Some(Box::new(move |scheduler: &mut EventScheduler| {
            drive(scheduler);
            None
        })),
        None,
    );
    scheduler.schedule(event);
}

fn drive(scheduler: &mut EventScheduler) {
    println!("Start driving at {}", scheduler.current_time);
    let delay = scheduler.current_time + DRIVE_DURATION;
    let event = Event::new(
        delay,
        Some(Box::new(move |scheduler: &mut EventScheduler| {
                park(scheduler); // Return to parking
                None
            })),
        None,
    );
    scheduler.schedule(event);
}


fn main() {
    // Initialize the event scheduler
    let mut scheduler = EventScheduler::new();

    // Start the clock simulation
    car(&mut scheduler);

    // Run the scheduler for a max time
    scheduler.run_until_max_time(MAX_TIME);
}

