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

    mutex_rw.read_lock(|data| {
        println!("Read lock: x={}, y={}, z={}", data.x, data.y, data.z);
        mutex_rw.write_lock(|data_inner| {
            data_inner.x += data.x;
            data_inner.y += data.x;
            println!(
                "Nested Read lock: x={}, y={}, z={}",
                data_inner.x, data_inner.y, data_inner.z
            );
        });
    });
}
