//! Random selection operator

use Genome;
use super::SelectOperator;
use rand::Rng;
use rand::distributions::{Range, IndependentSample};

/// TODO
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Random;

impl<G, C, O> SelectOperator<G, C, O> for Random
where
    G: Genome<C>,
    C: Clone + Sized,
    O: Clone + Ord,
{
    /// Select k genomes from a population
    fn select<R: Rng>(
        &self,
        pop_with_fit: &Vec<(O, G)>,
        k: usize,
        rng: &mut R,
    ) -> Vec<G>
    {
        let range = Range::new(0, pop_with_fit.len());
        let mut chosen = vec![];

        for _ in 0..k
        {
            let idx = range.ind_sample(rng);
            chosen.push(pop_with_fit[idx].clone().1);
        }

        chosen
    }
}
