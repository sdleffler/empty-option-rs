//! This crate provides convenient wrappers for dealing with `&mut Option<T>`. There are two main types, `OptionGuard` and `OptionGuardMut`:
//! 
//! ## `OptionGuard`
//! 
//! Using `EmptyOptionExt::steal` on an `&mut Option<T>` produces the `T` from the option as well as an `OptionGuard`. If `OptionGuard::restore` is not called before the `OptionGuard` is dropped, then a panic will occur.
//! 
//! ### Examples
//! 
//! Calling `guard.restore()` puts the stolen value back into the original option:
//! 
//! ```rust
//! use empty_option::EmptyOptionExt;
//! 
//! // A mutable option, from which we shall steal a value!
//! let mut thing = Some(5);
//! 
//! // Scope so that when we do `guard.restore()`, the mutable borrow on `thing` will end.
//! {
//!     // Steal the value - we now have the guard and also a concrete `T` from our `Option<T>`.
//!     let (guard, five) = thing.steal();
//! 
//!     assert_eq!(five, 5);
//! 
//!     // Move the value back into `thing` - we're done.
//!     guard.restore(6);
//! }
//! 
//! // The value is returned by `guard.restore()`.
//! assert_eq!(thing, Some(6));
//! ```
//! 
//! But, if the guard is dropped instead, a runtime panic results.
//! 
//! ```rust,should_panic
//! use empty_option::EmptyOptionExt;
//! 
//! let mut thing = Some(5);
//! 
//! let (_, _) = thing.steal();
//! 
//! // Never return the value!
//! ```
//! 
//! ## `OptionGuardMut`
//! 
//! Using `EmptyOptionExt::steal_mut` on an `&mut Option<T>` produces an `OptionGuardMut`, which dereferences to a `T`. To get the inner value out, `OptionGuardMut::into_inner` can be called. On `Drop`, if the `OptionGuardMut` is not consumed with `OptionGuardMut::into_inner`, the value in the `OptionGuardMut` will be returned to the `Option` that it was borrowed from.
//! 
//! ### Examples
//! 
//! Take a value from an option, which is automatically returned:
//! 
//! ```rust
//! use empty_option::EmptyOptionExt;
//! 
//! let mut thing = Some(5);
//! 
//! {
//!     let mut stolen = thing.steal_mut();
//! 
//!     assert_eq!(*stolen, 5);
//! 
//!     *stolen = 6;
//! }
//! 
//! assert_eq!(thing, Some(6));
//! ```
//! 
//! If the guard is consumed, the value is never returned.
//! 
//! ```rust
//! use empty_option::EmptyOptionExt;
//! 
//! let mut thing = Some(5);
//! 
//! {
//!     // Keep the thing!
//!     let stolen = thing.steal_mut().into_inner();
//! 
//!     assert_eq!(stolen, 5);
//! }
//! 
//! assert_eq!(thing, None);
//! ```

use std::mem;
use std::ops::{Deref, DerefMut};


/// Extension trait providing nice method sugar for `steal` and `steal_mut`.
pub trait EmptyOptionExt {
    type Inner;

    /// Take a value out of an option, providing a guard which panics if the value is not returned.
    fn steal(&mut self) -> (OptionGuard<Self::Inner>, Self::Inner);

    /// Take a value out of an option, providing a guard which returns the value unless consumed by
    /// `OptionGuardMut::into_inner`.
    fn steal_mut<'a>(&'a mut self) -> OptionGuardMut<'a, Self::Inner>;
}


/// An option which has had its value taken. On `Drop`, `OptionGuard` will panic - in order to
/// prevent a panic, the stolen value must be moved back in with `OptionGuard::restore`.
///
/// This is useful if you are using an `Option` because you have a value which you need to take,
/// and then deal with by-value, but you want to preserve the invariant that your optional value is
/// always present.
/// 
/// # Examples
///
/// Calling `guard.restore()` puts the stolen value back into the original option:
///
/// ```
/// # use empty_option::EmptyOptionExt;
/// // A mutable option, from which we shall steal a value!
/// let mut thing = Some(5);
/// 
/// // Scope so that when we do `guard.restore()`, the mutable borrow on `thing` will end.
/// {
///     // Steal the value - we now have the guard and also a concrete `T` from our `Option<T>`.
///     let (guard, five) = thing.steal();
/// 
///     assert_eq!(five, 5);
/// 
///     // Move the value back into `thing` - we're done.
///     guard.restore(6);
/// }
/// 
/// // The value is returned by `guard.restore()`.
/// assert_eq!(thing, Some(6));
/// ```
///
/// But, if the guard is dropped instead, a runtime panic results.
///
/// ```should_panic
/// # use empty_option::EmptyOptionExt;
/// let mut thing = Some(5);
/// 
/// let (_, _) = thing.steal();
/// 
/// // Never return the value!
/// ```
pub struct OptionGuard<'a, T: 'a> {
    opt: &'a mut Option<T>,
}


impl<'a, T> Drop for OptionGuard<'a, T> {
    fn drop(&mut self) {
        panic!("`Some` value was never restored to a victimized Option!");
    }
}


impl<'a, T> OptionGuard<'a, T> {
    fn new(opt: &'a mut Option<T>) -> OptionGuard<'a, T> {
        OptionGuard {
            opt
        }
    }


    /// Restore a stolen value to an `Option`.
    pub fn restore(self, obj: T) {
        *self.opt = Some(obj);

        mem::forget(self);
    }
}


/// A value taken from an `Option<T>`. `OptionGuardMut<T>` dereferences to a `T`, and the inner `T`
/// can be moved out with `OptionGuardMut::into_inner`. When dropped, the `OptionGuardMut` moves
/// the taken value back into the `Option` it came from.
///
/// # Examples
///
/// Take a value from an option, which is automatically returned:
///
/// ```
/// # use empty_option::EmptyOptionExt;
/// let mut thing = Some(5);
/// 
/// {
///     let mut stolen = thing.steal_mut();
/// 
///     assert_eq!(*stolen, 5);
/// 
///     *stolen = 6;
/// }
/// 
/// assert_eq!(thing, Some(6));
/// ```
/// 
/// If the guard is consumed, the value is never returned.
///
/// ```
/// # use empty_option::EmptyOptionExt;
/// let mut thing = Some(5);
/// 
/// {
///     // Keep the thing!
///     let stolen = thing.steal_mut().into_inner();
/// 
///     assert_eq!(stolen, 5);
/// }
/// 
/// assert_eq!(thing, None);
/// ```
pub struct OptionGuardMut<'a, T: 'a> {
    origin: &'a mut Option<T>,
    value: Option<T>,
}


impl<'a, T> Drop for OptionGuardMut<'a, T> {
    fn drop(&mut self) {
        *self.origin = self.value.take();
    }
}


impl<'a, T> OptionGuardMut<'a, T> {
    /// Keep the value stolen from the `Option` and do not return it.
    pub fn into_inner(mut self) -> T {
        self.value.take().unwrap()
    }
}


impl<'a, T> Deref for OptionGuardMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.value.as_ref().unwrap()
    }
}


impl<'a, T> DerefMut for OptionGuardMut<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.value.as_mut().unwrap()
    }
}


impl<T> EmptyOptionExt for Option<T> {
    type Inner = T;

    fn steal(&mut self) -> (OptionGuard<T>, T) {
        let value = self.take().expect("attempted to steal from None");
        (OptionGuard::new(self), value)
    }

    fn steal_mut(&mut self) -> OptionGuardMut<T> {
        let value = self.take();

        OptionGuardMut {
            origin: self,
            value,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catch_and_release() {
        let mut thing = Some(5);

        {
            let (guard, five) = thing.steal();

            assert_eq!(five, 5);

            guard.restore(6);
        }

        assert_eq!(thing, Some(6));
    }

    #[test]
    #[should_panic]
    fn catch_and_keep() {
        let mut thing = Some(5);

        let (_, _) = thing.steal();

        // Never return the value!
    }

    #[test]
    fn mut_and_release() {
        let mut thing = Some(5);

        {
            let mut stolen = thing.steal_mut();

            assert_eq!(*stolen, 5);

            *stolen = 6;
        }

        assert_eq!(thing, Some(6));
    }

    #[test]
    fn mut_and_keep() {
        let mut thing = Some(5);
        
        {
            // Keep the thing!
            let stolen = thing.steal_mut().into_inner();

            assert_eq!(stolen, 5);
        }

        assert_eq!(thing, None);
    }
}
