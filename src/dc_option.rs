//! Implement extension methods over `Option<T>`.

/// Defines methods to extract the `Some(T)` of an `Option<T>` without performing checks on Release, but panicking on Debug.
pub trait DcOption {
    /// Type in `Option<T>`.
    type Result;

    /// Unwraps the content of the option without performing checks on release.
    ///
    /// # Panics (Debug)
    ///
    /// Panics if `self` is `None`.
    ///
    /// # Safety
    ///
    /// `self` must be `Some(T)`.
    ///
    ///  Failing this produces undefined behavior on Release.
    unsafe fn unwrap_dc(self) -> Self::Result;

    /// Unwraps the content of the option without performing checks on release.
    ///
    /// # Panics (Debug)
    ///
    /// Panics with the specified message if `self` is `None`.
    ///
    /// # Safety
    ///
    /// `self` must be `Some(T)`.
    ///
    ///  Failing this produces undefined behavior on Release.
    unsafe fn expect_dc(self, msg: &str) -> Self::Result;
}

#[cfg(debug_assertions)]
impl<T> DcOption for Option<T> {
    type Result = T;

    #[inline(always)]
    #[track_caller]
    unsafe fn unwrap_dc(self) -> Self::Result {
        self.unwrap()
    }

    #[inline(always)]
    #[track_caller]
    unsafe fn expect_dc(self, msg: &str) -> Self::Result {
        self.expect(msg)
    }
}

#[cfg(not(debug_assertions))]
impl<T> DcOption for Option<T> {
    type Result = T;

    #[inline(always)]
    unsafe fn unwrap_dc(self) -> Self::Result {
        self.unwrap_unchecked()
    }

    #[inline(always)]
    unsafe fn expect_dc(self, _msg: &str) -> Self::Result {
        self.unwrap_unchecked()
    }
}
