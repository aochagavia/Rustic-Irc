use std::io;
use std::io::BufferedReader;
use rustic_irc::IrcConn;

mod rustic_irc;

fn main() {
    let irc = IrcConn::new("irc.something.org", "#channel".to_owned(), "Nobody".to_owned());
    let mut irc = irc.ok().expect("Unexpected error");
    
    // We clone two times... It is the only way to satisfy the compiler
    // The underlying stream remains the same
    let mut cloned_irc = irc.clone();
    spawn(proc() handle_input(&mut cloned_irc.clone()));
    
    // Send what you write as a message
    let mut reader = BufferedReader::new(io::stdin());
    for line in reader.lines().map(|l| l.unwrap()) {
        irc.send_message(line);
    }
}

fn handle_input(irc: &mut IrcConn) {
    // Print all what is said
    loop {
        println!("Received message: {}", irc.receive_message().ok().unwrap());
    }
}