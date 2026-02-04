use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() {
    
    let mut child = match Command::new("ping")
        .arg("google.com")
        .spawn() 
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: Could not spawn 'ping google.com'. Ensure 'ping' is available and you have network connectivity. {}", e);
            return;
        }
    };
    
    println!("Child Process ('ping google.com') spawned successfully.");
    println!("PID of child process: {}", child.id());

    thread::sleep(Duration::from_secs(5));
    
    child.kill().unwrap();
    println!("Child process (PID {}) successfully terminated by parent.", child.id());

    match child.wait() {
        Ok(status) => {
            println!("\nCleanup complete.");
            if let Some(code) = status.code() {
                println!("Child process exited with status code: {}", code);
            } else {
                #[cfg(unix)]
                {
                    use std::os::unix::process::ExitStatusExt;
                    if let Some(signal) = status.signal() {
                        println!("Child process terminated by signal: {}", signal);
                    }
                }
                #[cfg(not(unix))]
                {
                    println!("Child process terminated alternatively.");
                }
            }
        },
        Err(e) => {
            eprintln!("Error waiting for child process: {}", e);
        }
    }

}