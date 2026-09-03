use rtic_rw::*;

#[derive(Debug, Copy, Clone)]
struct NonAtomicU32 {
    x: u32,
    y: u32,
    z: u32,
}

fn main() {
    // let mut mutex_rw = TestMutexRW {
    //     data: NonAtomicU32 { x: 0, y: 0, z: 0 },
    // };

    // let x = mutex_rw.write_lock(|data| {
    //     data.x += 1;
    //     println!("Write lock: x={}, y={}, z={}", data.x, data.y, data.z);
    //     let data = &*data;
    //     println!("Read lock: x={}, y={}, z={}", data.x, data.y, data.z);
    //     // data += 1; // This line is invalid because `data` is an immutable reference
    //     data.x
    // });
    // println!("Read lock: x={}", x);

    // let mut mutex_rw2 = TestMutexRW2 {
    //     data: NonAtomicU32 { x: 0, y: 0, z: 0 },
    // };
    // let x = mutex_rw2.write_lock(|data| {
    //     data.x += 1;
    //     println!("Write lock: x={}, y={}, z={}", data.x, data.y, data.z);
    //     TestMutexRW2::demote_write_lock(data, |data_inner| {
    //         println!(
    //             "Read lock: x={}, y={}, z={}",
    //             data_inner.x, data_inner.y, data_inner.z
    //         );

    //         let p = &*data_inner;
    //         //*data_inner
    //     });
    // });

    let mut mutex_rw2 = TestMutexRW2 {
        data: NonAtomicU32 { x: 0, y: 0, z: 0 },
    };
    let x = mutex_rw2.read_lock(|data| {
        // data.x += 1; // This line is invalid because `data` is an immutable reference
        println!("Read lock: x={}, y={}, z={}", data.x, data.y, data.z);

        TestMutexRW2::promote_read_lock(data, |data_inner| {
            println!(
                "Read lock: x={}, y={}, z={}",
                data_inner.x, data_inner.y, data_inner.z
            );

            println!("promote: {:?}", data);
            let p = &mut *data_inner;
        });
    });
}
