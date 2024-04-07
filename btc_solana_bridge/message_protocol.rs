// message_protocol.rs
use serde::{Serialize, Deserialize};

/// Represents a message for asset transfer across blockchains.
#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    asset: String,
    amount: u64,
    destination: String,
}

impl Message {
    /// Creates a new message for transferring assets.
    ///
    /// # Arguments
    ///
    /// * `asset` - A string slice that holds the name of the asset to transfer.
    /// * `amount` - The amount of the asset to transfer.
    /// * `destination` - A string slice that holds the destination address.
    ///
    /// # Returns
    ///
    /// A new `Message` instance.
    pub fn create_message(asset: String, amount: u64, destination: String) -> Self {
        Message {
            asset,
            amount,
            destination,
        }
    }

    /// Serializes the message into a JSON string.
    ///
    /// # Returns
    ///
    /// A `Result` which is either a string of JSON or a serialization error.
    pub fn serialize(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }

    /// Deserializes a JSON string into a `Message` instance.
    ///
    /// # Arguments
    ///
    /// * `data` - A string slice that holds the JSON data to deserialize.
    ///
    /// # Returns
    ///
    /// A `Result` which is either a `Message` instance or a deserialization error.
    pub fn deserialize(data: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_message() {
        let message = Message::create_message("BTC".to_string(), 100, "SolanaAddress".to_string());
        assert_eq!(message.asset, "BTC");
        assert_eq!(message.amount, 100);
        assert_eq!(message.destination, "SolanaAddress");
    }

    #[test]
    fn test_serialize_deserialize() {
        let message = Message::create_message("BTC".to_string(), 100, "SolanaAddress".to_string());
        let serialized = message.serialize().unwrap();
        let deserialized: Message = Message::deserialize(&serialized).unwrap();

        assert_eq!(deserialized.asset, "BTC");
        assert_eq!(deserialized.amount, 100);
        assert_eq!(deserialized.destination, "SolanaAddress");
    }
}
