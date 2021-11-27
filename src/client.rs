use super::common::{Message};

use message_io::network::{NetEvent, Transport, Endpoint};
use message_io::node::{self, NodeHandler, NodeListener};

use std::io::{self};

#[derive(Clone)]
pub struct Client {
    handler: NodeHandler<()>,
    name: String,
    server_endpoint: Endpoint
}

impl Client {
    pub fn new(name: &str) -> io::Result<Client> {
        let (handler, node_listener) = node::split();
        let server_endpoint = "127.0.0.1:5000"; // Connection to the discovery server.
        let (endpoint, address) = handler.network().connect(Transport::FramedTcp, server_endpoint)?;
        let message = Message::RegisterClient(name.to_string().clone(), address);
        let output_data = bincode::serialize(&message).unwrap();
        handler.network().send(endpoint, &output_data);
        Ok(Client {
            handler,
            name: name.to_string(),
            server_endpoint: endpoint,
        })
    }

    pub fn send_data(self, message: &str) {
        let message = Message::DataTransferMessage(self.name.clone(), message.to_string());
        let output_data = bincode::serialize(&message).unwrap();
        self.handler.network().send(self.server_endpoint, &output_data);
    }
}