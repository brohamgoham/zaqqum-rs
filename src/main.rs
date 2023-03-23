use std::net::TcpListener;
use tungstenite::server::accept;


fn main() {
    let server = TcpListener::bind("127.0.0.1").unwrap();
    for stream in server.incoming() {
        let stream = stream.unwrap();
        accept(stream).unwrap().0.for_each(|msg| {
            let msg = msg.unwrap();
            println!("Receieved msg")
        });
    }
}
