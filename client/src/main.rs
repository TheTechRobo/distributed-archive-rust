use victorem;
fn main() {
    let mut client = victorem::ClientSocket::new(11111, "127.0.0.1:22222").expect("Couldn't connect to server - try again later.");
    loop {
        let _ = client.send(format!("GET").into_bytes());
        let _ = client
            .recv()
            .map(|v| String::from_utf8(v).unwrap_or(String::new()))
            .map(|s| println!("From Server: {}", s));
    }
}

