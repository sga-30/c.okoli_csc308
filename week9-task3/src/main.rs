use std::fs::{self, OpenOptions};
use std::io::{self, Write, BufReader, BufRead};
use chrono::{DateTime, Local};

const FILE_PATH: &str = "notes.txt";

fn main() {
    loop {
        match get_menu_choice() {
            1 => add_note().expect("Failed to add note."),
            2 => display_notes().expect("Failed to display notes."),
            3 => {
                println!("Exiting program.");
                break;
            }
            _ => println!("Invalid choice. Please enter 1, 2, or 3."),
        }
    }
}

fn get_menu_choice() -> u32 {
    println!("1. Add a new note");
    println!("2. Display all notes");
    println!("3. Exit");
    print!("Enter your choice: ");

    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    choice.trim().parse().unwrap_or(0)
}

fn add_note() -> io::Result<()> {
    print!("Enter your note: ");
    io::stdout().flush()?;

    let mut note_content = String::new();
    io::stdin().read_line(&mut note_content)?;

    let now: DateTime<Local> = Local::now();

    let formatted_note = format!(
        "[{}] {}\n",
        now.format("%Y-%m-%d %H:%M:%S"),
        note_content.trim()
    );

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(FILE_PATH)?;

    file.write_all(formatted_note.as_bytes())?;

    println!("Note saved successfully to {}.", FILE_PATH);
    Ok(())
}

fn display_notes() -> io::Result<()> {
    match fs::File::open(FILE_PATH) {
        Ok(file) => {
            println!("\n--- Your Notes ---");

            let reader = BufReader::new(file);
            let mut count = 0;

            for line in reader.lines() {
                println!("{}", line?);
                count += 1;
            }

            if count == 0 {
                println!("(No notes found in the file.)");
            }

            Ok(())
        }
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => {
            println!("The notes file '{}' does not exist yet.", FILE_PATH);
            Ok(())
        }
        Err(e) => Err(e),
    }
}
