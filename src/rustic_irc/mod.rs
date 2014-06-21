use std::io::IoResult;
use std::io::net::tcp::TcpStream;
use std::io::net::addrinfo;
use std::str;

pub struct IrcConn {
    nick: String,
    channel: String,
    stream: TcpStream,
    buffer: [u8, ..4096],
}

impl Clone for IrcConn {
    fn clone(&self) -> IrcConn {
        IrcConn {
            nick: self.nick.clone(),
            channel: self.channel.clone(),
            stream: self.stream.clone(),
            
            // We don't need to clone the buffer, we will just use an empty one
            buffer: [0 as u8, ..4096],
        }
    }
}

impl IrcConn {
    // Returns an IrcConn wrapped in an IoResult.
    pub fn new(hostname: &str, channel: String, nick: String) -> IoResult<IrcConn> {
        let ips = try!(addrinfo::get_host_addresses(hostname));
        let ip = ips.get(0);
        let stream = try!(TcpStream::connect(ip.to_str().as_slice(), 6667));
        
        let mut irc_conn = IrcConn { nick: nick, channel: channel, stream: stream, buffer: [0, ..4096] };
        
        // Set nick and user
        let nick = irc_conn.nick.clone(); // To satisfy the compiler
        try!(irc_conn.send(format!("NICK {}\r\n", nick).as_slice()));
        try!(irc_conn.send(format!("USER {0} {0} {0} :{1}\r\n", nick, hostname).as_slice()));
        
        // Discard default welcome message
        try!(irc_conn.receive_message());
        
        // Receive and respond to the first PING
        try!(irc_conn.receive_message());
        
        // Join the given channel
        let chan = irc_conn.channel.clone(); // To satisfy the compiler
        try!(irc_conn.join_channel(chan));
        
        Ok(irc_conn)
    }
    
    // Join a channel on the current server
    pub fn join_channel(&mut self, channel: String) -> IoResult<()> {
        self.channel = channel;
        let chan = self.channel.clone(); // To satisfy the compiler
        self.send(format!("JOIN {}\r\n", chan).as_slice())
    }
    
    // Returns a Result containing the string sent by the server
    // Will also respond any ping message automatically
    pub fn receive_message(&mut self) -> IoResult<String> {
        // Receive the message
        let amount = try!(self.stream.read(self.buffer.as_mut_slice()));
        let message = str::from_utf8_lossy(self.buffer.mut_slice(0, amount)).to_string();
        
        println!("{} bytes received!", amount);
        
        // Detect and respond to possible PINGs
        try!(self.respond_pings(message.as_slice()));
        
        Ok(message)
    }
    
    // Send a message to the current channel
    pub fn send_message(&mut self, msg: &str) -> IoResult<()> {
        let chan = self.channel.clone(); // To satisfy the compiler
        self.send(format!("PRIVMSG {} :{}\r\n", chan, msg).as_slice())
    }
    
    // Send a command to the server, without any special formatting
    fn send(&mut self, raw_msg: &str) -> IoResult<()> {
        self.stream.write(raw_msg.as_bytes())
    }
    
    // Detect incoming pings and respond them
    fn respond_pings(&mut self, message: &str) -> IoResult<()> {
        for line in message.split('\n') {
            let mut words = line.words();
            match (words.next(), words.next()) {
                (Some(w1), Some(pong_key)) if w1 == "PING" => {
                    println!("Ping received, pong answered");
                    return self.send(format!("PONG {}\r\n", pong_key).as_slice());
                }
                _ => { return Ok(()); }
            }
        }
        
        Ok(())
    }
}