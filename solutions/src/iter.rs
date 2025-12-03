//! Utilities for working with iterators.
//!
//! For the examples in this module, we define an `Item` struct that compares
//! only by one of its fields, but has an additional field for identification:
//!
//! ```
//! use core::cmp::Ordering;
//! #[derive(Debug, PartialEq, Eq)]
//! struct Item(u8, u8);
//!
//! impl Ord for Item {
//!     /// Compares items by the first field only.
//!     fn cmp(&self, other: &Self) -> Ordering {
//!         self.0.cmp(&other.0)
//!     }
//! }
//!
//! impl PartialOrd for Item {
//!     /// Compares items by the first field only.
//!     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//!         Some(self.cmp(&other))
//!     }
//! }
//! ```

use crate::cmp;
use std::cmp::Ordering;

/// An [`Iterator`] blanket implementation that provides extra adaptors and methods.
pub trait IteratorExt: Iterator {
    /// Returns the maximum element of an iterator.
    ///
    /// If several elements are equally maximum, the first element is
    /// returned. If the iterator is empty, [`None`] is returned.
    ///
    /// This differs from [`Iterator::max`] in that it returns the first maximum
    /// instead of the last maximum.
    ///
    /// Note that [`f32`]/[`f64`] doesn't implement [`Ord`] due to NaN being
    /// incomparable. You can work around this by using [`Iterator::reduce`]:
    /// ```
    /// assert_eq!(
    ///     [2.4, f32::NAN, 1.3]
    ///         .into_iter()
    ///         .reduce(f32::max)
    ///         .unwrap_or(0.),
    ///     2.4
    /// );
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// # use core::cmp::Ordering;
    /// # #[derive(Debug, PartialEq, Eq)]
    /// # struct Item(u8, u8);
    /// #
    /// # impl Ord for Item {
    /// #     /// Compares items by the first field only.
    /// #     fn cmp(&self, other: &Self) -> Ordering {
    /// #         self.0.cmp(&other.0)
    /// #     }
    /// # }
    /// #
    /// # impl PartialOrd for Item {
    /// #     /// Compares items by the first field only.
    /// #     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    /// #         Some(self.cmp(&other))
    /// #     }
    /// # }
    /// use solutions::iter::IteratorExt;
    /// let a = [Item(3, 1), Item(2, 2), Item(3, 3)];
    /// assert_eq!(a.into_iter().first_max(), Some(Item(3, 1)));
    /// ```
    #[inline]
    fn first_max(self) -> Option<Self::Item>
    where
        Self: Sized,
        Self::Item: Ord,
    {
        self.first_max_by(Ord::cmp)
    }

    /// Returns the element that gives the maximum value from the
    /// specified function.
    ///
    /// If several elements are equally maximum, the first element is
    /// returned. If the iterator is empty, [`None`] is returned.
    ///
    /// This differs from [`Iterator::max_by_key`] in that it returns the first
    /// maximum instead of the last maximum.
    ///
    /// # Examples
    ///
    /// ```
    /// // max by the first element of the tuple, differentiate by the second element
    /// use solutions::iter::IteratorExt;
    /// let a = [(5, 1), (5, 2), (3, 3)];
    /// assert_eq!(a.into_iter().first_max_by_key(|x| x.0).unwrap(), (5, 1));
    /// ```
    #[inline]
    fn first_max_by_key<B: Ord, F>(self, f: F) -> Option<Self::Item>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> B,
    {
        #[inline]
        fn key<T, B>(mut f: impl FnMut(&T) -> B) -> impl FnMut(T) -> (B, T) {
            move |x| (f(&x), x)
        }

        #[inline]
        fn compare<T, B: Ord>((x_p, _): &(B, T), (y_p, _): &(B, T)) -> Ordering {
            x_p.cmp(y_p)
        }

        let (_, x) = self.map(key(f)).first_max_by(compare)?;
        Some(x)
    }

    /// Returns the element that gives the maximum value with respect to the
    /// specified comparison function.
    ///
    /// If several elements are equally maximum, the first element is
    /// returned. If the iterator is empty, [`None`] is returned.
    ///
    /// This differs from [`Iterator::max_by`] in that it returns the first maximum
    /// instead of the last maximum.
    ///
    /// # Examples
    ///
    /// ```
    /// // max by the first element of the tuple, differentiate by the second element
    /// use solutions::iter::IteratorExt;
    /// let a = [(5, 1), (5, 2), (3, 3)];
    /// assert_eq!(a.into_iter().first_max_by(|x, y| x.0.cmp(&y.0)).unwrap(), (5, 1));
    /// ```
    #[inline]
    fn first_max_by<F>(self, compare: F) -> Option<Self::Item>
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> Ordering,
    {
        #[inline]
        fn fold<T>(mut compare: impl FnMut(&T, &T) -> Ordering) -> impl FnMut(T, T) -> T {
            move |x, y| cmp::max_by_prefer_left(x, y, &mut compare)
        }

        self.reduce(fold(compare))
    }

    /// Returns the minimum element of an iterator.
    ///
    /// If several elements are equally minimum, the last element is
    /// returned. If the iterator is empty, [`None`] is returned.
    ///
    /// This differs from [`Iterator::min`] in that it returns the last minimum
    /// instead of the first minimum.
    ///
    /// Note that [`f32`]/[`f64`] doesn't implement [`Ord`] due to NaN being
    /// incomparable. You can work around this by using [`Iterator::reduce`]:
    /// ```
    /// assert_eq!(
    ///     [2.4, f32::NAN, 1.3]
    ///         .into_iter()
    ///         .reduce(f32::min)
    ///         .unwrap_or(0.),
    ///     1.3
    /// );
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// # use core::cmp::Ordering;
    /// # #[derive(Debug, PartialEq, Eq)]
    /// # struct Item(u8, u8);
    /// #
    /// # impl Ord for Item {
    /// #     /// Compares items by the first field only.
    /// #     fn cmp(&self, other: &Self) -> Ordering {
    /// #         self.0.cmp(&other.0)
    /// #     }
    /// # }
    /// #
    /// # impl PartialOrd for Item {
    /// #     /// Compares items by the first field only.
    /// #     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    /// #         Some(self.cmp(&other))
    /// #     }
    /// # }
    /// use solutions::iter::IteratorExt;
    /// let a = [Item(3, 1), Item(2, 2), Item(2, 3)];
    /// assert_eq!(a.into_iter().last_min(), Some(Item(2, 3)));
    /// ```
    #[inline]
    fn last_min(self) -> Option<Self::Item>
    where
        Self: Sized,
        Self::Item: Ord,
    {
        self.last_min_by(Ord::cmp)
    }

    /// Returns the element that gives the minimum value from the
    /// specified function.
    ///
    /// If several elements are equally minimum, the last element is
    /// returned. If the iterator is empty, [`None`] is returned.
    ///
    /// This differs from [`Iterator::min_by_key`] in that it returns the last
    /// minimum instead of the first minimum.
    ///
    /// # Examples
    ///
    /// ```
    /// // min by the first element of the tuple, differentiate by the second element
    /// use solutions::iter::IteratorExt;
    /// let a = [(5, 1), (3, 2), (3, 3)];
    /// assert_eq!(a.into_iter().last_min_by_key(|x| x.0).unwrap(), (3, 3));
    /// ```
    #[inline]
    fn last_min_by_key<B: Ord, F>(self, f: F) -> Option<Self::Item>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> B,
    {
        #[inline]
        fn key<T, B>(mut f: impl FnMut(&T) -> B) -> impl FnMut(T) -> (B, T) {
            move |x| (f(&x), x)
        }

        #[inline]
        fn compare<T, B: Ord>((x_p, _): &(B, T), (y_p, _): &(B, T)) -> Ordering {
            x_p.cmp(y_p)
        }

        let (_, x) = self.map(key(f)).last_min_by(compare)?;
        Some(x)
    }

    /// Returns the element that gives the minimum value with respect to the
    /// specified comparison function.
    ///
    /// If several elements are equally minimum, the last element is
    /// returned. If the iterator is empty, [`None`] is returned.
    ///
    /// This differs from [`Iterator::min_by`] in that it returns the last minimum
    /// instead of the first minimum.
    ///
    /// # Examples
    ///
    /// ```
    /// // min by the first element of the tuple, differentiate by the second element
    /// use solutions::iter::IteratorExt;
    /// let a = [(5, 1), (3, 2), (3, 3)];
    /// assert_eq!(a.into_iter().last_min_by(|x, y| x.0.cmp(&y.0)).unwrap(), (3, 3));
    /// ```
    #[inline]
    fn last_min_by<F>(self, compare: F) -> Option<Self::Item>
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> Ordering,
    {
        #[inline]
        fn fold<T>(mut compare: impl FnMut(&T, &T) -> Ordering) -> impl FnMut(T, T) -> T {
            move |x, y| cmp::min_by_prefer_right(x, y, &mut compare)
        }

        self.reduce(fold(compare))
    }

    /// Return the position of the maximum element in the iterator.
    ///
    /// If several elements are equally maximum, the position of the
    /// first of them is returned.
    ///
    /// This differs from [`itertools::Itertools::position_max`] in that it
    /// returns the first maximum instead of the last maximum.
    ///
    /// # Examples
    ///
    /// ```
    /// use solutions::iter::IteratorExt;
    ///
    /// let a: [i32; 0] = [];
    /// assert_eq!(a.iter().position_first_max(), None);
    ///
    /// let a = [-3, 0, 1, 5, -10];
    /// assert_eq!(a.iter().position_first_max(), Some(3));
    ///
    /// let a = [1, 1, -1, -1];
    /// assert_eq!(a.iter().position_first_max(), Some(0));
    /// ```
    fn position_first_max(self) -> Option<usize>
    where
        Self: Sized,
        Self::Item: Ord,
    {
        self.enumerate()
            .first_max_by(|x, y| Ord::cmp(&x.1, &y.1))
            .map(|x| x.0)
    }

    /// Return the position of the maximum element in the iterator, as
    /// determined by the specified function.
    ///
    /// If several elements are equally maximum, the position of the
    /// first of them is returned.
    ///
    /// This differs from [`itertools::Itertools::position_max_by_key`] in that
    /// it returns the first maximum instead of the last maximum.
    ///
    /// # Examples
    ///
    /// ```
    /// use solutions::iter::IteratorExt;
    ///
    /// let a: [i32; 0] = [];
    /// assert_eq!(a.iter().position_first_max_by_key(|x| x.abs()), None);
    ///
    /// let a = [-3_i32, 0, 1, 5, -10];
    /// assert_eq!(a.iter().position_first_max_by_key(|x| x.abs()), Some(4));
    ///
    /// let a = [1_i32, 1, -1, -1];
    /// assert_eq!(a.iter().position_first_max_by_key(|x| x.abs()), Some(0));
    /// ```
    fn position_first_max_by_key<K, F>(self, mut key: F) -> Option<usize>
    where
        Self: Sized,
        K: Ord,
        F: FnMut(&Self::Item) -> K,
    {
        self.enumerate()
            .first_max_by(|x, y| Ord::cmp(&key(&x.1), &key(&y.1)))
            .map(|x| x.0)
    }

    /// Return the position of the maximum element in the iterator, as
    /// determined by the specified comparison function.
    ///
    /// If several elements are equally maximum, the position of the
    /// first of them is returned.
    ///
    /// This differs from [`itertools::Itertools::position_max_by`] in that it
    /// returns the first maximum instead of the last maximum.
    ///
    /// # Examples
    ///
    /// ```
    /// use solutions::iter::IteratorExt;
    ///
    /// let a: [i32; 0] = [];
    /// assert_eq!(a.iter().position_first_max_by(|x, y| x.cmp(y)), None);
    ///
    /// let a = [-3_i32, 0, 1, 5, -10];
    /// assert_eq!(a.iter().position_first_max_by(|x, y| x.cmp(y)), Some(3));
    ///
    /// let a = [1_i32, 1, -1, -1];
    /// assert_eq!(a.iter().position_first_max_by(|x, y| x.cmp(y)), Some(0));
    /// ```
    fn position_first_max_by<F>(self, mut compare: F) -> Option<usize>
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> Ordering,
    {
        self.enumerate()
            .first_max_by(|x, y| compare(&x.1, &y.1))
            .map(|x| x.0)
    }

    /// Return the position of the minimum element in the iterator.
    ///
    /// If several elements are equally minimum, the position of the
    /// last of them is returned.
    ///
    /// This differs from [`itertools::Itertools::position_min`] in that it
    /// returns the last minimum instead of the first minimum.
    ///
    /// # Examples
    ///
    /// ```
    /// use solutions::iter::IteratorExt;
    ///
    /// let a: [i32; 0] = [];
    /// assert_eq!(a.iter().position_last_min(), None);
    ///
    /// let a = [-3, 0, 1, 5, -10];
    /// assert_eq!(a.iter().position_last_min(), Some(4));
    ///
    /// let a = [1, 1, -1, -1];
    /// assert_eq!(a.iter().position_last_min(), Some(3));
    /// ```
    fn position_last_min(self) -> Option<usize>
    where
        Self: Sized,
        Self::Item: Ord,
    {
        self.enumerate()
            .last_min_by(|x, y| Ord::cmp(&x.1, &y.1))
            .map(|x| x.0)
    }

    /// Return the position of the minimum element in the iterator, as
    /// determined by the specified function.
    ///
    /// If several elements are equally minimum, the position of the
    /// last of them is returned.
    ///
    /// This differs from [`itertools::Itertools::position_min_by_key`] in that
    /// it returns the last minimum instead of the first minimum.
    ///
    /// # Examples
    ///
    /// ```
    /// use solutions::iter::IteratorExt;
    ///
    /// let a: [i32; 0] = [];
    /// assert_eq!(a.iter().position_last_min_by_key(|x| x.abs()), None);
    ///
    /// let a = [-3_i32, 0, 1, 5, -10];
    /// assert_eq!(a.iter().position_last_min_by_key(|x| x.abs()), Some(1));
    ///
    /// let a = [1_i32, 1, -1, -1];
    /// assert_eq!(a.iter().position_last_min_by_key(|x| x.abs()), Some(3));
    /// ```
    fn position_last_min_by_key<K, F>(self, mut key: F) -> Option<usize>
    where
        Self: Sized,
        K: Ord,
        F: FnMut(&Self::Item) -> K,
    {
        self.enumerate()
            .last_min_by(|x, y| Ord::cmp(&key(&x.1), &key(&y.1)))
            .map(|x| x.0)
    }

    /// Return the position of the minimum element in the iterator, as
    /// determined by the specified comparison function.
    ///
    /// If several elements are equally minimum, the position of the
    /// last of them is returned.
    ///
    /// This differs from [`itertools::Itertools::position_min_by`] in that it
    /// returns the last minimum instead of the first minimum.
    ///
    /// # Examples
    ///
    /// ```
    /// use solutions::iter::IteratorExt;
    ///
    /// let a: [i32; 0] = [];
    /// assert_eq!(a.iter().position_last_min_by(|x, y| x.cmp(y)), None);
    ///
    /// let a = [-3_i32, 0, 1, 5, -10];
    /// assert_eq!(a.iter().position_last_min_by(|x, y| x.cmp(y)), Some(4));
    ///
    /// let a = [1_i32, 1, -1, -1];
    /// assert_eq!(a.iter().position_last_min_by(|x, y| x.cmp(y)), Some(3));
    /// ```
    fn position_last_min_by<F>(self, mut compare: F) -> Option<usize>
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> Ordering,
    {
        self.enumerate()
            .last_min_by(|x, y| compare(&x.1, &y.1))
            .map(|x| x.0)
    }

    /// Collects an iterator of numbers into a single number in the specified radix.
    ///
    /// `T` must implement the necessary arithmetic traits to perform the collection.
    ///
    /// # Examples
    ///
    /// ```
    /// use solutions::iter::IteratorExt;
    /// let a: [u8; _] = [1, 2, 3];
    /// let number = a.into_iter().collect_num(10);
    /// assert_eq!(number, 123);
    /// ```
    fn collect_num<T>(self, radix: T) -> T
    where
        Self: Sized + Iterator<Item = T>,
        T: Default + Copy + core::ops::Mul<Output = T> + core::ops::Add<Output = T>,
    {
        self.fold(T::default(), |acc, x| acc * radix + x)
    }
}

impl<T> IteratorExt for T where T: Iterator + ?Sized {}
