use std::env;
mod agent;
mod plot;
mod simulation_loop;
mod stats;
use crate::simulation_loop::GenerationStats;
use crate::simulation_loop::SimParams;

// mod simulation_loop; // Commenting out the old import
// it can run a baseline model with standard natural selection, and then we can add a "social culling" mechanism to see how it affects the population over time.
// use crate::simulation_loop::SimParams; // Commenting out the old import
fn main() {
    let args: Vec<String> = env::args().collect();

    // Helper to get value after a flag
    fn get_flag_value(args: &[String], flag: &str) -> Option<f64> {
        args.iter()
            .position(|a| a == flag)
            .and_then(|i| args.get(i + 1))
            .and_then(|v| v.parse().ok())
    }
    // Default parameters
    let default_params = SimParams {
        selection_exponent: 1.5,
        cull_threshold: 0.5,
        envy_coefficient: 0.2,
        intel_weight: 1.0,
        conformity_coefficient: 0.0,
    };
    // ... argument parsing and simulation logic ...

    let params = SimParams {
        selection_exponent: get_flag_value(&args, "-s")
            .unwrap_or(default_params.selection_exponent),
        cull_threshold: get_flag_value(&args, "-c").unwrap_or(default_params.cull_threshold),
        envy_coefficient: get_flag_value(&args, "-e").unwrap_or(default_params.envy_coefficient),
        intel_weight: get_flag_value(&args, "-i").unwrap_or(default_params.intel_weight),
        conformity_coefficient: get_flag_value(&args, "-f")
            .unwrap_or(default_params.conformity_coefficient),
    };

    let generations = args
        .iter()
        .position(|a| a == "-g")
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(50);

    let pop_size = args
        .iter()
        .position(|a| a == "-n")
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(100);

    // Baseline: social culling OFF
    let mut params_baseline = params.clone();
    params_baseline.envy_coefficient = 0.0;
    params_baseline.conformity_coefficient = 0.0;
    let baseline_stats = simulation_loop::run_simulation(generations, pop_size, &params_baseline);
    let social_stats = simulation_loop::run_simulation(generations, pop_size, &params);

    // Prepare vectors for each metric for plotting
    let avg_intel_baseline: Vec<f64> = baseline_stats.iter().map(|s| s.avg_intelligence).collect();
    let stddev_baseline: Vec<f64> = baseline_stats
        .iter()
        .map(|s| s.stddev_intelligence)
        .collect();
    let avg_latent_baseline: Vec<f64> = baseline_stats
        .iter()
        .map(|s| s.avg_latent_fitness)
        .collect();

    let avg_intel_social: Vec<f64> = social_stats.iter().map(|s| s.avg_intelligence).collect();
    let stddev_social: Vec<f64> = social_stats.iter().map(|s| s.stddev_intelligence).collect();
    let avg_stealth_social: Vec<f64> = social_stats.iter().map(|s| s.avg_social_stealth).collect();
    let avg_detection_social: Vec<f64> = social_stats.iter().map(|s| s.avg_detection).collect();
    let avg_latent_social: Vec<f64> = social_stats.iter().map(|s| s.avg_latent_fitness).collect();
    let avg_conflict_social: Vec<f64> = social_stats
        .iter()
        .map(|s| s.avg_conflict_penalty)
        .collect();

    let rows = 10;
    println!("\nBaseline (no social culling):");
    println!("gen,avg_intel,stddev_intel");
    let mut printed_last = false;
    for i in 0..=rows {
        let generation = (generations as f64 * i as f64 / (rows as f64 + 1.0)).round() as usize;
        if generation >= generations {
            continue;
        }
        let avg = avg_intel_baseline.get(generation).copied().unwrap_or(0.0);
        let stddev = stddev_baseline.get(generation).copied().unwrap_or(0.0);
        println!("{}, {:.2}, {:.4}", generation, avg, stddev);
        if generation == generations - 1 {
            printed_last = true;
        }
    }
    if !printed_last {
        let generation = generations - 1;
        let avg = avg_intel_baseline.get(generation).copied().unwrap_or(0.0);
        let stddev = stddev_baseline.get(generation).copied().unwrap_or(0.0);
        println!("{}, {:.2}, {:.4}", generation, avg, stddev);
    }

    println!("\nWith Social Game:");
    println!("gen,avg_intel,stddev_intel,avg_stealth,avg_detection,avg_latent,avg_conflict");
    printed_last = false;
    for i in 0..=rows {
        let generation = (generations as f64 * i as f64 / (rows as f64 + 1.0)).round() as usize;
        if generation >= generations {
            continue;
        }
        let avg = avg_intel_social.get(generation).copied().unwrap_or(0.0);
        let stddev = stddev_social.get(generation).copied().unwrap_or(0.0);
        let stealth = avg_stealth_social.get(generation).copied().unwrap_or(0.0);
        let detection = avg_detection_social.get(generation).copied().unwrap_or(0.0);
        let latent = avg_latent_social.get(generation).copied().unwrap_or(0.0);
        let conflict = avg_conflict_social.get(generation).copied().unwrap_or(0.0);
        println!(
            "{}, {:.2}, {:.4}, {:.4}, {:.4}, {:.4}, {:.4}",
            generation, avg, stddev, stealth, detection, latent, conflict
        );
        if generation == generations - 1 {
            printed_last = true;
        }
    }
    if !printed_last {
        let generation = generations - 1;
        let avg = avg_intel_social.get(generation).copied().unwrap_or(0.0);
        let stddev = stddev_social.get(generation).copied().unwrap_or(0.0);
        let stealth = avg_stealth_social.get(generation).copied().unwrap_or(0.0);
        let detection = avg_detection_social.get(generation).copied().unwrap_or(0.0);
        let latent = avg_latent_social.get(generation).copied().unwrap_or(0.0);
        let conflict = avg_conflict_social.get(generation).copied().unwrap_or(0.0);
        println!(
            "{}, {:.2}, {:.4}, {:.4}, {:.4}, {:.4}, {:.4}",
            generation, avg, stddev, stealth, detection, latent, conflict
        );
    }
}
