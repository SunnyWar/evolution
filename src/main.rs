use std::env;
mod agent;
mod plot;
mod simulation_loop;
mod stats;
use crate::simulation_loop::SimParams;

// mod simulation_loop; // Commenting out the old import
// it can run a baseline model with standard natural selection, and then we can add a "social culling" mechanism to see how it affects the population over time.
// use crate::simulation_loop::SimParams; // Commenting out the old import
fn main() {
    // Default parameters
    let default_params = SimParams {
        selection_exponent: 1.5,
        cull_threshold: 0.5,
        envy_coefficient: 0.2,
        intel_weight: 1.0,
        conformity_coefficient: 0.0,
    };

    println!("Evolution Simulation\n");
    let args: Vec<String> = env::args().collect();
    println!(
        "Usage: {} [-s selection_exponent] [-c cull_threshold] [-e envy_coefficient] [-i intel_weight] [-f conformity_coefficient] [-g generations] [-n population_size]",
        args[0]
    );
    println!("Defaults:");
    println!(
        "  -s selection_exponent:      {}",
        default_params.selection_exponent
    );
    println!(
        "  -c cull_threshold:          {}",
        default_params.cull_threshold
    );
    println!(
        "  -e envy_coefficient:        {}",
        default_params.envy_coefficient
    );
    println!(
        "  -i intel_weight:            {}",
        default_params.intel_weight
    );
    println!(
        "  -f conformity_coefficient:  {}",
        default_params.conformity_coefficient
    );
    println!("  -g generations:             50");
    println!("  -n population_size:         100");
    println!("Example: {} -s 2.0 -e 0.3 -g 100 -n 200", args[0]);

    if args.iter().any(|a| a == "-h" || a == "--help") {
        return;
    }

    // Helper to get value after a flag
    fn get_flag_value(args: &[String], flag: &str) -> Option<f64> {
        args.iter()
            .position(|a| a == flag)
            .and_then(|i| args.get(i + 1))
            .and_then(|v| v.parse().ok())
    }

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
    let (avg_intel_baseline, stddev_baseline) =
        simulation_loop::run_simulation(generations, pop_size, &params_baseline);
    let (avg_intel_culling, stddev_culling) =
        simulation_loop::run_simulation(generations, pop_size, &params);
    plot::plot_dual_avg_intel(
        &avg_intel_baseline,
        &avg_intel_culling,
        generations,
        "avg_intel_comparison.png",
    );
    println!("Plot saved to avg_intel_comparison.png (baseline vs social culling)");

    let rows = 10;
    // Output results
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

    println!("\nWith Social Culling:");
    println!("gen,avg_intel,stddev_intel");
    printed_last = false;
    for i in 0..=rows {
        let generation = (generations as f64 * i as f64 / (rows as f64 + 1.0)).round() as usize;
        if generation >= generations {
            continue;
        }
        let avg = avg_intel_culling.get(generation).copied().unwrap_or(0.0);
        let stddev = stddev_culling.get(generation).copied().unwrap_or(0.0);
        println!("{}, {:.2}, {:.4}", generation, avg, stddev);
        if generation == generations - 1 {
            printed_last = true;
        }
    }
    if !printed_last {
        let generation = generations - 1;
        let avg = avg_intel_culling.get(generation).copied().unwrap_or(0.0);
        let stddev = stddev_culling.get(generation).copied().unwrap_or(0.0);
        println!("{}, {:.2}, {:.4}", generation, avg, stddev);
    }
    let generation = generations - 1;
    let avg = avg_intel_culling.get(generation).copied().unwrap_or(0.0);
    let stddev = stddev_culling.get(generation).copied().unwrap_or(0.0);
    println!("{}, {:.2}, {:.4}", generation, avg, stddev);
}
