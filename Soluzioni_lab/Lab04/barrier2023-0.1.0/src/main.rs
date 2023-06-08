use std::{sync::Arc};
use barrier2023::{ChannelBarrier, SupervisorBarrier, CyclicBarrier};


fn test_barrier() {
    let abarrrier = Arc::new(CyclicBarrier::new(3));

    let mut vt = Vec::new();

    for i in 0..3 {
        let cbarrier = abarrrier.clone();

        vt.push(std::thread::spawn(move || {
            for j in 0..10 {
                cbarrier.wait();
                println!("barrier open {} {}", i, j);
            }
        }));
    }

    for t in vt {
        t.join().unwrap();
    }
}


fn test_channel_barrier() {
    let barrier = Arc::new(ChannelBarrier::new(10));

    let mut threads = Vec::new();
    for i in 0..10 {
        let barrier = barrier.clone();
        threads.push(std::thread::spawn(move || {
            for k in 0.. 10 {
                //println!("thread {} before barrier {} ", i, k);
                barrier.wait(i);
                println!("thread {} after barrier {}", i, k);
            }
        }));
    }

    
}

fn test_supervisor_barrier() {

    // barrier with generic handling of values
    let mut barriers = SupervisorBarrier::new(10, |x: Vec<i32>| {
        x.iter().sum()
    });

    let mut threads = Vec::new();
    for i in 0..10 {
        let barrier = barriers.pop().unwrap();
        threads.push(std::thread::spawn(move || {
            for k in 0.. 10 {
                let res = barrier.wait(k);
                println!("thread:{} sum:{} after k:{}", i, res, k);
            }
        }));
    }

    for t in threads {
        t.join().unwrap();
    }
}
fn main() {
    //test_channel_barrier();
    //test_supervisor_barrier();
    test_barrier();
}
