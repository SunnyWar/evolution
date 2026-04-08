use crate::simulation_loop::SimParams;
use crate::stats::WorldStats;
use rand;

#[derive(Clone)]
pub struct VisibleTraits {
    pub intelligence: f64,     // High individual benefit, high social risk
    pub physical_size: f64,    // Moderate benefit, moderate social risk
    pub appearance_delta: f64, // Zero benefit, high social risk (the "Outlier" tax)
}

#[derive(Clone)]
pub struct Agent {
    pub latent_fitness: f64, // Invisible traits (e.g., immune system efficiency)
    pub traits: VisibleTraits,
    pub fitness_score: f64,
}

impl Agent {
    // Standard evolution: higher intelligence = more resources
    pub fn calculate_fitness(&mut self, world_stats: &WorldStats, params: &SimParams) {
        // 1. Natural Advantage (S-curve for intelligence)
        let k = 2.0; // Reduced steepness of the S-curve
        let x0 = 1.5; // Center of the S-curve
        let sigmoid = 1.0 / (1.0 + (-k * (self.traits.intelligence - x0)).exp());
        let bio_advantage = self.latent_fitness + sigmoid * params.intel_weight;

        // 2. The Social Brake (Your Theory)
        let intel_outlier_factor = (self.traits.intelligence - world_stats.avg_intel).abs();
        let look_outlier_factor = self.traits.appearance_delta;

        let social_penalty = (intel_outlier_factor * params.envy_coefficient)
            + (look_outlier_factor * params.conformity_coefficient);

        // 3. Final Result (clamped to minimum 0.01)
        self.fitness_score = (bio_advantage - social_penalty).max(0.01);
    }
}
