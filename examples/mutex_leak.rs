use rtic_rw::*;

#[derive(Debug, Copy, Clone)]
struct NonAtomicU32 {
    x: u32,
    y: u32,
    z: u32,
}

fn main() {
    let mut mutex = TestMutex {
        data: NonAtomicU32 { x: 0, y: 0, z: 0 },
    };

    let d = mutex.lock(|data| {
        data.x += 1;
        println!("Read lock: x={}, y={}, z={}", data.x, data.y, data.z);
        data
    });

    println!("Read lock: x={}, y={}, z={}", d.x, d.y, d.z);
}
