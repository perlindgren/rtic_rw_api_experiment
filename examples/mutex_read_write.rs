use rtic_rw::*;

#[derive(Debug, Copy, Clone)]
struct NonAtomicU32 {
    x: u32,
    y: u32,
    z: u32,
}

fn main() {
    let mut mutex_rw = TestMutexRW {
        data: NonAtomicU32 { x: 0, y: 0, z: 0 },
    };

    let (x, y) = mutex_rw.read_lock(|data| {
        println!("Read lock: x={}, y={}, z={}", data.x, data.y, data.z);
        let y = mutex_rw.write_lock(|data| {
            data.x += 1;
            println!("Nested Read lock: x={}, y={}, z={}", data.x, data.y, data.z);
            data.y
        });
        (data.x, y)
    });
    println!("Nested Read lock: x={}, y={}", x, y);
}
