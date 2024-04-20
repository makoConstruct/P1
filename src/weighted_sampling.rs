use noisy_float::prelude::n64;

pub trait Weighted<Target> {
    fn weight(&self) -> f64;
    fn transmit(self) -> Target;
}
impl<T> Weighted<T> for (f64, T) {
    fn weight(&self) -> f64 {
        self.0
    }
    fn transmit(self) -> T {
        self.1
    }
}

pub trait Rng {
    fn next(&mut self) -> f64;
}
impl<T> Rng for T
where
    T: rand::Rng,
{
    fn next(&mut self) -> f64 {
        self.gen()
    }
}

/// not particularly efficient afaik, but it was the first kinda efficient way I thought of
pub fn weighted_draws<T>(
    from: &mut Vec<impl Weighted<T>>,
    removing: usize,
    to: &mut Vec<T>,
    rng: &mut impl Rng,
) {
    let mut quota = from.len().min(removing);
    let mut draws = Vec::with_capacity(quota);
    let mut total_weight: f64 = from.iter().map(|t| t.weight()).sum();
    while quota != 0 {
        draws.clear();
        for _ in 0..quota {
            draws.push(n64(total_weight * rng.next()));
        }
        draws.sort();
        let mut acc = 0.0;
        let mut di = 0;
        for d in from.extract_if(|e| {
            if di == draws.len() { return false }
            acc += e.weight();
            if acc > draws[di].into() {
                di += 1;
                quota -= 1;
                total_weight -= e.weight();
                while di < draws.len() && acc > draws[di].into() {
                    di += 1;
                }
                true
            } else {
                false
            }
        }) {
            to.push(d.transmit());
        }
    }
}
pub fn weighted_draws_for_simple_types<T>(
    from: &mut Vec<(f64, T)>,
    removing: usize,
    rng: &mut impl Rng,
) -> Vec<T> {
    let mut ret = Vec::with_capacity(from.len().min(removing));
    weighted_draws(from, removing, &mut ret, rng);
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    #[test]
    fn draw_some() {
        let n: usize = 90;
        let mut ins: Vec<(f64, usize)> = (0..n).map(|i| (i as f64 / n as f64, i)).collect();
        let drawing: usize = 20;
        let drawn = weighted_draws_for_simple_types(
            &mut ins,
            drawing,
            &mut rand::prelude::StdRng::seed_from_u64(80),
        );
        assert_eq!(drawing, drawn.len());
        assert_eq!(ins.len(), n - drawing);
    }
    #[test]
    fn draw_all() {
        let n: usize = 90;
        let mut ins: Vec<(f64, usize)> = (0..n).map(|i| (i as f64 / n as f64, i)).collect();
        let drawing: usize = n;
        let drawn = weighted_draws_for_simple_types(
            &mut ins,
            drawing,
            &mut rand::prelude::StdRng::seed_from_u64(80),
        );
        assert_eq!(drawing, drawn.len());
        //assert that all drawn elements are unique
        assert_eq!(drawn.iter().cloned().collect::<std::collections::HashSet<usize>>().len(), drawing);
        assert_eq!(ins.len(), n - drawing);
    }
}
