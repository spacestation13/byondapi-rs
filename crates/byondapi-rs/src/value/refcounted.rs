use super::ByondValue;
use std::ops::{Deref, DerefMut};

/// Refcounted ByondValue, for refs you want rust to handle incrementing and decrementing.
#[derive(Debug)]
pub struct RcByondValue(ByondValue);

impl From<ByondValue> for RcByondValue {
    fn from(mut value: ByondValue) -> Self {
        value.increment_ref();
        RcByondValue(value)
    }
}

impl Deref for RcByondValue {
    type Target = ByondValue;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RcByondValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for RcByondValue {
    fn drop(&mut self) {
        self.0.decrement_ref();
    }
}
