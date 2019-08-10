
use std::collections::HashSet;

use commons::types::NodeId;


pub struct ConfigurationManager {
    node_id: NodeId,
    node_ids: HashSet<NodeId>
}

impl ConfigurationManager {
    pub fn new(node_id: NodeId, node_ids: HashSet<NodeId>) -> ConfigurationManager {
        ConfigurationManager {
            node_id: node_id,
            node_ids: node_ids
        }
    }

    pub fn node_id(&self) -> NodeId {
        self.node_id
    }

    pub fn node_ids(&self) -> &HashSet<NodeId> {
        &self.node_ids
    }
}