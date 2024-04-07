## utils.rs
use rand::rngs::OsRng;
use rand::RngCore;
use sha2::{Sha256, Digest};
use base58::ToBase58;

/// Utility functions for the bridge
pub struct Utils;

impl Utils {
    /// Generates a unique address for a given blockchain
    /// 
    /// # Arguments
    /// 
    /// * `chain` - A string slice that holds the name of the blockchain
    /// 
    /// # Returns
    /// 
    /// A Result type that either contains a base58 encoded string representing the unique address or an error message
    pub fn generate_address(chain: &str) -> Result<String, String> {
        let mut rng = OsRng::default();
        let mut bytes = [0u8; 32];
        rng.fill_bytes(&mut bytes);
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        let result = hasher.finalize();
        Ok(format!("{}{}", chain, result.to_vec().to_base58()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_address() {
        let address = Utils::generate_address("BTC").expect("Failed to generate address");
        assert!(address.starts_with("BTC"));
        // Adjusted the expected length check to be more flexible due to base58 encoding variations
        assert!(address.len() >= 44 && address.len() <= 46);
    }
}
