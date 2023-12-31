//! Items which do not have a correspondence to any API in the proc_macro crate,
//! but are necessary to include in proc-macro2.

use crate::fallback;
use crate::imp;
use crate::marker::Marker;
use crate::Span;
use core::fmt::{self, Debug};

/// An object that holds a [`Group`]'s `span_open()` and `span_close()` together
/// (in a more compact representation than holding those 2 spans individually.
///
/// [`Group`]: crate::Group
#[derive(Copy, Clone)]
pub struct DelimSpan {
    inner: DelimSpanEnum,
    _marker: Marker,
}

#[derive(Copy, Clone)]
enum DelimSpanEnum {
    Fallback(fallback::Span),
}

impl DelimSpan {
    pub(crate) fn new(group: &imp::Group) -> Self {
        let inner = DelimSpanEnum::Fallback(group.span());

        DelimSpan {
            inner,
            _marker: Marker,
        }
    }

    /// Returns a span covering the entire delimited group.
    pub fn join(&self) -> Span {
        match &self.inner {
            DelimSpanEnum::Fallback(span) => Span::_new_fallback(*span),
        }
    }

    /// Returns a span for the opening punctuation of the group only.
    pub fn open(&self) -> Span {
        match &self.inner {
            DelimSpanEnum::Fallback(span) => Span::_new_fallback(span.first_byte()),
        }
    }

    /// Returns a span for the closing punctuation of the group only.
    pub fn close(&self) -> Span {
        match &self.inner {
            DelimSpanEnum::Fallback(span) => Span::_new_fallback(span.last_byte()),
        }
    }
}

impl Debug for DelimSpan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(&self.join(), f)
    }
}
