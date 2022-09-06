use super::{ConstructionError, ExtendedId, Id, StandardId, SFF_MASK};

/// This trait allows to use the same structure for the CAN Identifier as in the embedded-hal crate.
pub trait IntoCanId {
    fn to_can_id(&self) -> Result<Id, ConstructionError>;
}

impl IntoCanId for u32 {
    fn to_can_id(&self) -> Result<Id, ConstructionError> {
        if *self > SFF_MASK {
            ExtendedId::new(*self).map(Id::Extended)
        } else {
            StandardId::new(*self as u16).map(Id::Standard)
        }
        .ok_or(ConstructionError::IDTooLarge)
    }
}

impl IntoCanId for Id {
    fn to_can_id(&self) -> Result<Id, ConstructionError> {
        Ok(*self)
    }
}
