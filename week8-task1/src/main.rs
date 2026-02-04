use std::process::Command;

fn main() {
    let mut child = Command::new("sleep")
        .arg("5")
        .spawn()
        .expect("Failed to spawn");

    let status = child.wait().unwrap();
    println!("Exited with: {}", status);

    println!("Child PID: {}", child.id());

    let output = Command::new("ls")
        .arg("-la")
        .output()
        .expect("Failed to execute command");
    println!("{}", String::from_utf8_lossy(&output.stdout));

    let result = Command::new("echo").arg("Hello from child").output().unwrap();
    println!("{}", String::from_utf8_lossy(&result.stdout));
}