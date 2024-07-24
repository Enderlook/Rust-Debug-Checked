/// Defines methods to extract the `Ok(T)` of a `Result<T, E>`.
pub trait DcResultOk {
    /// Result type in `Result<T, E>`.
    type T;

    /// Error type in `Result<T, E>`.
    type E;

    /// Unwraps the ok content of the result without performing checks on release.
    ///
    /// # Panics (Debug)
    ///
    /// Panics if `self` is `Err(E)`.
    ///
    /// # Safety
    ///
    /// `self` must be `Ok(T)`.
    ///
    /// Failing this produces undefined behavior on Release.
    unsafe fn unwrap_dc_ok(self) -> Self::T;

    /// Unwraps the ok content of the option without performing checks on release.
    ///
    /// # Panics (Debug)
    ///
    /// Panics with the specified message if `self` is `Err(E)`.
    ///
    /// # Safety
    ///
    /// `self` must be `Ok(T)`.
    ///
    /// Failing this produces undefined behavior on Release.
    unsafe fn expect_dc_ok(self, msg: &str) -> Self::T;
}

/// Defines methods to extract the `Err(E)` of a `Result<T, E>`.
pub trait DcResultErr {
    /// Result type in `Result<T, E>`.
    type T;

    /// Error type in `Result<T, E>`.
    type E;

    /// Unwraps the err content of the result without performing checks on release.
    ///
    /// # Panics (Debug)
    ///
    /// Panics if `self` is `Ok(T)`.
    ///
    /// # Safety
    ///
    /// `self` must be `Err(E)`.
    ///
    /// Failing this produces undefined behavior on Release.
    unsafe fn unwrap_dc_err(self) -> Self::E;

    /// Unwraps the err content of the option without performing checks on release.
    ///
    /// # Panics (Debug)
    ///
    /// Panics with the specified message if `self` is `Ok(T)`.
    ///
    /// # Safety
    ///
    /// `self` must be `Err(E)`.
    ///
    /// Failing this produces undefined behavior on Release.
    unsafe fn expect_dc_err(self, msg: &str) -> Self::E;
}

#[cfg(debug_assertions)]
impl<T, E: std::fmt::Debug> DcResultOk for Result<T, E> {
    type T = T;
    type E = E;

    #[inline(always)]
    #[track_caller]
    unsafe fn unwrap_dc_ok(self) -> Self::T {
        self.unwrap()
    }

    #[inline(always)]
    #[track_caller]
    unsafe fn expect_dc_ok(self, msg: &str) -> Self::T {
        self.expect(msg)
    }
}

#[cfg(debug_assertions)]
impl<T: std::fmt::Debug, E> DcResultErr for Result<T, E> {
    type T = T;
    type E = E;

    #[inline(always)]
    #[track_caller]
    unsafe fn unwrap_dc_err(self) -> Self::E {
        self.unwrap_err()
    }

    #[inline(always)]
    #[track_caller]
    unsafe fn expect_dc_err(self, msg: &str) -> Self::E {
        self.expect_err(msg)
    }
}

#[cfg(not(debug_assertions))]
impl<T, E: std::fmt::Debug> DcResultOk for Result<T, E> {
    type T = T;
    type E = E;

    #[inline(always)]
    unsafe fn unwrap_dc_ok(self) -> Self::T {
        self.unwrap_unchecked()
    }

    #[inline(always)]
    unsafe fn expect_dc_ok(self, _msg: &str) -> Self::T {
        self.unwrap_unchecked()
    }
}

#[cfg(not(debug_assertions))]
impl<T, E: std::fmt::Debug> DcResultErr for Result<T, E> {
    type T = T;
    type E = E;

    #[inline(always)]
    unsafe fn unwrap_dc_err(self) -> Self::E {
        self.unwrap_err_unchecked()
    }

    #[inline(always)]
    unsafe fn expect_dc_err(self, _msg: &str) -> Self::E {
        self.unwrap_err_unchecked()
    }
}
