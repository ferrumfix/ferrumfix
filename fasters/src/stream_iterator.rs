// Copyright (C) Steven Fackler (@sfackler).
// This file is taken directly from
// https://github.com/sfackler/streaming-iterator/blob/master/src/lib.rs, which
// is the source for Steven's awesome library to do streaming iterator stuff
// (aka iterators that yield borrows). There's no mature release available
// despise the crate being high-quality, so I embedded it.
// TODO: release a 1.0 of this crate and add it as an external dependency.

//! Streaming iterators.
//!
//! The iterator APIs in the Rust standard library do not allow elements to be yielded which borrow
//! from the iterator itself. That means, for example, that the `std::io::Lines` iterator must
//! allocate a new `String` for each line rather than reusing an internal buffer. The
//! `StreamIterator` trait instead provides access to elements being iterated over only by
//! reference rather than by value.
//!
//! `StreamIterator`s cannot be used in Rust `for` loops, but `while let` loops offer a similar
//! level of ergonomics:
//!
//! ```ignore
//! while let Some(item) = iter.next() {
//!     // work with item
//! }
//! ```
//!
//! However, **make sure to only use the above form with a mutable reference to an existing iterator**,
//! not with an expression that creates an iterator.
//! For example, the following code will loop forever over the first element of the array:
//!
//! ```no_run
//! use fasters::stream_iterator::{convert, StreamIterator};
//! let array = [0, 1, 2, 3];
//!
//! while let Some(item) = convert(array.iter()).next() {
//!   // This is an infinite loop!
//! }
//! ```
//!
//! While the standard `Iterator` trait's functionality is based off of the `next` method,
//! `StreamIterator`'s functionality is based off of a pair of methods: `advance` and `get`. This
//! essentially splits the logic of `next` in half (in fact, `StreamIterator`'s `next` method
//! does nothing but call `advance` followed by `get`).
//!
//! This is required because of Rust's lexical handling of borrows (more specifically a lack of
//! single entry, multiple exit borrows). If `StreamIterator` was defined like `Iterator` with
//! just a required `next` method, operations like `filter` would be impossible to define.
#![doc(html_root_url = "https://docs.rs/streaming-iterator/0.1")]
#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate core;

use core::cmp;
use core::marker::PhantomData;

/// An interface for dealing with streaming iterators.
pub trait StreamIterator<'a> {
    /// The type of the elements being iterated over.
    type Item;

    fn advance(&mut self);

    /// Returns a reference to the current element of the iterator.
    ///
    /// The behavior of calling this method before `advance` has been called is unspecified.
    fn next(&'a self) -> Option<Self::Item>;

    /// Returns the bounds on the remaining length of the iterator.
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }

    /// Determines if all elements of the iterator satisfy a predicate.
    #[inline]
    fn all<F>(&'a mut self, mut f: F) -> bool
    where
        Self: Sized,
        F: FnMut(Self::Item) -> bool,
    {
        while let Some(i) = self.next() {
            if !f(i) {
                return false;
            }
        }

        true
    }

    /// Determines if any elements of the iterator satisfy a predicate.
    #[inline]
    fn any<F>(&'a mut self, mut f: F) -> bool
    where
        Self: Sized,
        F: FnMut(Self::Item) -> bool,
    {
        !self.all(|i| !f(i))
    }

    /// Borrows an iterator, rather than consuming it.
    ///
    /// This is useful to allow the application of iterator adaptors while still retaining ownership
    /// of the original adaptor.
    #[inline]
    fn by_ref(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        self
    }


    /// Returns the first element of the iterator that satisfies the predicate.
    #[inline]
    fn find<F>(&'a mut self, mut f: F) -> Option<Self::Item>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> bool,
    {
        loop {
            match self.next() {
                Some(i) => {
                    if f(i) {
                        break;
                    }
                }
                None => break,
            }
        }

        self.next()
    }

    /// Consumes the first `n` elements of the iterator, returning the next one.
    #[inline]
    fn nth(&'a mut self, n: usize) -> Option<Self::Item> {
        for _ in 0..n {
            if self.next().is_none() {
                return None;
            }
        }
        self.next()
    }

    /// Returns the index of the first element of the iterator matching a predicate.
    #[inline]
    fn position<F>(&'a mut self, mut f: F) -> Option<usize>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> bool,
    {
        let mut n = 0;

        while let Some(i) = self.next() {
            if f(i) {
                return Some(n);
            }
            n += 1;
        }

        None
    }
}