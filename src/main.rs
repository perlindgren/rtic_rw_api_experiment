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
    fn read_lock<R>(&self, f: impl FnOnce(&Self::T) -> R) -> R;

    fn write_lock<R>(&mut self, f: impl FnOnce(&mut Self::T) -> R) -> R;
}

struct TestMutex<T> {
    data: T,
}

impl<T> Mutex for TestMutex<T> {
    type T = T;

    fn read_lock<R>(&self, f: impl FnOnce(&Self::T) -> R) -> R {
        f(&self.data)
    }

    fn write_lock<R>(&mut self, f: impl FnOnce(&mut Self::T) -> R) -> R {
        f(&mut self.data)
    }
}

fn main() {
    let mut mutex = TestMutex { data: 42 };

    mutex.read_lock(|data| {
        println!("Read lock: {}", data);
    });

    mutex.write_lock(|data| {
        *data += 1;
        println!("Write lock: {}", data);
    });

    mutex.read_lock(|data| {
        println!("Read lock: {}", data);
        mutex.read_lock(|data| {
            println!("Nested Read lock: {}", data);
        });
    });

    mutex.read_lock(|data| {
        println!("Read lock: {}", data);
        mutex.write_lock(|data| {
            println!("Nested Write lock: {}", data);
        });
    });

    mutex.write_lock(|data| {
        println!("Write lock: {}", data);
        mutex.write_lock(|data| {
            println!("Nested Write lock: {}", data);
        });
    });
}
