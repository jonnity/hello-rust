extern crate rand;
pub mod pokeca_util {
    use rand::{random, thread_rng, Rng};

    pub fn cointos() -> &'static str {
        return if random::<bool>() { "表" } else { "裏" };
    }
    pub fn d6_roll() -> i8 {
        let mut rng = thread_rng();
        return rng.gen_range(1..7);
    }
}
