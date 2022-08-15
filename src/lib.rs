//! Adds the trait `AsRefMut<T>`, which is implemented for any type
//! which implements `AsRef<T>` and `AsMut<T>`.

/// Marker trait representing a type which implements `AsRef<T>` and `AsMut<T>`.
pub trait AsRefMut<T: ?Sized>: AsRef<T> + AsMut<T> {}

impl<Target: ?Sized + AsRef<T> + AsMut<T>, T: ?Sized> AsRefMut<T> for Target {}
