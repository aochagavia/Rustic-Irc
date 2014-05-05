use rustic_irc::IrcConn;

mod rustic_irc;

fn main() {
    let irc = IrcConn::new("irc.address.com", "#channel-name".to_owned(), "Nickname".to_owned());
    let mut irc = irc.ok().expect("Unexpected error");
    
    // Discard the welcome message of the server
    irc.receive_message().ok();
    
    // Say hello world
    irc.send_message("Hello world");
    
    // Show me all what is said
    loop {
        println!("Received message: {}", irc.receive_message().ok().unwrap());
    }
}