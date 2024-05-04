use rand_distr::{Distribution, Exp};
use rand::thread_rng;

#[derive(Debug, PartialEq)]
enum Event {
    BusArrival(f64),
    Boredom(f64),
}

#[derive(Debug, PartialEq)]
enum State {
    Waiting,
    Walking,
}

fn main() {
    // Parameters
    let bus_arrival_rate = 0.1; // Example rate for bus arrivals (bus/min)
    let boredom_rate = 0.05; // Example rate for getting bored (boredom/min)

    // Initialize state
    let mut state = State::Waiting;

    // Initialize time
    let mut time = 0.0;

    // Create a random number generator
    let mut rng = thread_rng();

    loop {
        // Sample times for bus arrival and boredom
        let bus_arrival_dist = Exp::new(bus_arrival_rate).unwrap();
        let bus_arrival_time = bus_arrival_dist.sample(&mut rng);
        let boredom_dist = Exp::new(boredom_rate).unwrap();
        let boredom_time = boredom_dist.sample(&mut rng);

        // Determine the next event: bus arrival or getting bored
        let (event, event_time) = if bus_arrival_time < boredom_time {
            (Event::BusArrival(bus_arrival_time), bus_arrival_time)
        } else {
            (Event::Boredom(boredom_time), boredom_time)
        };

        // Advance time to the next event
        time += event_time;

        // Handle the event
        match event {
            Event::BusArrival(_) => {
                if state == State::Waiting {
                    println!("Bus arrived at time {:.2}", time);
                    break;
                }
            }
            Event::Boredom(_) => {
                if state == State::Waiting {
                    println!("Got bored and started walking at time {:.2}", time);
                    state = State::Walking;
                }
            }
        }
    }
}

