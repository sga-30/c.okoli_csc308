use std::thread;
use std::sync::mpsc; 
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let num_threads = 3;
    let messages_per_thread = 5;

    let mut handles = vec![];

    for i in 1..=num_threads {
        let thread_name = format!("T{}", i);
        
        let thread_tx = tx.clone();

        let handle = thread::spawn(move || { 
            for j in 1..=messages_per_thread {
                let msg = format!("{}: Message {}", thread_name, j);
                
                thread_tx.send(msg).unwrap();

                thread::sleep(Duration::from_millis(10));
            }
        });
        handles.push(handle);
    }
    
    drop(tx); 

    for received in rx {
        println!("Main thread prints: {}", received);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    
}