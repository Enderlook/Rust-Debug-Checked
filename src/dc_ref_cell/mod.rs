mod r#ref;
mod ref_mut;

use std::{cell::{RefCell, UnsafeCell}, mem, fmt::{Debug, Formatter, Result}, cmp::Ordering, ops::CoerceUnsized, marker::PhantomData, ptr::NonNull};

pub use r#ref::Ref;
pub use ref_mut::RefMut;

/// A mutable memory location, which does not perform checks in release, but it does in debug.
///
/// At debug, it behaves like `RefCell<T>`.
///
/// At release, it behaves like `UnsafeCell<T>`.
///
/// To simplify debugging and finding errors, all the operations whose checks only run at debug are marked as `unsafe`.
///
/// For this reason the type doesn't implement `Clone`, `Eq`, `Ord`, `PartialEq` nor `PartialOrd` unlike `RefCell<T>`, as they would be unsafe but we can't mark them as such.
#[cfg(debug_assertions)]
#[derive(Debug, Default)]
pub struct DcRefCell<T: ?Sized>(RefCell<T>);

/// A mutable memory location, which does not perform checks in release, but it does in debug.
///
/// At debug, it behaves like `RefCell<T>`.
///
/// At release, it behaves like `UnsafeCell<T>`.
///
/// To simplify debugging and finding errors, all the operations whose checks only run at debug are marked as `unsafe`.
///
/// For this reason the type doesn't implement `Clone`, `Eq`, `Ord`, `PartialEq` nor `PartialOrd` unlike `RefCell<T>`, as they would be unsafe but we can't mark them as such.
#[cfg(not(debug_assertions))]
#[derive(Default)]
pub struct DcRefCell<T: ?Sized>(UnsafeCell<T>);

#[cfg(not(debug_assertions))]
impl<T: ?Sized> Debug for DcRefCell<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("DcRefCell").finish_non_exhaustive()
    }
}

impl<T: ?Sized> !Sync for DcRefCell<T> {}

unsafe impl<T: ?Sized> Send for DcRefCell<T> where T: Send {}

impl<T: CoerceUnsized<U>, U> CoerceUnsized<DcRefCell<U>> for DcRefCell<T> {}

impl<T> From<T> for DcRefCell<T> {
    /// Creates a new `DcRefCell<T>` containing the given value.
    #[inline(always)]
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T: ?Sized + Clone> DcRefCell<T> {
    /// Returns a copy of the value.
    ///
    /// # Panics (Debug)
    ///
    /// Panics if the value is currently mutably borrowed.
    ///
    /// # Safety
    ///
    /// Value must not be currently mutably borrowed.
    /// Failing this produces undefined behavior on Release.
    #[inline(always)]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn clone(&self) -> Self {
        Self::new(self.borrow_mut().clone())
    }

    /// Returns a copy of the value.
    ///
    /// # Panics (Debug)
    ///
    /// Panics with the specified message if the value is currently mutably borrowed.
    ///
    /// # Safety
    ///
    /// Value must not be currently mutably borrowed.
    ///
    /// Failing this produces undefined behavior on Release.
    #[inline(always)]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn clone_expect(&self, msg: &str) -> Self {
        Self::new(self.borrow_mut_expect(msg).clone())
    }

    /// Returns a copy of the value.
    ///
    /// Since this method borrows `DcRefCell` mutably, it is statically guaranteed that no borrows to the underlying data exist.
    /// The dynamic checks (at Debug) inherent in `clone` and most other methods of `DcRefCell` are therefore unnecessary.
    #[inline(always)]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn clone_mut(&mut self) -> Self {
        Self::new(self.get_mut().clone())
    }
}

impl<T: ?Sized + PartialEq> DcRefCell<T> {
    /// This method tests for `self` and other `values` to be equal.
    ///
    /// # Panics (Debug)
    ///
    /// Panics if the value in either `DcRefCell` is currently mutably borrowed.
    ///
    /// # Safety
    ///
    /// Value in either `DcRefCell` must not be currently mutably borrowed.
    ///
    /// Failing this produces undefined behavior on Release.
    #[inline(always)]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn eq(&self, other: &Self) -> bool {
        *self.borrow() == *other.borrow()
    }

    /// This method tests for `self` and other `values` to be equal.
    ///
    /// # Panics (Debug)
    ///
    /// Panics with the specified message if the value in either `DcRefCell` is currently mutably borrowed.
    ///
    /// # Safety
    ///
    /// Value in either `DcRefCell` must not be currently mutably borrowed.
    ///
    /// Failing this produces undefined behavior on Release.
    #[inline(always)]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn eq_expect(&self, other: &Self, msg: &str) -> bool {
        *self.borrow_expect(msg) == *other.borrow_expect(msg)
    }
}

impl<T: ?Sized + PartialOrd> DcRefCell<T> {
    /// This method returns an ordering between `self` and `other` values if one exists.
    ///
    /// # Panics (Debug)
    ///
    /// Panics if the value in either `DcRefCell` is currently borrowed.
    ///
    /// # Safety
    ///
    /// Value in either `DcRefCell` must not be currently mutably borrowed.
    ///
    /// Failing this produces undefined behavior on Release.
    #[inline(always)]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.borrow().partial_cmp(&*other.borrow())
    }

    /// This method returns an ordering between `self` and `other` values if one exists.
    ///
    /// # Panics (Debug)
    ///
    /// Panics with the specified message if the value in either `DcRefCell` is currently mutably borrowed.
    ///
    /// # Safety
    ///
    /// Value in either `DcRefCell` must not be currently mutably borrowed.
    ///
    /// Failing this produces undefined behavior on Release.
    #[inline(always)]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn partial_cmp_expect(&self, other: &Self, msg: &str) -> Option<Ordering> {
        self.borrow_expect(msg).partial_cmp(&*other.borrow_expect(msg))
    }

    /// This method tests less than (for `self` and `other`).
    ///
    /// # Panics (Debug)
    ///
    /// Panics if the value in either `DcRefCell` is currently borrowed.
    ///
    /// # Safety
    ///
    /// Value in either `DcRefCell` must not be currently mutably borrowed.
    ///
    /// Failing this produces undefined behavior on Release.
    #[inline(always)]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn lt(&self, other: &Self) -> bool {
        *self.borrow() < *other.borrow()
    }

    /// This method tests less than (for `self` and `other`).
    ///
    /// # Panics (Debug)
    ///
    /// Panics with the specified message if the value in either `DcRefCell` is currently borrowed.
    ///
    /// # Safety
    ///
    /// Value in either `DcRefCell` must not be currently mutably borrowed.
    ///
    /// Failing this produces undefined behavior on Release.
    #[inline(always)]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn lt_expect(&self, other: &Self, msg: &str) -> bool {
        *self.borrow_expect(msg) < *other.borrow_expect(msg)
    }

    /// This method tests less than or equal to (for `self` and `other`).
    ///
    /// # Panics (Debug)
    ///
    /// Panics if the value in either `DcRefCell` is currently borrowed.
    ///
    /// # Safety
    ///
    /// Value in either `DcRefCell` must not be currently mutably borrowed.
    ///
    /// Failing this produces undefined behavior on Release.
    #[inline(always)]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn le(&self, other: &Self) -> bool {
        *self.borrow() < *other.borrow()
    }

    /// This method tests less than or equal to (for `self` and `other`).
    ///
    /// # Panics (Debug)
    ///
    /// Panics with the specified message if the value in either `DcRefCell` is currently borrowed.
    ///
    /// # Safety
    ///
    /// Value in either `DcRefCell` must not be currently mutably borrowed.
    ///
    /// Failing this produces undefined behavior on Release.
    #[inline(always)]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn le_expect(&self, other: &Self, msg: &str) -> bool {
        *self.borrow_expect(msg) < *other.borrow_expect(msg)
    }

    /// This method tests greater than (for `self` and `other`).
    ///
    /// # Panics (Debug)
    ///
    /// Panics if the value in either `DcRefCell` is currently borrowed.
    ///
    /// # Safety
    ///
    /// Value in either `DcRefCell` must not be currently mutably borrowed.
    ///
    /// Failing this produces undefined behavior on Release.
    #[inline(always)]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn gt(&self, other: &Self) -> bool {
        *self.borrow() < *other.borrow()
    }

    /// This method tests greater than (for `self` and `other`).
    ///
    /// # Panics (Debug)
    ///
    /// Panics with the specified message if the value in either `DcRefCell` is currently borrowed.
    ///
    /// # Safety
    ///
    /// Value in either `DcRefCell` must not be currently mutably borrowed.
    ///
    /// Failing this produces undefined behavior on Release.
    #[inline(always)]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn gt_expect(&self, other: &Self, msg: &str) -> bool {
        *self.borrow_expect(msg) < *other.borrow_expect(msg)
    }

    /// This method tests greater than or equal to (for `self` and `other`).
    ///
    /// # Panics (Debug)
    ///
    /// Panics if the value in either `DcRefCell` is currently borrowed.
    ///
    /// # Safety
    ///
    /// Value in either `DcRefCell` must not be currently mutably borrowed.
    ///
    /// Failing this produces undefined behavior on Release.
    #[inline(always)]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn ge(&self, other: &Self) -> bool {
        *self.borrow() < *other.borrow()
    }

    /// This method tests greater than or equal to (for `self` and `other`).
    ///
    /// # Panics (Debug)
    ///
    /// Panics with the specified message if the value in either `DcRefCell` is currently borrowed.
    ///
    /// # Safety
    ///
    /// Value in either `DcRefCell` must not be currently mutably borrowed.
    ///
    /// Failing this produces undefined behavior on Release.
    #[inline(always)]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn ge_expect(&self, other: &Self, msg: &str) -> bool {
        *self.borrow_expect(msg) < *other.borrow_expect(msg)
    }
}

impl<T: ?Sized + Ord> DcRefCell<T> {
    /// This method returns an `Ordering` between `self` and `other`.
    ///
    /// By convention, `self.cmp(&other)` returns the ordering matching the expression `self <operator> other` if true.
    ///
    /// # Panics (Debug)
    ///
    /// Panics if the value in either `DcRefCell` is currently borrowed.
    ///
    /// # Safety
    ///
    /// Value in either `DcRefCell` must not be currently mutably borrowed.
    ///
    /// Failing this produces undefined behavior on Release.
    #[inline(always)]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn cmp(&self, other: &Self) -> Ordering {
        self.borrow().cmp(&*other.borrow())
    }

    /// This method returns an ordering between `self` and `other` values if one exists.
    ///
    /// # Panics (Debug)
    ///
    /// Panics with the specified message if the value in either `DcRefCell` is currently mutably borrowed.
    ///
    /// # Safety
    ///
    /// Value in either `DcRefCell` must not be currently mutably borrowed.
    ///
    /// Failing this produces undefined behavior on Release.
    #[inline(always)]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn cmp_expect(&self, other: &Self, msg: &str) -> Ordering {
        self.borrow_expect(msg).cmp(&*other.borrow_expect(msg))
    }
}

impl<T: ?Sized> DcRefCell<T> {
    /// Returns a mutable reference to the underlying data.
    ///
    /// Since this method borrows `DcRefCell` mutably, it is statically guaranteed that no borrows to the underlying data exist.
    /// The dynamic checks (at Debug) inherent in `borrow_mut` and most other methods of `DcRefCell` are therefore unnecessary.
    ///
    /// This method can only be called if `DcRefCell` can be mutably borrowed,
    /// which in general is only the case directly after the `DcRefCell` has been created.
    /// In these situations, skipping the aforementioned dynamic borrowing checks may yield better ergonomics and runtime-performance.
    ///
    /// In most situations where `DcRefCell` is used, it can’t be borrowed mutably.
    /// Use `borrow_mut` to get mutable access to the underlying data then.
    #[inline(always)]
    pub fn get_mut(&mut self) -> &mut T {
        self.0.get_mut()
    }
}

impl<T> DcRefCell<T> {
    /// Consumes the `DcRefCell`, returning the wrapped value.
    #[inline(always)]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn into_inner(self) -> T {
        self.0.into_inner()
    }

    /// Replaces the wrapped value with a new one, returning the old value, without deinitializing either one.
    ///
    /// This function corresponds to `mem::replace`.
    ///
    /// # Panics (Debug)
    ///
    /// Panics if the value is currently borrowed.
    ///
    /// # Safety
    ///
    /// Value must not be currently mutably borrowed.
    ///
    /// Failing this produces undefined behavior on Release.
    #[inline(always)]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn replace(&self, t: T) -> T {
        mem::replace(&mut *self.borrow_mut(), t)
    }

    /// Replaces the wrapped value with a new one, returning the old value, without deinitializing either one.
    ///
    /// This function corresponds to `mem::replace`.
    ///
    /// # Panics (Debug)
    ///
    /// Panics with the specified message if the value is currently borrowed.
    ///
    /// # Safety
    ///
    /// Value must not be currently mutably borrowed.
    ///
    /// Failing this produces undefined behavior on Release.
    #[inline(always)]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn replace_expect(&self, t: T, msg: &str) -> T {
        mem::replace(&mut *self.borrow_mut_expect(msg), t)
    }

    /// Replaces the wrapped value with a new one computed from `f`, returning the old value, without deinitializing either one.
    ///
    /// # Panics (Debug)
    ///
    /// Panics if the value is currently borrowed.
    ///
    /// # Safety
    ///
    /// Value must not be currently mutably borrowed.
    ///
    /// Failing this produces undefined behavior on Release.
    #[inline(always)]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn replace_with<F: FnOnce(&mut T) -> T>(&self, f: F) -> T {
        let mut_borrow = &mut *self.borrow_mut();
        let replacement = f(mut_borrow);
        mem::replace(mut_borrow, replacement)
    }

    /// Replaces the wrapped value with a new one computed from `f`, returning the old value, without deinitializing either one.
    ///
    /// # Panics (Debug)
    ///
    /// Panics with the specified message if the value is currently borrowed.
    ///
    /// # Safety
    ///
    /// Value must not be currently mutably borrowed.
    ///
    /// Failing this produces undefined behavior on Release.
    #[inline(always)]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn replace_with_expect<F: FnOnce(&mut T) -> T>(&self, f: F, msg: &str) -> T {
        let mut_borrow = &mut *self.borrow_mut_expect(msg);
        let replacement = f(mut_borrow);
        mem::replace(mut_borrow, replacement)
    }

    /// Swaps the wrapped value of self with the wrapped value of other, without deinitializing either one.
    ///
    /// This function corresponds to `mem::swap`.
    ///
    /// # Panics (Debug)
    ///
    /// Panics if the value in either `DcRefCell` is currently borrowed or point to the same `DcRefCell`.
    ///
    /// # Safety
    ///
    /// Value in either `DcRefCell` must not be currently mutably borrowed.
    ///
    /// Failing this produces undefined behavior on Release.
    #[inline(always)]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn swap(&self, other: &Self) {
        mem::swap(&mut *self.borrow_mut(), &mut *other.borrow_mut());
    }

    /// Swaps the wrapped value of self with the wrapped value of other, without deinitializing either one.
    ///
    /// This function corresponds to `mem::swap`.
    ///
    /// # Panics (Debug)
    ///
    /// Panics if the value in either `DcRefCell` is currently borrowed or point to the same `DcRefCell`.
    ///
    /// # Safety
    ///
    /// Value in either `DcRefCell` must not be currently mutably borrowed.
    ///
    /// Failing this produces undefined behavior on Release.
    #[inline(always)]
    #[cfg_attr(debug_assertions, track_caller)]
    pub unsafe fn swap_expect(&self, other: &Self, msg: &str) {
        mem::swap(&mut *self.borrow_mut_expect(msg), &mut *other.borrow_mut_expect(msg));
    }
}

#[cfg(debug_assertions)]
impl<T: ?Sized> DcRefCell<T> {
    /// Returns a raw pointer to the underlying data in this cell.
    #[inline(always)]
    pub fn as_ptr(&self) -> *mut T {
        self.0.as_ptr()
    }

    /// Immutably borrows the wrapped value.
    ///
    /// The borrow lasts until the returned `Ref` exits scope.
    /// Multiple immutable borrows can be taken out at the same time.
    ///
    /// # Panics (Debug)
    ///
    /// Panics if the value is currently mutably borrowed.
    ///
    /// # Safety
    ///
    /// Value must not be currently mutably borrowed.
    ///
    /// Failing this produces undefined behavior on Release.
    #[inline(always)]
    #[track_caller]
    pub unsafe fn borrow(&self) -> Ref<'_, T> {
        Ref(self.0.try_borrow().unwrap())
    }

    /// Immutably borrows the wrapped value.
    ///
    /// The borrow lasts until the returned `Ref` exits scope.
    /// Multiple immutable borrows can be taken out at the same time.
    ///
    /// # Panics (Debug)
    ///
    /// Panics with the specified message if the value is currently mutably borrowed.
    ///
    /// # Safety
    ///
    /// Value must not be currently mutably borrowed.
    ///
    /// Failing this produces undefined behavior on Release.
    #[inline(always)]
    #[track_caller]
    pub unsafe fn borrow_expect(&self, msg: &str) -> Ref<'_, T> {
        Ref(self.0.try_borrow().expect(msg))
    }

    /// Mutability borrows the wrapped value.
    ///
    /// The borrow lasts until the returned `RefMut` or all `RefMuts` derived from it exit scope.
    /// The value cannot be borrowed while this borrow is active.
    ///
    /// # Panics (Debug)
    ///
    /// Panics if the value is currently mutably borrowed.
    ///
    /// # Safety
    ///
    /// Value must not be currently mutably borrowed.
    ///
    /// Failing this produces undefined behavior on Release.
    #[inline(always)]
    #[track_caller]
    pub unsafe fn borrow_mut(&self) -> RefMut<'_, T> {
        RefMut(self.0.try_borrow_mut().unwrap())
    }

    /// Mutability borrows the wrapped value.
    ///
    /// The borrow lasts until the returned `RefMut` or all `RefMuts` derived from it exit scope.
    /// The value cannot be borrowed while this borrow is active.
    ///
    /// # Panics (Debug)
    ///
    /// Panics with the specified message if the value is currently mutably borrowed.
    ///
    /// # Safety
    ///
    /// Value must not be currently mutably borrowed.
    ///
    /// Failing this produces undefined behavior on Release.
    #[inline(always)]
    #[track_caller]
    pub unsafe fn borrow_mut_expect(&self, msg: &str) -> RefMut<'_, T> {
        RefMut(self.0.try_borrow_mut().expect(msg))
    }
}


#[cfg(debug_assertions)]
impl<T> DcRefCell<T> {
    /// Creates a new `DcRefCell` containing `value.`
    #[inline(always)]
    pub fn new(value: T) -> Self {
        Self(RefCell::new(value))
    }
}


#[cfg(not(debug_assertions))]
impl<T: ?Sized> DcRefCell<T> {
    /// Returns a raw pointer to the underlying data in this cell.
    #[inline(always)]
    pub fn as_ptr(&self) -> *mut T {
        self.0.get()
    }

    /// Immutably borrows the wrapped value.
    ///
    /// The borrow lasts until the returned `Ref` exits scope.
    /// Multiple immutable borrows can be taken out at the same time.
    ///
    /// # Panics (Debug)
    ///
    /// Panics if the value is currently mutably borrowed.
    ///
    /// # Safety
    ///
    /// Value must not be currently mutably borrowed.
    ///
    /// Failing this produces undefined behavior on Release.
    #[inline(always)]
    #[track_caller]
    pub unsafe fn borrow<'a>(&'a self) -> Ref<'a, T> {
        Ref(&*self.0.get())
    }

    /// Immutably borrows the wrapped value.
    ///
    /// The borrow lasts until the returned `Ref` exits scope.
    /// Multiple immutable borrows can be taken out at the same time.
    ///
    /// # Panics (Debug)
    ///
    /// Panics with the specified message if the value is currently mutably borrowed.
    ///
    /// # Safety
    ///
    /// Value must not be currently mutably borrowed.
    ///
    /// Failing this produces undefined behavior on Release.
    #[inline(always)]
    #[track_caller]
    pub unsafe fn borrow_expect(&self, msg: &str) -> Ref<'_, T> {
        Ref(&*self.0.get())
    }

    /// Mutability borrows the wrapped value.
    ///
    /// The borrow lasts until the returned `RefMut` or all `RefMuts` derived from it exit scope.
    /// The value cannot be borrowed while this borrow is active.
    ///
    /// # Panics (Debug)
    ///
    /// Panics if the value is currently mutably borrowed.
    ///
    /// # Safety
    ///
    /// Value must not be currently mutably borrowed.
    ///
    /// Failing this produces undefined behavior on Release.
    #[inline(always)]
    #[track_caller]
    pub unsafe fn borrow_mut(&self) -> RefMut<'_, T> {
        RefMut(NonNull::new_unchecked(self.0.get()), PhantomData)
    }

    /// Mutability borrows the wrapped value.
    ///
    /// The borrow lasts until the returned `RefMut` or all `RefMuts` derived from it exit scope.
    /// The value cannot be borrowed while this borrow is active.
    ///
    /// # Panics (Debug)
    ///
    /// Panics with the specified message if the value is currently mutably borrowed.
    ///
    /// # Safety
    ///
    /// Value must not be currently mutably borrowed.
    ///
    /// Failing this produces undefined behavior on Release.
    #[inline(always)]
    #[track_caller]
    pub unsafe fn borrow_mut_expect(&self, msg: &str) -> RefMut<'_, T> {
        RefMut(NonNull::new_unchecked(self.0.get()), PhantomData)
    }
}


#[cfg(not(debug_assertions))]
impl<T> DcRefCell<T> {
    /// Creates a new `DcRefCell` containing `value.`
    #[inline(always)]
    pub fn new(value: T) -> Self {
        Self(UnsafeCell::new(value))
    }
}