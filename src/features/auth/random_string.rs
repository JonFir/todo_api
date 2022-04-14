use rand::{distributions::Alphanumeric, thread_rng, Rng};

pub fn new(lenght: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(lenght)
        .map(char::from)
        .collect()
}
