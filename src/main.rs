extern crate websocket;

mod server;
mod message;

fn main() {
    server::serve()
}
