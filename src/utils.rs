//! Utility functions for constant-time operations.

/// Constant-time memory comparison to prevent timing side-channel attacks.
///
/// Returns `true` if slices are identical, `false` otherwise.
#[inline]
pub fn constant_time_compare(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}
