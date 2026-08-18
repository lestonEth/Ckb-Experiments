#![no_std]

/// Counter contract errors.
#[derive(Debug, PartialEq, Eq)]
pub enum CounterError {
    InvalidDataLength,
    InvalidVersion,
    InvalidTransition,
    CounterOverflow,
}

/// Version of the counter data format.
pub const COUNTER_VERSION: u8 = 1;

/// Counter data stored inside a Cell.
///
/// Layout:
///
/// Byte 0:
///     version
///
/// Bytes 1-8:
///     counter (u64 little-endian)
///
/// Total:
///     9 bytes
#[derive(Debug, PartialEq, Eq)]
pub struct CounterData {
    pub version: u8,
    pub counter: u64,
}

impl CounterData {
    pub const SIZE: usize = 9;

    /// Creates a new counter.
    pub fn new(counter: u64) -> Self {
        Self {
            version: COUNTER_VERSION,
            counter,
        }
    }

    /// Decode Cell data.
    pub fn from_bytes(data: &[u8]) -> Result<Self, CounterError> {
        if data.len() != Self::SIZE {
            return Err(CounterError::InvalidDataLength);
        }

        let version = data[0];

        if version != COUNTER_VERSION {
            return Err(CounterError::InvalidVersion);
        }

        let mut counter_bytes = [0u8; 8];

        counter_bytes.copy_from_slice(&data[1..9]);

        let counter = u64::from_le_bytes(counter_bytes);

        Ok(Self {
            version,
            counter,
        })
    }

    /// Encode counter data for storage in a Cell.
    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        let mut data = [0u8; Self::SIZE];

        data[0] = self.version;

        data[1..9].copy_from_slice(
            &self.counter.to_le_bytes()
        );

        data
    }
}

/// Validate a counter state transition.
///
/// A valid transition must satisfy:
///
/// output = input + 1
///
/// Examples:
///
/// 0 -> 1   valid
/// 1 -> 2   valid
/// 10 -> 11 valid
///
/// 1 -> 3   invalid
/// 5 -> 4   invalid
pub fn validate_increment(
    input_counter: u64,
    output_counter: u64,
) -> Result<(), CounterError> {
    let expected = input_counter
        .checked_add(1)
        .ok_or(CounterError::CounterOverflow)?;

    if output_counter != expected {
        return Err(CounterError::InvalidTransition);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_counter_with_zero() {
        let counter = CounterData::new(0);

        assert_eq!(counter.version, COUNTER_VERSION);
        assert_eq!(counter.counter, 0);
    }

    #[test]
    fn accepts_one_step_increment() {
        assert_eq!(
            validate_increment(0, 1),
            Ok(())
        );

        assert_eq!(
            validate_increment(1, 2),
            Ok(())
        );

        assert_eq!(
            validate_increment(100, 101),
            Ok(())
        );
    }

    #[test]
    fn rejects_skipped_increment() {
        assert_eq!(
            validate_increment(10, 12),
            Err(CounterError::InvalidTransition)
        );
    }

    #[test]
    fn rejects_decrement() {
        assert_eq!(
            validate_increment(10, 9),
            Err(CounterError::InvalidTransition)
        );
    }

    #[test]
    fn detects_overflow() {
        assert_eq!(
            validate_increment(u64::MAX, 0),
            Err(CounterError::CounterOverflow)
        );
    }

    #[test]
    fn encodes_and_decodes_counter() {
        let original = CounterData::new(42);

        let encoded = original.to_bytes();

        let decoded =
            CounterData::from_bytes(&encoded)
                .expect("counter should decode");

        assert_eq!(decoded, original);
    }

    #[test]
    fn rejects_invalid_data_length() {
        let data = [1u8, 2u8];

        assert_eq!(
            CounterData::from_bytes(&data),
            Err(CounterError::InvalidDataLength)
        );
    }

    #[test]
    fn rejects_invalid_version() {
        let mut data = [0u8; CounterData::SIZE];

        data[0] = 99;

        assert_eq!(
            CounterData::from_bytes(&data),
            Err(CounterError::InvalidVersion)
        );
    }
}