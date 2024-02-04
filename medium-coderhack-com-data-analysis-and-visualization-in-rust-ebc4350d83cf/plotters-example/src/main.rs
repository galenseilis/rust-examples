extern crate plotters;

use plotters::prelude::*;

fn main() {
    // Define data points
    let mut data = Vec::new();
    data.push((1., 2.));
    data.push((2., 3.));
    data.push((3., 1.));

    // Set up the chart
    let root = BitMapBackend::new("example_plot.png", (1200, 800))
        .into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Example Plot", ("sans-serif", 20))
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0.0..4.0, 0.0..4.0)
        .unwrap();

    // Draw the line chart
    chart.configure_mesh().draw().unwrap();

    chart.draw_series(LineSeries::new(data, &BLACK)).unwrap();
}

