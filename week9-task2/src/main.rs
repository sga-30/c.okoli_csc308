use std::thread;
use std::sync::{Arc, Mutex}; 

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let num_threads = 10;
    let mut handles = vec![];

    for i in 0..num_threads {
        let counter_clone = Arc::clone(&counter); 
        let thread_name = format!("Thread-{}", i + 1);

        let handle = thread::spawn(move || { 
            println!("{}: Starting work...", thread_name);

            let mut num = counter_clone.lock().unwrap();

            *num += 3; 

            println!("{}: Added 3. Counter is now at {}", thread_name, *num);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_result = *counter.lock().unwrap();
    
    println!("Final Result: {}", final_result);

}