use plotters::prelude::*;
use plotters::style::text_anchor::{HPos, Pos, VPos};
use crate::benchmark::BenchmarkResult;

pub fn generate_plot(results: &[BenchmarkResult]) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("benchmark_results.png", (1200, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_memory = results
        .iter()
        .map(|r| r.memory_used_kb)
        .max()
        .unwrap_or(1000)
        + 100;

    let max_time = results
        .iter()
        .map(|r| r.duration.as_secs_f64())
        .fold(0.0, f64::max)
        * 1.1;

    let mut chart = ChartBuilder::on(&root)
        .caption("Benchmark Results (Time vs Memory)", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .right_y_label_area_size(40)
        .build_cartesian_2d(0..results.len(), 0u64..max_memory)?
        .set_secondary_coord(0..results.len(), 0.0..max_time);

    chart
        .configure_mesh()
        .x_labels(results.len() + 2)
        .y_desc("Memory Used (KB)")
        .y_labels(results.len() + 2)
        .label_style(("sans-serif", 16))
        .disable_x_mesh()
        .draw()?;

    chart
        .configure_secondary_axes()
        .y_desc("Time (s)")
        .y_labels(results.len() + 2)
        .label_style(("sans-serif", 16))
        .draw()?;

    // Słupki pamięci (niebieskie)
    chart
        .draw_series(
            results
                .iter()
                .enumerate()
                .map(|(i, r)| {
                    Rectangle::new(
                        [(i, 0), (i + 1, r.memory_used_kb)],
                        BLUE.mix(0.6).filled(),
                    )
                }),
        )?
        .label("Memory (KB)")
        .legend(|(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], BLUE.mix(0.6).filled()));

    for (i, r) in results.iter().enumerate() {
        root.draw(&Text::new(
            format!("{}", r.memory_used_kb),
            (160 + i as i32 * 220, 50 + 685 - (685.0 * r.memory_used_kb as f64 / max_memory as f64) as i32), // Przesunięcie lekko nad słupek
            ("sans-serif", 16)
                .into_font()
                .color(&BLACK)
                .pos(Pos::new(HPos::Center, VPos::Bottom)),
        ))?;
    }

    // Wykres czasu (czerwony)
    chart
        .draw_secondary_series(LineSeries::new(
            results
                .iter()
                .enumerate()
                .map(|(i, r)| (i, r.duration.as_secs_f64())),
            &RED,
        ))?
        .label("Time (s)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &RED));

    // Etykiety benchmarków (pod osią X)
    for (i, r) in results.iter().enumerate() {
        root.draw(&Text::new(
            r.name.clone(),
            (160 + i as i32 * 220, 780), // 👈 pozycja na ekranie w pikselach
            ("sans-serif", 20)
                .into_font()
                .color(&BLACK)
                .pos(Pos::new(HPos::Center, VPos::Top)),
        ))?;
    }

    // Automatyczna legenda
    chart
        .configure_series_labels()
        .label_font(("sans-serif", 20))
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .draw()?;

    Ok(())
}
