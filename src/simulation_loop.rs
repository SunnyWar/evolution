use crate::agent::{Agent, VisibleTraits};
use crate::stats::WorldStats;
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::Rng;
use rand::SeedableRng;

#[derive(Clone)]
pub struct SimParams {
    pub selection_exponent: f64,
    pub cull_threshold: f64,
    pub envy_coefficient: f64,
    pub intel_weight: f64,
    pub conformity_coefficient: f64,
}

pub fn run_simulation(generations: usize, pop_size: usize, params: &SimParams) -> Vec<f64> {
    let mut rng = SmallRng::seed_from_u64(42);
    let mut population: Vec<Agent> = (0..pop_size)
        .map(|_| Agent {
            latent_fitness: rng.gen::<f64>() * 2.0,
            traits: VisibleTraits {
                intelligence: 0.3 + rng.gen::<f64>() * 1.2, // start low, but wider
                physical_size: 0.5 + rng.gen::<f64>() * 2.0,
                appearance_delta: rng.gen::<f64>(),
            },
            fitness_score: 0.0,
        })
        .collect();

    // RNG already initialized above

    let mut avg_intel_history = Vec::with_capacity(generations);
    for _generation in 0..generations {
        // Calculate world stats
        let stats = WorldStats {
            avg_intel: population
                .iter()
                .map(|a| a.traits.intelligence)
                .sum::<f64>()
                / population.len() as f64,
            avg_physical_size: population
                .iter()
                .map(|a| a.traits.physical_size)
                .sum::<f64>()
                / population.len() as f64,
            avg_appearance_delta: population
                .iter()
                .map(|a| a.traits.appearance_delta)
                .sum::<f64>()
                / population.len() as f64,
            population_size: population.len(),
        };

        // Calculate fitness for each agent
        for agent in &mut population {
            agent.calculate_fitness(&stats, params);
        }

        // Tournament selection: fill new population
        let mut new_population = Vec::with_capacity(pop_size);
        while new_population.len() < pop_size {
            // Pick two random agents for tournament
            let a = population.choose(&mut rng).unwrap();
            let b = population.choose(&mut rng).unwrap();
            let parent = if a.fitness_score >= b.fitness_score {
                a
            } else {
                b
            };
            let mut child = parent.clone();
            child.traits.intelligence += (rng.gen::<f64>() - 0.5) * 0.05;
            child.traits.intelligence = child.traits.intelligence.max(0.0);
            child.traits.physical_size += (rng.gen::<f64>() - 0.5) * 0.1;
            child.traits.appearance_delta += (rng.gen::<f64>() - 0.5) * 0.05;
            child.latent_fitness += (rng.gen::<f64>() - 0.5) * 0.05;
            child.fitness_score = 0.0;
            new_population.push(child);
        }
        population = new_population;

        avg_intel_history.push(stats.avg_intel);
    }
    avg_intel_history
}
