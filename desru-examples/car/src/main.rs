use desru::{Event, EventScheduler};

const CHARGE_DURATION: f64 = 5.0;
const TRIP_DURATION: f64 = 2.0;

struct Car<'a> {
    scheduler: &'a mut EventScheduler,
}

impl<'a> Car<'a> {
    fn new(scheduler: &'a mut EventScheduler) -> Self {
        let mut car = Car { scheduler };
        car.start();
        car
    }

    fn start(&mut self) {
        // Start the car process
        self.scheduler.schedule(Event::new(
            self.scheduler.current_time,
            Some(Box::new(move |scheduler: &mut EventScheduler| {
                let mut car_instance = Car { scheduler };  // Create a car instance with a mutable reference
                car_instance.charge(); // Call the run method to start the car process
                None
            })),
            None,
        ));
    }

    fn charge(&mut self) {
        // Car running process
        println!("Start parking and charging at {}", self.scheduler.current_time);

        // Schedule the charge process
        self.scheduler.schedule(Event::new(
            self.scheduler.current_time + CHARGE_DURATION,
            Some(Box::new(move |scheduler: &mut EventScheduler| {
                let mut car_instance = Car { scheduler };
                car_instance.drive(); // After charging, start driving
                None
            })),
            None,
        ));
    }

    fn drive(&mut self) {
        // Start driving process
        println!("Start driving at {}", self.scheduler.current_time);

        // Schedule the next run cycle (parking and charging) after the trip duration
        self.scheduler.schedule(Event::new(
            self.scheduler.current_time + TRIP_DURATION,
            Some(Box::new(move |scheduler: &mut EventScheduler| {
                let mut car_instance = Car { scheduler };
                car_instance.charge(); // Repeat the cycle
                None
            })),
            None,
        ));
    }
}

fn main() {
    // Initialize the event scheduler
    let mut scheduler = EventScheduler::new();

    // Create a car instance
    let _car = Car::new(&mut scheduler);

    // Run the scheduler for a max time of 15 units
    scheduler.run_until_max_time(15.0);
}
