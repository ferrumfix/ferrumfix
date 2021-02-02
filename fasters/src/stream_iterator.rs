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

#[cfg(feature = "std")]
extern crate core;

/// [`Iterator`]-like interface with support for borrowed items.
pub trait StreamIterator<'a> {
    /// The type of the elements being iterated over.
    type Item;

    /// Advances `self` to the next element.
    fn advance(&mut self);

    /// Returns a reference to the current element of the iterator.
    ///
    /// The behavior of calling this method before `advance` has been called is unspecified.
    fn get(&'a self) -> Option<Self::Item>;

    /// Advances the iterator and returns the next value.
    ///
    /// The behavior of calling this method after the end of the iterator has been reached is
    /// unspecified.
    ///
    /// The default implementation simply calls `advance` followed by `get`.
    #[inline]
    fn next(&'a mut self) -> Option<Self::Item> {
        self.advance();
        self.get()
    }
}
