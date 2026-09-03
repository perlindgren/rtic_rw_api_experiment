pub mod generated;

/// Memory safe access to shared resources
///
/// In RTIC, locks are implemented as critical sections that prevent other tasks from *starting*.
/// These critical sections are implemented by temporarily increasing the dynamic priority of the
/// current context. Entering and leaving these critical sections is always done in bounded constant
/// time (a few instructions in bare metal contexts).
pub trait Mutex {
    /// Data protected by the mutex
    type T;

    /// Creates a critical section and grants temporary access to the protected data

    fn lock<R>(&mut self, f: impl FnOnce(&mut Self::T) -> R) -> R;
}

pub trait MutexRW {
    /// Data protected by the mutex
    type T;

    /// Creates a critical section and grants temporary access to the protected data
    fn read_lock<R>(&self, f: impl FnOnce(&Self::T) -> R) -> R;

    fn write_lock<R>(&mut self, f: impl FnOnce(&mut Self::T) -> R) -> R;
}

pub trait MutexRW2 {
    /// Data protected by the mutex
    type T;

    /// Creates a critical section and grants temporary access to the protected data
    fn read_lock<R>(&self, f: impl FnOnce(&Self::T) -> R) -> R;

    fn write_lock<R>(&mut self, f: impl FnOnce(&mut Self::T) -> R) -> R;

    fn demote_write_lock<R>(_self: Self::T, f: impl FnOnce(&Self::T) -> R) -> R;

    fn promote_read_lock<R>(_self: Self::T, f: impl FnOnce(&mut Self::T) -> R) -> R;
}

pub use generated::*;
