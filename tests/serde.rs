#![cfg(feature = "serde")]
// Disable this entire file if "serde" is disabled

use color_art::Color;
use thiserror::Error;

#[derive(Error, Debug)]
enum SerdeTestError {
    #[error("Serde JSON Error: {0}")]
    SerdeJSONError(#[from] serde_json::Error),

    #[error("{0}")]
    Custom(&'static str),
}

#[test]
fn test_serde_color_json() -> Result<(), SerdeTestError> {
    let color = Color::random();

    let serial = serde_json::to_string(&color)?;

    let deserial = serde_json::from_str(&serial)?;

    if color == deserial {
        Ok(())
    } else {
        Err(SerdeTestError::Custom(
            "Deserialized color doesn't match serialized color!",
        ))
    }
}
