use crate::simulation_loop::SimParams;
use crate::stats::WorldStats;
use rand;

#[derive(Clone)]
pub struct VisibleTraits {
    pub intelligence: f64,     // High individual benefit, high social risk
    pub physical_size: f64,    // Moderate benefit, moderate social risk
    pub appearance_delta: f64, // Zero benefit, high social risk (the "Outlier" tax)
    pub social_stealth: f64,   // Ability to hide outlier status
    pub detection: f64,        // Ability to see through stealth
}

#[derive(Clone)]
pub struct Agent {
    pub latent_fitness: f64, // Invisible traits (e.g., immune system efficiency)
    pub traits: VisibleTraits,
    pub fitness_score: f64,
}

impl Agent {
    // Standard evolution: higher intelligence = more resources
    pub fn calculate_fitness(
        &mut self,
        world_stats: &WorldStats,
        params: &SimParams,
        neighbors: &[Agent],
    ) {
        // 1. Natural Advantage (S-curve for intelligence)
        let k = 2.0; // Reduced steepness of the S-curve
        let x0 = 1.5; // Center of the S-curve
        let sigmoid = 1.0 / (1.0 + (-k * (self.traits.intelligence - x0)).exp());
        let mut bio_advantage = self.latent_fitness + sigmoid * params.intel_weight;

        // 2. Energy cost for stealth and detection
        let stealth_cost = 0.02 * self.traits.social_stealth;
        let detection_cost = 0.02 * self.traits.detection;
        bio_advantage -= stealth_cost + detection_cost;

        // 3. Social penalty (herd pressure, mitigated by stealth)
        let mut social_penalty = 0.0;
        let mut exposure_penalty = 0.0;
        let mut conflict_penalty = 0.0;

        for neighbor in neighbors {
            // Outlier factor as perceived by this neighbor
            let perceived_outlier = (self.traits.intelligence - world_stats.avg_intel).abs()
                * (1.0 - self.traits.social_stealth);

            // If neighbor's detection > my stealth, I get exposed
            if neighbor.traits.detection > self.traits.social_stealth {
                exposure_penalty += 0.1 * (neighbor.traits.detection - self.traits.social_stealth);
            }

            // If both have high stealth, conflict penalty
            if self.traits.social_stealth > 0.5 && neighbor.traits.social_stealth > 0.5 {
                conflict_penalty +=
                    0.05 * (self.traits.social_stealth + neighbor.traits.social_stealth);
            }

            // Standard social penalty
            social_penalty += (perceived_outlier * params.envy_coefficient)
                + (self.traits.appearance_delta * params.conformity_coefficient);
        }

        // Average penalties over number of neighbors
        let n = neighbors.len().max(1) as f64;
        social_penalty /= n;
        exposure_penalty /= n;
        conflict_penalty /= n;

        // 4. Final fitness
        self.fitness_score =
            (bio_advantage - social_penalty - exposure_penalty - conflict_penalty).max(0.01);
    }
}
