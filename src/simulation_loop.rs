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

pub struct GenerationStats {
    pub avg_intelligence: f64,
    pub stddev_intelligence: f64,
    pub avg_social_stealth: f64,
    pub avg_detection: f64,
    pub avg_latent_fitness: f64,
    pub avg_conflict_penalty: f64,
}

pub fn run_simulation(
    generations: usize,
    pop_size: usize,
    params: &SimParams,
) -> Vec<GenerationStats> {
    let mut rng = SmallRng::seed_from_u64(42);
    let mut population: Vec<Agent> = (0..pop_size)
        .map(|_| Agent {
            latent_fitness: rng.gen::<f64>() * 2.0,
            traits: VisibleTraits {
                intelligence: 0.3 + rng.gen::<f64>() * 1.2,
                physical_size: 0.5 + rng.gen::<f64>() * 2.0,
                appearance_delta: rng.gen::<f64>(),
                social_stealth: rng.gen::<f64>(),
                detection: rng.gen::<f64>(),
            },
            fitness_score: 0.0,
        })
        .collect();

    // RNG already initialized above

    let mut stats_history = Vec::with_capacity(generations);
    for _generation in 0..generations {
        // Calculate world stats in a single pass
        let (sum_intel, sum_phys, sum_app, sum_stealth, sum_detection, sum_latent) = population
            .iter()
            .fold((0.0, 0.0, 0.0, 0.0, 0.0, 0.0), |acc, a| {
                (
                    acc.0 + a.traits.intelligence,
                    acc.1 + a.traits.physical_size,
                    acc.2 + a.traits.appearance_delta,
                    acc.3 + a.traits.social_stealth,
                    acc.4 + a.traits.detection,
                    acc.5 + a.latent_fitness,
                )
            });
        let pop_size_f = population.len() as f64;
        let avg_intel = sum_intel / pop_size_f;
        let avg_stealth = sum_stealth / pop_size_f;
        let avg_detection = sum_detection / pop_size_f;
        let avg_latent = sum_latent / pop_size_f;
        let stats = WorldStats {
            avg_intel,
            avg_physical_size: sum_phys / pop_size_f,
            avg_appearance_delta: sum_app / pop_size_f,
            population_size: population.len(),
        };

        // Calculate standard deviation of intelligence
        let variance = population
            .iter()
            .map(|a| {
                let diff = a.traits.intelligence - avg_intel;
                diff * diff
            })
            .sum::<f64>()
            / pop_size_f;
        let stddev_intel = variance.sqrt();

        // Calculate fitness for each agent and track conflict penalty
        let mut fitnesses = Vec::with_capacity(population.len());
        let mut conflict_penalties = Vec::with_capacity(population.len());
        for i in 0..population.len() {
            let neighbors: Vec<Agent> = population
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, a)| a.clone())
                .collect();
            let mut agent = population[i].clone();
            agent.calculate_fitness(&stats, params, &neighbors);
            fitnesses.push(agent.fitness_score);
            // Calculate average conflict penalty for this agent
            let mut conflict_penalty = 0.0;
            let n = neighbors.len().max(1) as f64;
            for neighbor in &neighbors {
                if agent.traits.social_stealth > 0.5 && neighbor.traits.social_stealth > 0.5 {
                    conflict_penalty +=
                        0.05 * (agent.traits.social_stealth + neighbor.traits.social_stealth);
                }
            }
            conflict_penalty /= n;
            conflict_penalties.push(conflict_penalty);
        }
        for (agent, &fitness) in population.iter_mut().zip(fitnesses.iter()) {
            agent.fitness_score = fitness;
        }

        let avg_conflict_penalty = conflict_penalties.iter().sum::<f64>() / pop_size_f;

        stats_history.push(GenerationStats {
            avg_intelligence: avg_intel,
            stddev_intelligence: stddev_intel,
            avg_social_stealth: avg_stealth,
            avg_detection,
            avg_latent_fitness: avg_latent,
            avg_conflict_penalty,
        });

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
            // Mutate all traits with Gaussian-like noise, clamped to 0.0
            child.traits.intelligence =
                (child.traits.intelligence + (rng.gen::<f64>() - 0.5) * 0.05).max(0.0);
            child.traits.physical_size =
                (child.traits.physical_size + (rng.gen::<f64>() - 0.5) * 0.1).max(0.0);
            child.traits.appearance_delta =
                (child.traits.appearance_delta + (rng.gen::<f64>() - 0.5) * 0.05).max(0.0);
            child.traits.social_stealth =
                (child.traits.social_stealth + (rng.gen::<f64>() - 0.5) * 0.05).max(0.0);
            child.traits.detection =
                (child.traits.detection + (rng.gen::<f64>() - 0.5) * 0.05).max(0.0);
            child.latent_fitness =
                (child.latent_fitness + (rng.gen::<f64>() - 0.5) * 0.05).max(0.0);
            child.fitness_score = 0.0;
            new_population.push(child);
        }
        population = new_population;
    }
    stats_history
}
