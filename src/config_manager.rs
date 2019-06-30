
use std::net::SocketAddr;

use std::collections::{HashMap, HashSet};

pub struct ConfigManager {
    id_to_socket_addr_map: HashMap<i32, SocketAddr>,
    socket_addr_to_id_map: HashMap<SocketAddr, i32>
}

impl ConfigManager {
    pub fn new() -> ConfigManager {
        ConfigManager {
            id_to_socket_addr_map: HashMap::new(),
            socket_addr_to_id_map: HashMap::new()
        }
    }

    pub fn insert_id_socket_addr_pair(&mut self, id: i32, socket_addr: SocketAddr) {
        self.id_to_socket_addr_map.insert(id, socket_addr);
        self.socket_addr_to_id_map.insert(socket_addr, id);
    }

    pub fn id_to_socket_addr(&self, id: i32) -> &SocketAddr {
        self.id_to_socket_addr_map.get(&id).unwrap()
    }

    pub fn socket_addr_to_id(&self, socket_addr: &SocketAddr) -> i32 {
        *self.socket_addr_to_id_map.get(socket_addr).unwrap()
    }

}