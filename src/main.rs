use ferris_says::say;
use std::io;
fn main() {
    println!("What should Ferris say: ");
    let stdout = io::stdout();
    let mut message = String::new();
    
    io::stdin()
    .read_line(&mut message)
    .expect("Failed to read line");
    
    let width = message.chars().count();
    let mut writer = io::BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}