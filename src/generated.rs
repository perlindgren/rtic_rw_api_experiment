use crate::*;

pub struct TestMutex<T> {
    pub data: T,
}

impl<T> Mutex for TestMutex<T> {
    type T = T;

    fn lock<R>(&mut self, f: impl FnOnce(&mut Self::T) -> R) -> R {
        // In a real implementation, this would ensure exclusive access to the data
        // For SRP raise system ceiling
        f(&mut self.data)
        // For SRP restore system ceiling
    }
}

pub struct TestMutexRW<T> {
    pub data: T,
}

impl<T> MutexRW for TestMutexRW<T> {
    type T = T;

    fn read_lock<R>(&self, f: impl FnOnce(&Self::T) -> R) -> R {
        f(&self.data)
    }

    fn write_lock<R>(&mut self, f: impl FnOnce(&mut Self::T) -> R) -> R {
        f(&mut self.data)
    }
}

pub struct TestMutexRW2<T> {
    pub data: T,
}

impl<T> MutexRW2 for TestMutexRW2<T> {
    type T = T;
    //
    fn read_lock<R>(&self, f: impl FnOnce(&Self::T) -> R) -> R {
        f(&self.data)
    }
    //
    fn write_lock<R>(&mut self, f: impl FnOnce(&mut Self::T) -> R) -> R {
        f(&mut self.data)
    }

    // This would be possible
    fn demote_write_lock<R>(_self: Self::T, f: impl FnOnce(&Self::T) -> R) -> R {
        f(&_self)
    }
    //
    // Not possible, there is no way to consume a an immutable reference in Rust
    // Moreover, it is not possible to ensure that outer scopes do not hold an immutable reference
    fn promote_read_lock<R>(&self) -> &mut Self {
        todo!();
        // &mut *(self as *const Self as *mut Self)
    }
}
