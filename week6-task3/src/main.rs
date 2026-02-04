use std::process::Command; 

fn main() {
    let output = Command::new("echo")
        .arg("Hello from child process!")
        .output()
        .expect("failed to execute process"); 

    println!("Status: {}", output.status);

    println!("Output: {}", String::from_utf8_lossy(&output.stdout));
}