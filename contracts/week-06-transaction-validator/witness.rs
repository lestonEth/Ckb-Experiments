use crate::errors::ValidationError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WitnessArgs {
    pub lock: Option<Vec<u8>>,
    pub input_type: Option<Vec<u8>>,
    pub output_type: Option<Vec<u8>>,
}

impl WitnessArgs {
    pub fn empty() -> Self {
        Self {
            lock: None,
            input_type: None,
            output_type: None,
        }
    }

    pub fn new(
        lock: Option<Vec<u8>>,
        input_type: Option<Vec<u8>>,
        output_type: Option<Vec<u8>>,
    ) -> Self {
        Self {
            lock,
            input_type,
            output_type,
        }
    }

    pub fn validate(
        &self,
    ) -> Result<(), ValidationError> {
        if self.lock.is_none()
            && self.input_type.is_none()
            && self.output_type.is_none()
        {
            return Err(
                ValidationError::EmptyWitness
            );
        }

        Ok(())
    }
}
