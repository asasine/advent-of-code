//! Utilities for comparing and ordering values.

use core::cmp::Ordering;

/// Returns the maximum of two values with respect to the specified comparison function.
///
/// Returns the first argument if the comparison determines them to be equal.
///
/// The parameter order is preserved when calling the `compare` function, i.e. `v1` is
/// always passed as the first argument and `v2` as the second.
///
/// This differs from [`core::cmp::max_by`] in that it prefers the left value in the case of equality.
///
/// # Examples
///
/// ```
/// use solutions::cmp;
/// # use core::cmp::Ordering;
/// #[derive(Debug, PartialEq, Eq)]
/// struct Item(u8, u8);
///
/// impl Ord for Item {
///     /// Compare only by the first element.
///     fn cmp(&self, other: &Self) -> Ordering {
///         self.0.cmp(&other.0)
///     }
/// }
///
/// impl PartialOrd for Item {
///     /// Compare only by the first element.
///     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
///        Some(self.cmp(&other))
///    }
/// }
///
/// let result = cmp::max_by_prefer_left(Item(2, 1), Item(2, 2), |a, b| a.cmp(b));
/// assert_eq!(result, Item(2, 1));
/// ```
#[inline]
#[must_use]
pub fn max_by_prefer_left<T, F: FnOnce(&T, &T) -> Ordering>(v1: T, v2: T, compare: F) -> T {
    if compare(&v1, &v2).is_ge() { v1 } else { v2 }
}

/// Returns the minimum of two values with respect to the specified comparison function.
///
/// Returns the second argument if the comparison determines them to be equal.
///
/// The parameter order is preserved when calling the `compare` function, i.e. `v1` is
/// always passed as the first argument and `v2` as the second.
///
/// This differs from [`core::cmp::min_by`] in that it prefers the right value in the case of equality.
///
/// # Examples
///
/// ```
/// use solutions::cmp;
/// # use core::cmp::Ordering;
/// #[derive(Debug, PartialEq, Eq)]
/// struct Item(u8, u8);
///
/// impl Ord for Item {
///     /// Compare only by the first element.
///     fn cmp(&self, other: &Self) -> Ordering {
///         self.0.cmp(&other.0)
///     }
/// }
///
/// impl PartialOrd for Item {
///     /// Compare only by the first element.
///     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
///        Some(self.cmp(&other))
///    }
/// }
///
/// let result = cmp::min_by_prefer_right(Item(2, 1), Item(2, 2), |a, b| a.cmp(b));
/// assert_eq!(result, Item(2, 2));
/// ```
#[inline]
#[must_use]
pub fn min_by_prefer_right<T, F: FnOnce(&T, &T) -> Ordering>(v1: T, v2: T, compare: F) -> T {
    if compare(&v1, &v2).is_lt() { v1 } else { v2 }
}
