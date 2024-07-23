//! Implement extension methods over `[U]`.
//! Using the `slice_index_methods` feature gate it also implement extension methods over `U`.

use std::slice::SliceIndex;

/// Defines methods for getting elements at specified indexes without performing check on Release, but panicking on Debug.
pub trait DcSlice {
    /// Get the element at the specified index without performing checks on release.
    ///
    /// # Panics (Debug)
    ///
    /// Panics if index is out of range.
    ///
    /// # Safety
    ///
    /// Index should always be on range.
    ///
    /// Failing this produces undefined behavior on Release.
    unsafe fn get_dc<T: SliceIndex<Self>>(&self, index: T) -> &T::Output;

    /// Get the element at the specified index without performing checks on release.
    ///
    /// # Panics (Debug)
    ///
    /// Panics with the specified message if index is out of range if index is out of range.
    ///
    /// # Safety
    ///
    /// Index should always be on range.
    ///
    /// Failing this produces undefined behavior on Release.
    unsafe fn get_expect_dc<T: SliceIndex<Self>>(&self, index: T, msg: &str) -> &T::Output;

    /// Get the mutable element at the specified index without performing checks on release.
    ///
    /// # Panics (Debug)
    ///
    /// Panics if index is out of range if index is out of range.
    ///
    /// # Safety
    ///
    /// Index should always be on range.
    ///
    /// Failing this produces undefined behavior on Release.
    unsafe fn get_dc_mut<T: SliceIndex<Self>>(&mut self, index: T) -> &mut T::Output;

    /// Get the mutable element at the specified index without performing checks on release.
    ///
    /// # Panics (Debug)
    ///
    /// Panics with the specified message if index is out of range if index is out of range.
    ///
    /// # Safety
    ///
    /// Index should always be on range.
    ///
    /// Failing this produces undefined behavior on Release.
    unsafe fn get_expect_dc_mut<T: SliceIndex<Self>>(
        &mut self,
        index: T,
        msg: &str,
    ) -> &mut T::Output;
}

#[cfg(debug_assertions)]
impl<U> DcSlice for [U] {
    #[inline(always)]
    #[track_caller]
    unsafe fn get_dc<T: SliceIndex<Self>>(&self, index: T) -> &T::Output {
        self.get(index).unwrap()
    }

    #[inline(always)]
    #[track_caller]
    unsafe fn get_expect_dc<T: SliceIndex<Self>>(&self, index: T, msg: &str) -> &T::Output {
        self.get(index).expect(msg)
    }

    #[inline(always)]
    #[track_caller]
    unsafe fn get_dc_mut<T: SliceIndex<Self>>(&mut self, index: T) -> &mut T::Output {
        self.get_mut(index).unwrap()
    }

    #[inline(always)]
    #[track_caller]
    unsafe fn get_expect_dc_mut<T: SliceIndex<Self>>(
        &mut self,
        index: T,
        msg: &str,
    ) -> &mut T::Output {
        self.get_mut(index).expect(msg)
    }
}

#[cfg(not(debug_assertions))]
impl<U> DcSlice for [U] {
    #[inline(always)]
    unsafe fn get_dc<T: SliceIndex<Self>>(&self, index: T) -> &T::Output {
        self.get(index).unwrap_unchecked()
    }

    #[inline(always)]
    unsafe fn get_expect_dc<T: SliceIndex<Self>>(&self, index: T, _msg: &str) -> &T::Output {
        self.get(index).unwrap_unchecked()
    }

    #[inline(always)]
    unsafe fn get_dc_mut<T: SliceIndex<Self>>(&mut self, index: T) -> &mut T::Output {
        self.get_mut(index).unwrap_unchecked()
    }

    #[inline(always)]
    unsafe fn get_expect_dc_mut<T: SliceIndex<Self>>(
        &mut self,
        index: T,
        _msg: &str,
    ) -> &mut T::Output {
        self.get_mut(index).unwrap_unchecked()
    }
}

#[cfg(feature = "slice_index_methods")]
#[cfg(debug_assertions)]
impl<U> DcSlice for U {
    #[inline(always)]
    #[track_caller]
    unsafe fn get_dc<T: SliceIndex<Self>>(&self, index: T) -> &T::Output {
        index.get(self).unwrap()
    }

    #[inline(always)]
    #[track_caller]
    unsafe fn get_expect_dc<T: SliceIndex<Self>>(&self, index: T, msg: &str) -> &T::Output {
        index.get(self).expect(msg)
    }

    #[inline(always)]
    #[track_caller]
    unsafe fn get_dc_mut<T: SliceIndex<Self>>(&mut self, index: T) -> &mut T::Output {
        index.get_mut(self).unwrap()
    }

    #[inline(always)]
    #[track_caller]
    unsafe fn get_expect_dc_mut<T: SliceIndex<Self>>(
        &mut self,
        index: T,
        msg: &str,
    ) -> &mut T::Output {
        index.get_mut(self).expect(msg)
    }
}

#[cfg(feature = "slice_index_methods")]
#[cfg(not(debug_assertions))]
impl<U> DcSlice for U {
    #[inline(always)]
    unsafe fn get_dc<T: SliceIndex<Self>>(&self, index: T) -> &T::Output {
        index.get(self).unwrap_unchecked()
    }

    #[inline(always)]
    unsafe fn get_expect_dc<T: SliceIndex<Self>>(&self, index: T, _msg: &str) -> &T::Output {
        index.get(self).unwrap_unchecked()
    }

    #[inline(always)]
    unsafe fn get_dc_mut<T: SliceIndex<Self>>(&mut self, index: T) -> &mut T::Output {
        index.get_mut(self).unwrap_unchecked()
    }

    #[inline(always)]
    unsafe fn get_expect_dc_mut<T: SliceIndex<Self>>(
        &mut self,
        index: T,
        _msg: &str,
    ) -> &mut T::Output {
        index.get_mut(self).unwrap_unchecked()
    }
}
