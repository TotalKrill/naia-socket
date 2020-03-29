
mod socket;

use crate::server::socket::{ServerSocket, ServerSocketImpl};
use std::net::SocketAddr;
use crate::shared::GaiaMessage;
const SERVER_ADDR: &str = "127.0.0.1:12351";

pub struct Server {
    //socket: ServerSocketImpl
}

impl Server {
    pub fn new() -> Server { //args should take a shared config, and a port

        println!("Server New!");

        let mut server_socket = ServerSocketImpl::new();

        server_socket.on_connection(|sender| {
            println!("Server on_connection()");

            let msg: String = "hello new client!".to_string();
            sender.send(msg.as_str());
        });

        server_socket.on_receive(|sender, msg| {
            println!("Server on_receive(): {:?}", msg);

            let response_msg = "echo from server: ".to_owned() + msg;
            sender.send(response_msg.as_str());
        });

        server_socket.listen(SERVER_ADDR);

        Server {
            //socket: server_socket
        }
    }

    pub fn update(&mut self) {

    }

    pub fn connect(&self, listen_addr: SocketAddr) { //put a port in here..

    }

    pub fn on_connect(&self, func: fn()) { //function should have client, clientData, and callback?

    }

    pub fn on_disconnect(&self, func: fn()) { //function should have client

    }

    pub fn add_object(&self) {

    }

    pub fn remove_object(&self) {

    }

    pub fn send_message(&self) {

    }

    pub fn receive_message(&self) {
    }
}
