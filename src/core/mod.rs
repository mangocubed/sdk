use rand::distr::Alphanumeric;
use rand::{Rng, rng};

pub mod config;

#[cfg(feature = "identity-client")]
pub mod identity_client;

pub fn generate_random_string(length: u8) -> String {
    rng()
        .sample_iter(&Alphanumeric)
        .take(length as usize)
        .map(char::from)
        .collect()
}
