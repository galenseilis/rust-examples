use plotters::prelude::{BitMapBackend, ChartBuilder, IntoDrawingArea, Rectangle};
use plotters::style::{Color, BLUE, WHITE};
use rand::Rng;

const NUM_WEEKS: usize = 100_000;
const MIN_VALUE: f64 = 1.0;
const MAX_VALUE: f64 = 10.0;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // SAMPLING
    let mut rng = rand::thread_rng();
    let mut positions = vec![0.0; NUM_WEEKS];
    let mut current: f64 = 5.0;

    for i in 0..NUM_WEEKS {


        let rand_delta: f64 = if rng.gen_bool(0.5) {1.0} else {-1.0};
        let mut proposal: f64 = current + rand_delta;

        if proposal < MIN_VALUE {
            proposal = MAX_VALUE;
        }

        if proposal > MAX_VALUE {
            proposal = MIN_VALUE;
        }


        let prob_move: f64 = proposal / current;

        let random_threshold: f64 = rng.gen_range(0.0..=1.0);

        let keep_proposal: bool = random_threshold < prob_move;

        current = if keep_proposal {
            proposal
        } else {
            current
        };

        positions[i] = current;
    }

    // Frequency counts of values
    let mut frequencies = vec![0; MAX_VALUE as usize];
    for &position in &positions {
        let index = (position - 1.0) as usize;
        frequencies[index] += 1;
    }

    // Create the bar plot
    let root = BitMapBackend::new("mh_bar_plot.png", (640, 480))
                .into_drawing_area();

    let _ = root.fill(&WHITE);

    // Find maximum frequency
    let max_frequency = *frequencies
                .iter()
                .max()
                .unwrap();

    // Setup canvas
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Good King Markov's Visit Frequencies", 
            ("sans-serif", 20)
        )
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(40)
        .build_cartesian_2d(1..MAX_VALUE as i32, 0..max_frequency)?;

    chart.configure_mesh().draw()?;

    // Plot bars
    for (value, &count) in frequencies.iter().enumerate() {
        let x = value as i32 + 1;
        chart.draw_series(
            std::iter::once(
                Rectangle::new(
                    [(x,0), (x, count)],
                    BLUE.filled(),
                )
            )
        )?;
    }

    let _ = root.present();

    Ok(())
}
