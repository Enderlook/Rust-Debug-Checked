use std::{marker::Unsize, ops::{CoerceUnsized, Deref}, fmt::{Display, Formatter, Error}};

/// Wraps a borrowed reference to a value in a `DcRefCell` box.
/// A wrapper type for an immutably borrowed value from a `DcRefCell<T>`.
#[cfg(debug_assertions)]
#[must_not_suspend = "holding a Ref across suspend points can cause BorrowErrors"]
#[repr(transparent)]
#[derive(Debug)]
pub struct Ref<'b, T: ?Sized + 'b>(pub(super) std::cell::Ref<'b, T>);

/// Wraps a borrowed reference to a value in a `DcRefCell` box.
/// A wrapper type for an immutably borrowed value from a `DcRefCell<T>`.
#[cfg(not(debug_assertions))]
#[must_not_suspend = "holding a Ref across suspend points can cause BorrowErrors"]
#[repr(transparent)]
#[derive(Debug)]
pub struct Ref<'b, T: ?Sized + 'b>(pub(super) &'b T);

impl<'b, T: Unsize<U> + ?Sized, U: ?Sized> CoerceUnsized<Ref<'b, U>> for Ref<'b, T> {}

impl<'b, T: ?Sized> Deref for Ref<'b, T> {
    /// The resulting type after dereferencing.
    type Target = T;

    /// Dereferences the value.
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<T: Display + ?Sized> Display for Ref<'_, T> {
    /// Formats the value using the given formatter.
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        Display::fmt(&self.0, f)
    }
}

#[cfg(debug_assertions)]
impl<'b, T> Ref<'b, T> {
    /// Copies a `Ref`.
    ///
    /// The `DcRefCell` is already immutably borrowed, so this cannot fail.
    ///
    /// This is an associated function that needs to be used as `Ref::clone(...)`.
    /// A `Clone` implementation or a method would interfere with the widespread use of `r.borrow().clone()` to clone the contents of a `DcRefCell`.
    #[inline(always)]
    pub fn clone(orig: &Self) ->Self {
        Ref(std::cell::Ref::clone(&orig.0))
    }

    /// Makes a new `Ref` for an optional component of the borrowed data.
    /// The original guard is returned as an `Err(..)` if the closure returns `None`.
    ///
    /// The `DcRefCell` is already immutably borrowed, so this cannot fail.
    ///
    /// This is an associated function that needs to be used as `Ref::filter_map(...)`.
    /// A method would interfere with methods of the same name on the contents of a `DcRefCell` used through `Deref`.
    #[inline(always)]
    pub fn filter_map<U: ?Sized, F: FnOnce(&T) -> Option<&U>>(orig: Self, f: F) -> Result<Ref<'b, U>, Self> {
        std::cell::Ref::filter_map(orig.0, f).map(|e| Ref(e)).map_err(|e| Ref(e))
    }

    /// Convert into a reference to the underlying data.
    ///
    /// The underlying `DcRefCell` can never be mutably borrowed from again and will always appear already immutably borrowed.
    /// It is not a good idea to leak more than a constant number of references.
    /// The `DcRefCell` can be immutably borrowed again if only a smaller number of leaks have occurred in total.
    ///
    /// This is an associated function that needs to be used as `Ref::leak(...)`.
    /// A method would interfere with methods of the same name on the contents of a `DcRefCell` used through `Deref`.
    #[inline(always)]
    #[cfg(feature = "cell_leak")]
    pub fn leak(orig: Self) -> &'b T {
        std::cell::Ref::leak(orig.0)
    }

    /// Makes a new `Ref` for a component of the borrowed data.
    ///
    /// The `RefCell` is already immutably borrowed, so this cannot fail.
    ///
    /// This is an associated function that needs to be used as `Ref::map(...)`.
    /// A method would interfere with methods of the same name on the contents of a `DcRefCell` used through `Deref`.
    #[inline(always)]
    pub fn map<U: ?Sized, F: FnOnce(&T) -> &U>(orig: Self, f: F) -> Ref<'b, U> {
        Ref(std::cell::Ref::map(orig.0, f))
    }

    /// Splits a `Ref` into multiple `Ref`s for different components of the borrowed data.
    ///
    /// The `DcRefCell` is already immutably borrowed, so this cannot fail.
    ///
    /// This is an associated function that needs to be used as `Ref::map_split(...)`.
    /// A method would interfere with methods of the same name on the contents of a `DcRefCell` used through `Deref`.
    #[inline(always)]
    pub fn map_split<U: ?Sized, V: ?Sized, F: FnOnce(&T) -> (&U, &V)>(orig: Self, f: F) -> (Ref<'b, U>, Ref<'b, V>) {
        let tuple = std::cell::Ref::map_split(orig.0, f);
        (Ref(tuple.0), Ref(tuple.1))
    }
}

#[cfg(not(debug_assertions))]
impl<'b, T> Ref<'b, T> {
    /// Copies a `Ref`.
    ///
    /// The `DcRefCell` is already immutably borrowed, so this cannot fail.
    ///
    /// This is an associated function that needs to be used as `Ref::clone(...)`.
    /// A `Clone` implementation or a method would interfere with the widespread use of `r.borrow().clone()` to clone the contents of a `DcRefCell`.
    #[inline(always)]
    pub fn clone(orig: &Self) ->Self {
        Ref(orig.0)
    }

    /// Makes a new `Ref` for an optional component of the borrowed data.
    /// The original guard is returned as an `Err(..)` if the closure returns `None`.
    ///
    /// The `DcRefCell` is already immutably borrowed, so this cannot fail.
    ///
    /// This is an associated function that needs to be used as `Ref::filter_map(...)`.
    /// A method would interfere with methods of the same name on the contents of a `DcRefCell` used through `Deref`.
    #[inline(always)]
    pub fn filter_map<U: ?Sized, F: FnOnce(&T) -> Option<&U>>(orig: Self, f: F) -> Result<Ref<'b, U>, Self> {
        match f(orig.0) {
            Some(value) => Ok(Ref(value)),
            None => Err(orig),
        }
    }

    /// Convert into a reference to the underlying data.
    ///
    /// The underlying `DcRefCell` can never be mutably borrowed from again and will always appear already immutably borrowed.
    /// It is not a good idea to leak more than a constant number of references.
    /// The `DcRefCell` can be immutably borrowed again if only a smaller number of leaks have occurred in total.
    ///
    /// This is an associated function that needs to be used as `Ref::leak(...)`.
    /// A method would interfere with methods of the same name on the contents of a `DcRefCell` used through `Deref`.
    #[inline(always)]
    #[cfg(feature = "cell_leak")]
    pub fn leak(orig: Self) -> &'b T {
        orig.0
    }

    /// Makes a new `Ref` for a component of the borrowed data.
    ///
    /// The `RefCell` is already immutably borrowed, so this cannot fail.
    ///
    /// This is an associated function that needs to be used as `Ref::map(...)`.
    /// A method would interfere with methods of the same name on the contents of a `DcRefCell` used through `Deref`.
    #[inline(always)]
    pub fn map<U: ?Sized, F: FnOnce(&T) -> &U>(orig: Self, f: F) -> Ref<'b, U> {
        Ref(f(orig.0))
    }

    /// Splits a `Ref` into multiple `Ref`s for different components of the borrowed data.
    ///
    /// The `DcRefCell` is already immutably borrowed, so this cannot fail.
    ///
    /// This is an associated function that needs to be used as `Ref::map_split(...)`.
    /// A method would interfere with methods of the same name on the contents of a `DcRefCell` used through `Deref`.
    #[inline(always)]
    pub fn map_split<U: ?Sized, V: ?Sized, F: FnOnce(&T) -> (&U, &V)>(orig: Self, f: F) -> (Ref<'b, U>, Ref<'b, V>) {
        let (a, b) = f(orig.0);
        (Ref(a), Ref(b))
    }
}