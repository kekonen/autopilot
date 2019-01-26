//! Utility module.
//!
//! Contains small functions not related to the core functionality, but still
//! exposed because they might be useful elsewhere.

/// Caps a value inside a certain range.
#[inline]
pub fn limit_range<T>(min: T, max: T, value: T) -> T
where T: PartialOrd {
    if value > max {
        max
    }
    else if value < min {
        min
    } else {
        value
    }
}
