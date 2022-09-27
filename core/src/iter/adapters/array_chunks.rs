use crate::array;
use crate::const_closure::ConstFnMutClosure;
use crate::iter::{ByRefSized, FusedIterator, Iterator};
use crate::ops::{ControlFlow, NeverShortCircuit, Try};

/// An iterator over `N` elements of the iterator at a time.
///
/// The chunks do not overlap. If `N` does not divide the length of the
/// iterator, then the last up to `N-1` elements will be omitted.
///
/// This `struct` is created by the [`array_chunks`][Iterator::array_chunks]
/// method on [`Iterator`]. See its documentation for more.
#[derive(Debug, Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
#[unstable(feature = "iter_array_chunks", reason = "recently added", issue = "100450")]
pub struct ArrayChunks<I: Iterator, const N: usize> {
    iter: I,
    remainder: Option<array::IntoIter<I::Item, N>>,
}

impl<I, const N: usize> ArrayChunks<I, N>
where
    I: Iterator,
{
    #[track_caller]
    pub(in crate::iter) fn new(iter: I) -> Self {
        assert!(N != 0, "chunk size must be non-zero");
        Self { iter, remainder: None }
    }

    /// Returns an iterator over the remaining elements of the original iterator
    /// that are not going to be returned by this iterator. The returned
    /// iterator will yield at most `N-1` elements.
    #[unstable(feature = "iter_array_chunks", reason = "recently added", issue = "100450")]
    #[inline]
    pub fn into_remainder(self) -> Option<array::IntoIter<I::Item, N>> {
        self.remainder
    }
}

#[unstable(feature = "iter_array_chunks", reason = "recently added", issue = "100450")]
impl<I, const N: usize> Iterator for ArrayChunks<I, N>
where
    I: Iterator,
{
    type Item = [I::Item; N];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.try_for_each(ControlFlow::Break).break_value()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.iter.size_hint();

        (lower / N, upper.map(|n| n / N))
    }

    #[inline]
    fn count(self) -> usize {
        self.iter.count() / N
    }

    fn try_fold<B, F, R>(&mut self, init: B, mut f: F) -> R
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> R,
        R: Try<Output = B>,
    {
        let mut acc = init;
        loop {
            match self.iter.next_chunk() {
                Ok(chunk) => acc = f(acc, chunk)?,
                Err(remainder) => {
                    // Make sure to not override `self.remainder` with an empty array
                    // when `next` is called after `ArrayChunks` exhaustion.
                    self.remainder.get_or_insert(remainder);

                    break try { acc };
                }
            }
        }
    }

    fn fold<B, F>(mut self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        self.try_fold(init, ConstFnMutClosure::new(&mut f, NeverShortCircuit::wrap_mut_2_imp)).0
    }
}

#[unstable(feature = "iter_array_chunks", reason = "recently added", issue = "100450")]
impl<I, const N: usize> DoubleEndedIterator for ArrayChunks<I, N>
where
    I: DoubleEndedIterator + ExactSizeIterator,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.try_rfold((), |(), x| ControlFlow::Break(x)).break_value()
    }

    fn try_rfold<B, F, R>(&mut self, init: B, mut f: F) -> R
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> R,
        R: Try<Output = B>,
    {
        // We are iterating from the back we need to first handle the remainder.
        self.next_back_remainder();

        let mut acc = init;
        let mut iter = ByRefSized(&mut self.iter).rev();

        // NB remainder is handled by `next_back_remainder`, so
        // `next_chunk` can't return `Err` with non-empty remainder
        // (assuming correct `I as ExactSizeIterator` impl).
        while let Ok(mut chunk) = iter.next_chunk() {
            // FIXME: do not do double reverse
            //        (we could instead add `next_chunk_back` for example)
            chunk.reverse();
            acc = f(acc, chunk)?
        }

        try { acc }
    }

    fn rfold<B, F>(mut self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        self.try_rfold(init, ConstFnMutClosure::new(&mut f, NeverShortCircuit::wrap_mut_2_imp)).0
    }
}

impl<I, const N: usize> ArrayChunks<I, N>
where
    I: DoubleEndedIterator + ExactSizeIterator,
{
    /// Updates `self.remainder` such that `self.iter.len` is divisible by `N`.
    fn next_back_remainder(&mut self) {
        // Make sure to not override `self.remainder` with an empty array
        // when `next_back` is called after `ArrayChunks` exhaustion.
        if self.remainder.is_some() {
            return;
        }

        // We use the `ExactSizeIterator` implementation of the underlying
        // iterator to know how many remaining elements there are.
        let rem = self.iter.len() % N;

        // Take the last `rem` elements out of `self.iter`.
        let mut remainder =
            // SAFETY: `unwrap_err` always succeeds because x % N < N for all x.
            unsafe { self.iter.by_ref().rev().take(rem).next_chunk().unwrap_err_unchecked() };

        // We used `.rev()` above, so we need to re-reverse the reminder
        remainder.as_mut_slice().reverse();
        self.remainder = Some(remainder);
    }
}

#[unstable(feature = "iter_array_chunks", reason = "recently added", issue = "100450")]
impl<I, const N: usize> FusedIterator for ArrayChunks<I, N> where I: FusedIterator {}

#[unstable(feature = "iter_array_chunks", reason = "recently added", issue = "100450")]
impl<I, const N: usize> ExactSizeIterator for ArrayChunks<I, N>
where
    I: ExactSizeIterator,
{
    #[inline]
    fn len(&self) -> usize {
        self.iter.len() / N
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.iter.len() < N
    }
}