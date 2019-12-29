use std::sync::{Mutex, Arc};
use std::thread::JoinHandle;

fn main() {
    let counter = Arc::new(Mutex::new(0u64));

    let mut handles : Vec<JoinHandle<()>> = Vec::new();

    for _ in 0..10 {
        let counter = counter.clone();
        let handle = std::thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }


    for handle in handles {
        handle.join().unwrap();
    }

    println!("count is {}", *counter.lock().unwrap());
}
