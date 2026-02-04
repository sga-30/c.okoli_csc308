use std::process::Command;
use std::fs::File;
use std::io::Write;

fn main() {
    let output = Command::new("echo")
        .arg("Rust Process Management")
        .output();

    let command_output = match output {
        Ok(out) => out,
        Err(e) => {
            eprintln!("Error executing command: {}", e);
            return; 
        }
    };

    if !command_output.status.success() {
        eprintln!("Command failed with status: {}", command_output.status);
        eprintln!("Stderr: {}", String::from_utf8_lossy(&command_output.stderr));
        return;
    }

    let stdout_bytes = &command_output.stdout;
    let stdout_string = String::from_utf8_lossy(stdout_bytes);
    println!("   -> Captured STDOUT: \"{}\"", stdout_string.trim());

    let filename = "output.txt";
    println!("Writing captured output to file: {}", filename);
    
    match File::create(filename) {
        Ok(mut file) => {
            match file.write_all(stdout_bytes) {
                Ok(_) => println!("   -> Successfully wrote to {}.", filename),
                Err(e) => eprintln!("Error writing to file: {}", e),
            }
        },
        Err(e) => eprintln!("Error creating file {}: {}", filename, e),
    }
}