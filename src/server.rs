use std::thread;

use websocket::{sync::Server, OwnedMessage};

pub fn serve() {
    let server = Server::bind("127.0.0.1:8080").unwrap();

    for request in server.filter_map(Result::ok) {
        thread::spawn(|| {
            // If protocol isn't specificed, close the connection
            if !request.protocols().contains(&"rust-websocket".to_string()) {
                request.reject().unwrap();
                return;
            }

            // Create the WebSocket client to send & recieve data
            let mut client = request
                .use_protocol(&"rust-websocket".to_string())
                .accept()
                .unwrap();

            // IP address (to print ONLY)
            let ip_addr = client.peer_addr().unwrap();
            println!("Conn from {}", ip_addr);

            // Create the "hello" message, and send it to client
            let hello_message = OwnedMessage::Text("Hello".to_string());
            client.send_message(&hello_message).unwrap();

            // Get the sender and reciever for the connection
            let (mut reciever, mut sender) = client.split().unwrap();

            // For each message that is recieved, send it back
            for recv_message in reciever.incoming_messages() {
                // Get the recieved message
                let message = recv_message.unwrap();

                match message {
                    // close on "close" signal
                    OwnedMessage::Close(_) => {
                        let close_message = OwnedMessage::Close(None);
                        sender.send_message(&close_message).unwrap();
                        // print client disconnected
                        println!("client disconnected");
                        return;
                    }
                    // 'pong' back on ping signal
                    OwnedMessage::Ping(ping) => {
                        let pong_message = OwnedMessage::Pong(ping);
                        sender.send_message(&pong_message).unwrap();
                    }
                    // for verything else, echo back
                    _ => sender.send_message(&message).unwrap(),
                }
            }
        });
    }
}
