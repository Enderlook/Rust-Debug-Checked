use std::{marker::{Unsize, PhantomData}, ops::{CoerceUnsized, Deref, DerefMut}, fmt::{Display, Formatter, Error}, ptr::NonNull};

/// A wrapper type for a mutably borrowed value from a `DcRefCell<T>`.
#[cfg(debug_assertions)]
#[must_not_suspend = "holding a Ref across suspend points can cause BorrowErrors"]
#[repr(transparent)]
#[derive(Debug)]
pub struct RefMut<'b, T: ?Sized + 'b>(pub(super) std::cell::RefMut<'b, T>);

/// A wrapper type for a mutably borrowed value from a `DcRefCell<T>`.
#[cfg(not(debug_assertions))]
#[must_not_suspend = "holding a Ref across suspend points can cause BorrowErrors"]
#[repr(transparent)]
#[derive(Debug)]
pub struct RefMut<'b, T: ?Sized + 'b>(pub(super) NonNull<T>, pub(super) PhantomData<&'b mut T>);

impl<'b, T: Unsize<U> + ?Sized, U: ?Sized> CoerceUnsized<RefMut<'b, U>> for RefMut<'b, T> {}

#[cfg(debug_assertions)]
impl<'b, T: ?Sized> Deref for RefMut<'b, T> {
    /// The resulting type after dereferencing.
    type Target = T;

    /// Dereferences the value.
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[cfg(debug_assertions)]
impl<'b, T: ?Sized> DerefMut for RefMut<'b, T> {
    /// Mutably dereferences the value.
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.deref_mut()
    }
}

#[cfg(debug_assertions)]
impl<T: Display + ?Sized> Display for RefMut<'_, T> {
    /// Formats the value using the given formatter.
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        Display::fmt(&*self.0, f)
    }
}

#[cfg(debug_assertions)]
impl<'b, T> RefMut<'b, T> {
    /// Makes a new `RefMut` for an optional component of the borrowed data.
    /// The original guard is returned as an `Err(..)` if the closure returns `None`.
    ///
    /// The `DcRefCell` is already immutably borrowed, so this cannot fail.
    ///
    /// This is an associated function that needs to be used as `RefMut::filter_map(...)`.
    /// A method would interfere with methods of the same name on the contents of a `DcRefCell` used through `Deref`.
    #[inline(always)]
    pub fn filter_map<U: ?Sized, F: FnOnce(&mut T) -> Option<&mut U>>(orig: Self, f: F) -> Result<RefMut<'b, U>, RefMut<'b, T>> {
        std::cell::RefMut::filter_map(orig.0, f).map(|e| RefMut(e)).map_err(|e| RefMut(e))
    }

    /// Convert into a reference to the underlying data.
    ///
    /// The underlying `DcRefCell` can never be mutably borrowed from again and will always appear already immutably borrowed.
    /// It is not a good idea to leak more than a constant number of references.
    /// The `DcRefCell` can be immutably borrowed again if only a smaller number of leaks have occurred in total.
    ///
    /// This is an associated function that needs to be used as `RefMut::leak(...)`.
    /// A method would interfere with methods of the same name on the contents of a `DcRefCell` used through `Deref`.
    #[inline(always)]
    #[cfg(feature = "cell_leak")]
    pub fn leak(orig: Self) -> &'b T {
        std::cell::RefMut::leak(orig.0)
    }

    /// Makes a new `RefMut` for a component of the borrowed data.
    ///
    /// The `RefCell` is already immutably borrowed, so this cannot fail.
    ///
    /// This is an associated function that needs to be used as `Ref::map(...)`.
    /// A method would interfere with methods of the same name on the contents of a `DcRefCell` used through `Deref`.
    #[inline(always)]
    pub fn map<U: ?Sized, F: FnOnce(&mut T) -> &mut U>(orig: Self, f: F) -> RefMut<'b, U> {
        RefMut(std::cell::RefMut::map(orig.0, f))
    }

    /// Splits a `RefMut` into multiple `RefMut`s for different components of the borrowed data.
    ///
    /// The `DcRefCell` is already immutably borrowed, so this cannot fail.
    ///
    /// This is an associated function that needs to be used as `RefMut::map_split(...)`.
    /// A method would interfere with methods of the same name on the contents of a `DcRefCell` used through `Deref`.
    #[inline(always)]
    pub fn map_split<U: ?Sized, V: ?Sized, F: FnOnce(&mut T) -> (&mut U, &mut V)>(orig: Self, f: F) -> (RefMut<'b, U>, RefMut<'b, V>) {
        let tuple = std::cell::RefMut::map_split(orig.0, f);
        (RefMut(tuple.0), RefMut(tuple.1))
    }
}

#[cfg(not(debug_assertions))]
impl<'b, T: ?Sized> Deref for RefMut<'b, T> {
    /// The resulting type after dereferencing.
    type Target = T;

    /// Dereferences the value.
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        // Safety: the value is accessible as long as we hold our borrow.
        unsafe { self.0.as_ref() }
    }
}

#[cfg(not(debug_assertions))]
impl<'b, T: ?Sized> DerefMut for RefMut<'b, T> {
    /// Mutably dereferences the value.
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety: the value is accessible as long as we hold our borrow.
        unsafe { self.0.as_mut() }
    }
}

#[cfg(not(debug_assertions))]
impl<T: Display + ?Sized> Display for RefMut<'_, T> {
    /// Formats the value using the given formatter
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        // Safety: the value is accessible as long as we hold our borrow.
        Display::fmt(unsafe { self.0.as_ref() }, f)
    }
}

#[cfg(not(debug_assertions))]
impl<'b, T> RefMut<'b, T> {
    /// Makes a new `RefMut` for an optional component of the borrowed data.
    /// The original guard is returned as an `Err(..)` if the closure returns `None`.
    ///
    /// The `DcRefCell` is already immutably borrowed, so this cannot fail.
    ///
    /// This is an associated function that needs to be used as `RefMut::filter_map(...)`.
    /// A method would interfere with methods of the same name on the contents of a `DcRefCell` used through `Deref`.
    #[inline(always)]
    pub fn filter_map<U: ?Sized, F: FnOnce(&mut T) -> Option<&mut U>>(mut orig: Self, f: F) -> Result<RefMut<'b, U>, Self> {
        // Safety: function holds onto an exclusive reference for the duration
        // of its call through `orig`, and the pointer is only de-referenced
        // inside of the function call never allowing the exclusive reference to
        // escape.
        match f(&mut *orig) {
            Some(value) => Ok(RefMut(NonNull::from(value), PhantomData)),
            None => Err(orig),
        }
    }

    /// Convert into a reference to the underlying data.
    ///
    /// The underlying `DcRefCell` can never be mutably borrowed from again and will always appear already immutably borrowed.
    /// It is not a good idea to leak more than a constant number of references.
    /// The `DcRefCell` can be immutably borrowed again if only a smaller number of leaks have occurred in total.
    ///
    /// This is an associated function that needs to be used as `RefMut::leak(...)`.
    /// A method would interfere with methods of the same name on the contents of a `DcRefCell` used through `Deref`.
    #[inline(always)]
    #[cfg(feature = "cell_leak")]
    pub fn leak(orig: Self) -> &'b T {
        orig.0
    }

    /// Makes a new `RefMut` for a component of the borrowed data.
    ///
    /// The `RefCell` is already immutably borrowed, so this cannot fail.
    ///
    /// This is an associated function that needs to be used as `Ref::map(...)`.
    /// A method would interfere with methods of the same name on the contents of a `DcRefCell` used through `Deref`.
    #[inline(always)]
    pub fn map<U: ?Sized, F: FnOnce(&mut T) -> &mut U>(mut orig: Self, f: F) -> RefMut<'b, U> {
        RefMut(NonNull::from(f(&mut *orig)), PhantomData)
    }

    /// Splits a `RefMut` into multiple `RefMut`s for different components of the borrowed data.
    ///
    /// The `DcRefCell` is already immutably borrowed, so this cannot fail.
    ///
    /// This is an associated function that needs to be used as `RefMut::map_split(...)`.
    /// A method would interfere with methods of the same name on the contents of a `DcRefCell` used through `Deref`.
    #[inline(always)]
    pub fn map_split<U: ?Sized, V: ?Sized, F: FnOnce(&mut T) -> (&mut U, &mut V)>(mut orig: Self, f: F) -> (RefMut<'b, U>, RefMut<'b, V>) {
        let (a, b) = f(&mut *orig);
        (RefMut(NonNull::from(a), PhantomData), RefMut(NonNull::from(b), PhantomData))
    }
}
