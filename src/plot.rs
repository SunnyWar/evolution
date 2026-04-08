use plotters::prelude::*;

pub fn plot_dual_avg_intel(baseline: &[f64], culling: &[f64], generations: usize, filename: &str) {
    let root = BitMapBackend::new(filename, (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let max_y = baseline
        .iter()
        .chain(culling.iter())
        .cloned()
        .fold(f64::NAN, f64::max)
        .max(0.5);
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Average Intelligence: Baseline vs Social Culling",
            ("sans-serif", 30),
        )
        .margin(40)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0..generations, 0.0..(max_y + 0.5))
        .unwrap();

    chart.configure_mesh().draw().unwrap();
    chart
        .draw_series(LineSeries::new(
            baseline.iter().enumerate().map(|(x, y)| (x, *y)),
            &BLUE,
        ))
        .unwrap()
        .label("Baseline")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
    chart
        .draw_series(LineSeries::new(
            culling.iter().enumerate().map(|(x, y)| (x, *y)),
            &RED,
        ))
        .unwrap()
        .label("Social Culling")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()
        .unwrap();
    root.present().unwrap();
}
