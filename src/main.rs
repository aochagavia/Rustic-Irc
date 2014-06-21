use std::io;
use std::io::BufferedReader;
use rustic_irc::IrcConn;

mod rustic_irc;

fn main() {
    let irc = IrcConn::new("irc.something.org", "#channel".to_string(), "Nobody".to_string());
    let mut irc = irc.ok().expect("Unexpected error");
    
    // The underlying stream remains the same
    let cloned_irc = irc.clone();
    spawn(proc() {
        // We need first to capture the object. Then we can use it.
        let mut captured_irc = cloned_irc;
        handle_input(&mut captured_irc)
    });
    
    // Send what you write as a message
    let mut reader = BufferedReader::new(io::stdin());
    for line in reader.lines().map(|l| l.unwrap()) {
        irc.send_message(line.as_slice()).ok().expect("IO error");
    }
}

fn handle_input(irc: &mut IrcConn) {
    // Print all what is said
    loop {
        println!("Received message: {}", irc.receive_message().ok().unwrap());
    }
}