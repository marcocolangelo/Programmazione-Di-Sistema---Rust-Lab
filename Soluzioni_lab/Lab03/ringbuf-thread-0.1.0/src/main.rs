use ringbuf_thread::RingBuf;
use std::{
    str::SplitTerminator,
    sync::{Arc},
    thread, time::Duration,
};


fn main() {
    let rb = Arc::new(RingBuf::<i32>::new(100));
    let rb1 = rb.clone();
    let rb2 = rb.clone();
    let rb3 = rb.clone();

    // es 2 producer 1 consumer, scrivi e leggi 200 valori
    let t1 = thread::spawn(move || {
        let mut count = 0;
        while count < 100 {
            if let Ok(_) =  rb1.write(count){
                count += 1;
            } else {
                thread::sleep(Duration::from_millis(1));
            };
        }
    });
    let t2 = thread::spawn(move || {
        let mut count = 0;
        while count < 100 {
            if let Ok(_) =  rb2.write(count){
                count += 1;
            } else {
                thread::sleep(Duration::from_millis(1));
            };
        }
    });

    // es 1 consumer
    let t3 = thread::spawn(move || {
        let mut sum = 0;
        let mut count = 0;
        while count < 200 {
            if let Some(val) = rb3.read() {
                sum += val;
                count += 1;
            } else {
                thread::sleep(Duration::from_millis(1));
                
            }
        } 
    
        println!("sum = {} count = {}", sum, count);
    });

    t3.join().unwrap();

}
