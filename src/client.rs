use super::common::{Message};

use message_io::network::{NetEvent, Transport, Endpoint};
use message_io::node::{self, NodeHandler, NodeListener};

use std::net::{SocketAddr};
use std::collections::{HashMap};
use std::io::{self};

pub struct Client {
    handler: NodeHandler<()>,
    name: String,
    server_endpoint: Endpoint
}

impl Client {
    fn new(name: &str) -> io::Result<Client> {
        let (handler, node_listener) = node::split();
        let server_endpoint = "127.0.0.1:5000"; // Connection to the discovery server.
        let (endpoint, _) = handler.network().connect(Transport::FramedTcp, server_endpoint)?;
        Ok(Client {
            handler,
            name,
            endpoint
        })
    }
}