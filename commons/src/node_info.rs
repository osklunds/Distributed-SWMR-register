
use std::net::SocketAddr;

pub type NodeId = i32;


#[derive(Debug, PartialEq, Eq, Hash)]
pub struct NodeInfo {
    pub node_id: NodeId,
    pub socket_addr: SocketAddr,
    pub key_path: String,
    pub username: String
}

impl NodeInfo {
    pub fn ip_addr_string(&self) -> String {
        format!("{}", self.socket_addr.ip())
    }
}